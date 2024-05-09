use crate::cbor;
use meowpad::{KeyCode, error::Error, KbReport};
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use palette::rgb::channels::Argb;
use palette::Srgb;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use serde_with::*;

#[derive(
    Serialize_repr, Deserialize_repr, FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, PartialEq,
)]
#[repr(u8)]
pub enum LightingMode {
    Off,
    Calibration,
    Error,

    Solid,
    RainbowMode,
    RainbowFlowMode,
    PressRadianceMode,

    BreatheGlowMode,
    BreatheGlowAsyncMode,

    RainDropMode,
    TapToGlowMode,
    SpeedLightMode
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Copy)]
pub struct KeyConfig {
    pub press_percentage: u8,
    pub release_percentage: u8,
    pub dead_zone: u8,   // 0-30
    pub key_data: [KeyCode; 6],
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Key {
    pub keys: [KeyConfig; 4],
    pub jitters_elimination_time: u16,
    pub continuous_report: bool,
    pub kalman_filter: bool,
    pub enable_hs: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Light {
    /// LED灯颜色
    pub led_colors: [Srgb<u8>; 4],
    /// 灯效模式
    pub lighting_mode: LightingMode,
    pub lighting_mode_sleep: LightingMode,
    pub max_brightness: u8,
    pub sleep_time: u16,

    // rainbow_flow_mode
    pub rainbow_flow_speed: u16,
    pub color_change_rate: u8,
    pub is_flow_delay: bool,

    // rainbow_mode
    pub rainbow_speed: u16,

    // breathing_mode
    pub breathing_speed: u16,
    pub max_keep_time: u16,
    pub min_keep_time: u16,
    pub breaths_before_color_switch: u8,

    // rain_drop_mode
    pub rain_drop_speed: u16,
    pub random_rain_chance: u16,

    // tap_to_glow_mode
    pub tap_to_glow_speed: u16,
    pub max_lum_freeze_time: u8,
    pub change_color_when_pressed: bool,
    pub random_color_mode: bool,

    
    // speed_light_mode
    pub speed_light_mode_speed: u16,
    pub attenuation_speed: u16,
    pub increase_difficulty: u8,
    pub low_speed_color: Srgb<u8>,
    pub high_speed_color: Srgb<u8>,
}

impl TryFrom<cbor::Keyboard> for Key {
    type Error = Error;
    fn try_from(cfg: cbor::Keyboard) -> Result<Self, Self::Error> {
        let mut keys: [KeyConfig; 4] = Default::default();
        for i in 0..4 {
            keys[i] = KeyConfig::from(cfg.KeyConfigs[i]);
        }

        Ok(Key {
            keys,
            continuous_report: cfg.ContinuousReport,
            kalman_filter: cfg.KalmanFilter,
            jitters_elimination_time: cfg.JittersEliminationTime,
            enable_hs: cfg.EnableHS
        })
    }
}

impl TryFrom<cbor::Light> for Light {
    type Error = Error;
    fn try_from(cfg: cbor::Light) -> Result<Self, Self::Error> {
        let mut led_colors: [Srgb<u8>; 4] = Default::default();
        for i in 0..4 {
            led_colors[i] = Srgb::from_u32::<Argb>(cfg.led_colors[i]);
        }

        Ok(Light {
            led_colors,
            lighting_mode: LightingMode::from_u8(cfg.led_mode).ok_or(
                Error::ConfigDataCheckFailed("lighting_mode", cfg.led_mode as usize),
            )?,
            lighting_mode_sleep: LightingMode::from_u8(cfg.led_mode_sleep).ok_or(
                Error::ConfigDataCheckFailed("lighting_mode_sleep", cfg.led_mode_sleep as usize),
            )?,
            max_brightness: cfg.max_brightness,
            sleep_time: cfg.sleep_time,
            rainbow_flow_speed: cfg.rainbow_flow_speed,
            is_flow_delay: cfg.is_flow_delay,
            color_change_rate: cfg.color_change_rate,
            rainbow_speed: cfg.rainbow_speed,
            breathing_speed: cfg.breathing_speed,
            max_keep_time: cfg.max_keep_time,
            min_keep_time: cfg.min_keep_time,
            breaths_before_color_switch: cfg.breaths_before_color_switch,
            rain_drop_speed: cfg.rain_drop_speed,
            random_rain_chance: cfg.random_rain_chance,
            tap_to_glow_speed: cfg.tap_to_glow_speed,
            max_lum_freeze_time: cfg.max_lum_freeze_time,
            change_color_when_pressed: cfg.change_color_when_pressed,
            random_color_mode: cfg.random_color_mode,
            speed_light_mode_speed: cfg.speed_light_mode_speed,
            attenuation_speed: cfg.attenuation_speed,
            increase_difficulty: cfg.increase_difficulty,
            low_speed_color: Srgb::from_u32::<Argb>(cfg.low_speed_color),
            high_speed_color: Srgb::from_u32::<Argb>(cfg.high_speed_color),
        })
    }
}

impl From<cbor::KeyRTConfig> for KeyConfig {
    fn from(cfg: cbor::KeyRTConfig) -> Self {
        Self {
            press_percentage: cfg.PressPercentage,
            release_percentage: cfg.ReleasePercentage,
            dead_zone: cfg.DeadZone,
            key_data: KbReport::from(cfg.KeyData).into(),
        }
    }
}

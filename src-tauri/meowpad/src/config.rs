use crate::{cbor::{CborConfig, KeyRTConfig}, error::Error, KbReport, KeyCode};
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use palette::rgb::channels::Argb;
use palette::Srgb;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use serde_with::*;
use std::time::Duration;

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

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, Default, Copy)]
pub struct KeyConfig {
    pub press_percentage: u8,
    pub release_percentage: u8,
    pub dead_zone: u8,   // 0-30
    pub key_data: [KeyCode; 6],
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub keys: [KeyConfig; 4],
    /// LED灯颜色
    pub led_colors: [Srgb<u8>; 4],
    /// 灯效模式
    pub lighting_mode_key: LightingMode,
    pub max_brightness: u8,
    pub jitters_elimination_time: u16,
    pub continuous_report: bool,
    pub kalman_filter: bool,
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

impl TryFrom<CborConfig> for Config {
    type Error = Error;
    fn try_from(cfg: CborConfig) -> Result<Self, Self::Error> {
        let mut keys: [KeyConfig; 4] = Default::default();
        for i in 0..4 {
            keys[i] = KeyConfig::from(cfg.Key.KeyConfigs[i]);
        }

        let mut led_colors: [Srgb<u8>; 4] = Default::default();
        for i in 0..4 {
            led_colors[i] = Srgb::from_u32::<Argb>(cfg.Light.led_colors[i]);
        }

        Ok(Config {
            keys,
            led_colors,
            lighting_mode_key: LightingMode::from_u8(cfg.Light.led_mode).ok_or(
                Error::ConfigDataCheckFailed("lighting_mode_key", cfg.Light.led_mode as usize),
            )?,
            max_brightness: cfg.Light.max_brightness,
            continuous_report: cfg.Key.ContinuousReport,
            kalman_filter: cfg.Key.KalmanFilter,
            jitters_elimination_time: cfg.Key.JittersEliminationTime,
            sleep_time: cfg.Light.sleep_time,
            rainbow_flow_speed: cfg.Light.rainbow_flow_speed,
            is_flow_delay: cfg.Light.is_flow_delay,
            color_change_rate: cfg.Light.color_change_rate,
            rainbow_speed: cfg.Light.rainbow_speed,
            breathing_speed: cfg.Light.breathing_speed,
            max_keep_time: cfg.Light.max_keep_time,
            min_keep_time: cfg.Light.min_keep_time,
            breaths_before_color_switch: cfg.Light.breaths_before_color_switch,
            rain_drop_speed: cfg.Light.rain_drop_speed,
            random_rain_chance: cfg.Light.random_rain_chance,
            tap_to_glow_speed: cfg.Light.tap_to_glow_speed,
            max_lum_freeze_time: cfg.Light.max_lum_freeze_time,
            change_color_when_pressed: cfg.Light.change_color_when_pressed,
            random_color_mode: cfg.Light.random_color_mode,
            speed_light_mode_speed: cfg.Light.speed_light_mode_speed,
            attenuation_speed: cfg.Light.attenuation_speed,
            increase_difficulty: cfg.Light.increase_difficulty,
            low_speed_color: Srgb::from_u32::<Argb>(cfg.Light.low_speed_color),
            high_speed_color: Srgb::from_u32::<Argb>(cfg.Light.high_speed_color),
        })
    }
}

impl From<KeyRTConfig> for KeyConfig {
    fn from(cfg: KeyRTConfig) -> Self {
        Self {
            press_percentage: cfg.PressPercentage,
            release_percentage: cfg.ReleasePercentage,
            dead_zone: cfg.DeadZone,
            key_data: KbReport::from(cfg.KeyData).into(),
        }
    }
}

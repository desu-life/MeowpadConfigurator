use crate::{
    config::{self, LightingMode},
    KbReport, KeyCode,
};
use palette::{rgb::channels::Argb, WithAlpha};
use palette::Srgb;
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::io::Cursor;

#[inline]
pub unsafe fn serialize_raw<T: Sized>(src: &T) -> &[u8] {
    ::std::slice::from_raw_parts((src as *const T) as *const u8, ::std::mem::size_of::<T>())
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct KeyRTConfig {
    #[serde(rename = "pp")]
    pub PressPercentage: u8,
    #[serde(rename = "rp")]
    pub ReleasePercentage: u8,
    #[serde(rename = "dz")]
    pub DeadZone: u8,
    #[serde(rename = "kd")]
    pub KeyData: [u8; 6],
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Keyboard {
    #[serde(rename = "ks")]
    pub KeyConfigs: [KeyRTConfig; 4],
    #[serde(rename = "jet")]
    pub JittersEliminationTime: u16,
    #[serde(rename = "cr")]
    pub ContinuousReport: bool,
    #[serde(rename = "kf")]
    pub KalmanFilter: bool,
    #[serde(rename = "ehs")]
    pub EnableHS: bool,
}

/// speed 0-10
/// steep length 50 - 200
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Light {
    #[serde(rename = "c")]
    pub led_colors: [u32; 4],
    #[serde(rename = "km")]
    pub led_mode: u8,
    #[serde(rename = "kms")]
    pub led_mode_sleep: u8,
    #[serde(rename = "mb")]
    pub max_brightness: u8,
    #[serde(rename = "st")]
    pub sleep_time: u16,

    // rainbow_flow_mode
    #[serde(rename = "rfs")]
    pub rainbow_flow_speed: u16,
    #[serde(rename = "ccr")]
    pub color_change_rate: u8,
    #[serde(rename = "fd")]
    pub is_flow_delay: bool,

    // rainbow_mode
    #[serde(rename = "rs")]
    pub rainbow_speed: u16,

    // breathing_mode
    #[serde(rename = "bs")]
    pub breathing_speed: u16,
    #[serde(rename = "akt")]
    pub max_keep_time: u16,
    #[serde(rename = "ikt")]
    pub min_keep_time: u16,
    #[serde(rename = "bcs")]
    pub breaths_before_color_switch: u8,

    // rain_drop_mode
    #[serde(rename = "rds")]
    pub rain_drop_speed: u16,
    #[serde(rename = "rrc")]
    pub random_rain_chance: u16,

    // tap_to_glow_mode
    #[serde(rename = "tgs")]
    pub tap_to_glow_speed: u16,
    #[serde(rename = "lft")]
    pub max_lum_freeze_time: u8,
    #[serde(rename = "cwp")]
    pub change_color_when_pressed: bool,
    #[serde(rename = "rcm")]
    pub random_color_mode: bool,

    // speed_light_mode
    #[serde(rename = "slms")]
    pub speed_light_mode_speed: u16,
    #[serde(rename = "as")]
    pub attenuation_speed: u16,
    #[serde(rename = "id")]
    pub increase_difficulty: u8,
    #[serde(rename = "lsc")]
    pub low_speed_color: u32,
    #[serde(rename = "hsc")]
    pub high_speed_color: u32,
}

pub trait CborConvertor
where
    Self: Sized + Serialize,
    for<'de> Self: Deserialize<'de>,
{
    fn from_cbor<T: AsRef<[u8]>>(data: T) -> Result<Self, ciborium::de::Error<std::io::Error>> {
        Ok(ciborium::de::from_reader(Cursor::new(data))?)
    }

    fn to_cbor(self) -> Vec<u8> {
        let mut data = vec![];
        ciborium::ser::into_writer(&self, &mut data).unwrap();
        data
    }
}

impl CborConvertor for Keyboard {}
impl CborConvertor for Light {}

impl From<config::KeyConfig> for KeyRTConfig {
    fn from(cfg: config::KeyConfig) -> Self {
        Self {
            PressPercentage: cfg.press_percentage,
            ReleasePercentage: cfg.release_percentage,
            DeadZone: cfg.dead_zone,
            KeyData: cfg.key_data.into_iter().collect::<KbReport>().into(),
        }
    }
}

impl From<config::Key> for Keyboard {
    fn from(cfg: config::Key) -> Self {
        let mut key_configs = [KeyRTConfig::default(); 4];
        for i in 0..4 {
            key_configs[i] = cfg.keys[i].into();
        }

        Keyboard {
            KeyConfigs: key_configs,
            ContinuousReport: cfg.continuous_report,
            KalmanFilter: cfg.kalman_filter,
            JittersEliminationTime: cfg.jitters_elimination_time,
            EnableHS: cfg.enable_hs
        }
    }
}

impl From<config::Light> for Light {
    fn from(cfg: config::Light) -> Self {
        let mut led_colors = [0u32; 4];
        for i in 0..4 {
            led_colors[i] = cfg.led_colors[i].with_alpha(0).into_u32::<Argb>();
        }

        Light {
            led_colors: led_colors,
            led_mode: cfg.lighting_mode as u8,
            led_mode_sleep: cfg.lighting_mode_sleep as u8,
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
            low_speed_color: cfg.low_speed_color.with_alpha(0).into_u32::<Argb>(),
            high_speed_color: cfg.high_speed_color.with_alpha(0).into_u32::<Argb>(),
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        let led_colors = [
            Srgb::new(255, 255, 255).with_alpha(0).into_u32::<Argb>(),
            Srgb::new(255, 255, 255).with_alpha(0).into_u32::<Argb>(),
            Srgb::new(255, 255, 255).with_alpha(0).into_u32::<Argb>(),
            Srgb::new(255, 255, 255).with_alpha(0).into_u32::<Argb>(),
        ];
        Self {
            led_colors: led_colors,
            led_mode: LightingMode::BreatheGlowMode as u8,
            led_mode_sleep: LightingMode::Off as u8,
            max_brightness: 50,
            sleep_time: 120,
            rainbow_flow_speed: 100,
            is_flow_delay: true,
            color_change_rate: 1,
            rainbow_speed: 2,
            breathing_speed: 8,
            max_keep_time: 500,
            min_keep_time: 0,
            breaths_before_color_switch: 3,
            rain_drop_speed: 2,
            random_rain_chance: 400,
            tap_to_glow_speed: 10,
            max_lum_freeze_time: 50,
            change_color_when_pressed: true,
            random_color_mode: false,
            speed_light_mode_speed: 2,
            attenuation_speed: 80,
            increase_difficulty: 24,
            low_speed_color: Srgb::new(255, 255, 255).with_alpha(0).into_u32::<Argb>(),
            high_speed_color: Srgb::new(255, 0, 0).with_alpha(0).into_u32::<Argb>(),
        }
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        let key_configs = [
            KeyRTConfig::with_key(KeyCode::Z),
            KeyRTConfig::with_key(KeyCode::X),
            KeyRTConfig::with_key(KeyCode::C),
            KeyRTConfig::with_key(KeyCode::V),
            // KeyRTConfig::default(),
        ];
        Self {
            KeyConfigs: key_configs,
            ContinuousReport: false,
            KalmanFilter: true,
            JittersEliminationTime: 15 * 8,
            EnableHS: true
        }
    }
}

impl Default for KeyRTConfig {
    fn default() -> Self {
        Self {
            PressPercentage: 8,
            ReleasePercentage: 8,
            DeadZone: 15,
            KeyData: [0; 6],
        }
    }
}

impl KeyRTConfig {
    pub fn with_key(key: KeyCode) -> Self {
        Self {
            KeyData: key.to_report().into(),
            ..Default::default()
        }
    }
}

use crate::{cbor::{CONFIG, KeyRTConfig}, error::Error, KbReport, KeyCode};
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
    Off = 0,
    Debug = 1,
    Error = 2,

    Solid = 3,
    RainbowMode = 4,
    RainbowFlowMode = 5,
    PressRadianceMode = 6,
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
}

impl TryFrom<CONFIG> for Config {
    type Error = Error;
    fn try_from(cfg: CONFIG) -> Result<Self, Self::Error> {
        let mut keys: [KeyConfig; 4] = Default::default();
        for i in 0..4 {
            keys[i] = KeyConfig::from(cfg.Key.KeyConfigs[i]);
        }

        let mut led_colors: [Srgb<u8>; 4] = Default::default();
        for i in 0..4 {
            led_colors[i] = Srgb::from_u32::<Argb>(cfg.LED.LED_COLORS[i]);
        }

        Ok(Config {
            keys,
            led_colors,
            lighting_mode_key: LightingMode::from_u8(cfg.LED.LED_KEY_MODE).ok_or(
                Error::ConfigDataCheckFailed("lighting_mode_key", cfg.LED.LED_KEY_MODE as usize),
            )?,
            max_brightness: cfg.LED.LED_MAX_BRIGHTNESS,
            continuous_report: cfg.Key.ContinuousReport,
            kalman_filter: cfg.Key.KalmanFilter,
            jitters_elimination_time: cfg.Key.JittersEliminationTime,
            sleep_time: cfg.LED.LED_SLEEP_TIME
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

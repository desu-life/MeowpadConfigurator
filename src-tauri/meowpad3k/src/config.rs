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
    pub keys: [KeyConfig; 3],
    pub side_btn: [KeyCode; 6],
    pub jitters_elimination_time: u16,
    pub continuous_report: bool,
    pub kalman_filter: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Light {
    /// LED灯颜色
    pub led_colors: [Srgb<u8>; 3],
    /// 灯效模式
    pub lighting_mode: LightingMode,
    pub max_brightness: u8,
    pub sleep_time: u16,

}

impl TryFrom<cbor::Keyboard> for Key {
    type Error = Error;
    fn try_from(cfg: cbor::Keyboard) -> Result<Self, Self::Error> {
        let mut keys: [KeyConfig; 3] = Default::default();
        for i in 0..3 {
            keys[i] = KeyConfig::from(cfg.KeyConfigs[i]);
        }

        Ok(Key {
            keys,
            side_btn: KbReport::from(cfg.SideBtnKeyData).into(),
            continuous_report: cfg.ContinuousReport,
            kalman_filter: cfg.KalmanFilter,
            jitters_elimination_time: cfg.JittersEliminationTime,
        })
    }
}

impl TryFrom<cbor::Light> for Light {
    type Error = Error;
    fn try_from(cfg: cbor::Light) -> Result<Self, Self::Error> {
        let mut led_colors: [Srgb<u8>; 3] = Default::default();
        for i in 0..3 {
            led_colors[i] = Srgb::from_u32::<Argb>(cfg.led_colors[i]);
        }

        Ok(Light {
            led_colors,
            lighting_mode: LightingMode::from_u8(cfg.led_mode).ok_or(
                Error::ConfigDataCheckFailed("lighting_mode", cfg.led_mode as usize),
            )?,
            max_brightness: cfg.max_brightness,
            sleep_time: cfg.sleep_time,
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

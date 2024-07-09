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
    pub release_dead_zone: u8,   // 0-30
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Key {
    #[serde_as(as = "[_; 64]")]
    pub keys: [KeyConfig; 64],
    pub jitters_elimination_time: u16,
    pub continuous_report: bool,
    pub kalman_filter: bool,
    pub enable_hs: bool
}

impl TryFrom<cbor::Keyboard> for Key {
    type Error = Error;
    fn try_from(cfg: cbor::Keyboard) -> Result<Self, Self::Error> {
        let mut keys = [KeyConfig::default(); 64];
        for i in 0..64 {
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

impl From<cbor::KeyRTConfig> for KeyConfig {
    fn from(cfg: cbor::KeyRTConfig) -> Self {
        Self {
            press_percentage: cfg.PressPercentage,
            release_percentage: cfg.ReleasePercentage,
            dead_zone: cfg.DeadZone,
            release_dead_zone: cfg.ReleaseDeadZone,
        }
    }
}

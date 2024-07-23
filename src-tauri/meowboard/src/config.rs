use crate::{cbor, keymap};
use meowpad::{KeyCode, error::Error, KbReport};
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use palette::rgb::channels::Argb;
use palette::Srgb;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use serde_with::*;


#[derive(Serialize, Deserialize, Clone, Debug, Default, Copy)]
pub struct KeyConfig {
    pub press_percentage: u8,
    pub release_percentage: u8,
    pub dead_zone: u8,   // 0-30
    pub release_dead_zone: u8,   // 0-30
    pub rt_enabled: bool
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Device {
    #[serde_as(as = "[_; 64]")]
    pub keys: [KeyConfig; 64],
    #[serde_as(as = "[_; 64]")]
    pub normal_layer: [KeyCode; 64],
    #[serde_as(as = "[_; 64]")]
    pub fn_layer: [KeyCode; 64],
    pub jitters_elimination_time: u16,
    pub continuous_report: bool,
    pub hall_filter: u8,
    pub max_brightness: u8,
}

impl TryFrom<cbor::Device> for Device {
    type Error = Error;
    fn try_from(cfg: cbor::Device) -> Result<Self, Self::Error> {
        let mut keys = [KeyConfig::default(); 64];
        for i in 0..64 {
            keys[i] = KeyConfig::from(cfg.KeyConfigs[i]);
        }
        let map: [[KeyCode; 64]; 2] = keymap::KeyMap::from(cfg.KeyMap).into();

        Ok(Device {
            keys,
            normal_layer: map[0],
            fn_layer: map[1],
            continuous_report: cfg.ContinuousReport,
            hall_filter: cfg.HallFilter,
            jitters_elimination_time: cfg.JittersEliminationTime,
            max_brightness: cfg.MaxBrightness,
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
            rt_enabled: cfg.RtEnabled
        }
    }
}

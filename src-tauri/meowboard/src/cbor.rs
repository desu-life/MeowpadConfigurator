use crate::config::{self, LightingMode};
use meowpad::{KeyCode, KbReport};
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
    #[serde(rename = "rdz")]
    pub ReleaseDeadZone: u8,
    
}

#[repr(C)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Keyboard {
    #[serde(rename = "ks")]
    #[serde_as(as = "[_; 64]")]
    pub KeyConfigs: [KeyRTConfig; 64],
    #[serde(rename = "jet")]
    pub JittersEliminationTime: u16,
    #[serde(rename = "cr")]
    pub ContinuousReport: bool,
    #[serde(rename = "kf")]
    pub KalmanFilter: bool,
    #[serde(rename = "ehs")]
    pub EnableHS: bool,
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

impl From<config::KeyConfig> for KeyRTConfig {
    fn from(cfg: config::KeyConfig) -> Self {
        Self {
            PressPercentage: cfg.press_percentage,
            ReleasePercentage: cfg.release_percentage,
            DeadZone: cfg.dead_zone,
            ReleaseDeadZone: cfg.release_dead_zone,
        }
    }
}

impl From<config::Key> for Keyboard {
    fn from(cfg: config::Key) -> Self {
        let mut key_configs = [KeyRTConfig::default(); 64];
        for i in 0..64 {
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

impl Default for Keyboard {
    fn default() -> Self {
        let key_configs = [KeyRTConfig::default(); 64];
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
            ReleaseDeadZone: 5,
        }
    }
}


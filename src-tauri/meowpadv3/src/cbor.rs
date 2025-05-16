use crate::config;
use crate::keymap;
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
pub struct SOCDPairConfig {
    #[serde(rename = "1")]
    pub key1: u8,
    #[serde(rename = "2")]
    pub key2: u8,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct KeyRTConfig {
    #[serde(rename = "p")]
    pub PressPercentage: u8,
    #[serde(rename = "r")]
    pub ReleasePercentage: u8,
    #[serde(rename = "d")]
    pub DeadZone: u8,
    #[serde(rename = "rd")]
    pub ReleaseDeadZone: u8,
    #[serde(rename = "e")]
    pub RtEnabled: bool,
}

#[repr(C)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Device {
    #[serde(rename = "ks")]
    #[serde_as(as = "[_; 4]")]
    pub KeyConfigs: [KeyRTConfig; 4],
    #[serde(rename = "km")]
    #[serde_as(as = "[_; 42]")]
    pub KeyMap: [u8; 42],
    #[serde(rename = "jet")]
    pub JittersEliminationTime: u16,
    #[serde(rename = "hr")]
    pub HighReportRate: bool,
    #[serde(rename = "kp")]
    pub KeyProof: bool,
    #[serde(rename = "ac")]
    pub AutoCalibration: bool,
    #[serde(rename = "hf")]
    pub HallFilter: u8,
    #[serde(rename = "mb")]
    pub MaxBrightness: u8,
    #[serde(rename = "c")]
    pub led_color: u32,
    #[serde(rename = "spc")]
    pub socd_pair_count: u8,
    #[serde(rename = "so")]
    #[serde_as(as = "[_; 5]")]
    pub socd_pairs: [SOCDPairConfig; 5],
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

impl CborConvertor for Device {}

impl From<config::KeyConfig> for KeyRTConfig {
    fn from(cfg: config::KeyConfig) -> Self {
        Self {
            PressPercentage: cfg.press_percentage,
            ReleasePercentage: cfg.release_percentage,
            DeadZone: cfg.dead_zone,
            ReleaseDeadZone: cfg.release_dead_zone,
            RtEnabled: cfg.rt_enabled,
        }
    }
}

impl From<config::SOCDKeyPairs> for SOCDPairConfig {
    fn from(cfg: config::SOCDKeyPairs) -> Self {
        Self {
            key1: cfg.key1,
            key2: cfg.key2,
        }
    }
}

impl From<config::Device> for Device {
    fn from(cfg: config::Device) -> Self {
        let mut key_configs = [KeyRTConfig::default(); 4];
        for i in 0..4 {
            key_configs[i] = cfg.keys[i].into();
        }
        let map = keymap::KeyMap::from(cfg.layer).into();

        let mut socd_pairs = [SOCDPairConfig::default(); 5];
        let socd_pair_count = cfg.socd_key_pairs.len().min(5) as u8;
        for i in 0..socd_pair_count as usize {
            socd_pairs[i] = cfg.socd_key_pairs[i].into();
        }

        Device {
            KeyConfigs: key_configs,
            KeyMap: map,
            HighReportRate: cfg.high_reportrate,
            KeyProof: cfg.key_proof,
            AutoCalibration: cfg.auto_calibration,
            JittersEliminationTime: cfg.jitters_elimination_time,
            HallFilter: cfg.hall_filter,
            MaxBrightness: cfg.max_brightness,
            led_color: cfg.led_color.into_u32::<Argb>(),
            socd_pair_count,
            socd_pairs
        }
    }
}

impl Default for Device {
    fn default() -> Self {
        let key_configs = [KeyRTConfig::default(); 4];
        let key_maps = keymap::KeyMap::default();
        let mut socd_pairs = [SOCDPairConfig::default(); 5];
        // socd_pairs[0].key1 = 29;
        // socd_pairs[0].key2 = 31;
        Self {
            KeyConfigs: key_configs,
            KeyMap: key_maps.into(),
            HighReportRate: true,
            KeyProof: true,
            AutoCalibration: true,
            JittersEliminationTime: 15 * 8,
            HallFilter: 1,
            MaxBrightness: 50,
            led_color: Srgb::new(255, 255, 255).into_u32::<Argb>(),
            socd_pair_count: 1,
            socd_pairs
        }
    }
}

impl Default for KeyRTConfig {
    fn default() -> Self {
        Self {
            PressPercentage: 16,
            ReleasePercentage: 16,
            DeadZone: 30,
            ReleaseDeadZone: 0,
            RtEnabled: true,
        }
    }
}


impl Default for SOCDPairConfig {
    fn default() -> Self {
        Self {
            key1: 0,
            key2: 0,
        }
    }
}
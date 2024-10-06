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
    #[serde_as(as = "[_; 64]")]
    pub KeyConfigs: [KeyRTConfig; 64],
    #[serde(rename = "km")]
    #[serde_as(as = "[_; 256]")]
    pub KeyMap: [u8; 256],
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

impl From<config::Device> for Device {
    fn from(cfg: config::Device) -> Self {
        let mut key_configs = [KeyRTConfig::default(); 64];
        for i in 0..64 {
            key_configs[i] = cfg.keys[i].into();
        }
        let map = keymap::KeyMap::from([cfg.normal_layer, cfg.fn_layer]).into();

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
        }
    }
}

impl Default for Device {
    fn default() -> Self {
        let key_configs = [KeyRTConfig::default(); 64];
        let key_maps = keymap::KeyMap::default();
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


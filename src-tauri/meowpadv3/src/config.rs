use crate::{cbor, keymap::{self, KeyMatrix, ROW_SIZE, TOTAL_KEYS}, HALL_KEY_NUMS};
use meowpad::{error::Error, keycode::KeyValue, KbReport, KeyCode};
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
}


#[derive(Serialize, Deserialize, Clone, Debug, Default, Copy)]
pub struct SOCDKeyPairs {
    pub key1: u8,
    pub key2: u8,
}


#[derive(Serialize, Deserialize, Clone, Debug, Default, Copy)]
pub struct KeyConfig {
    pub press_percentage: u8,
    pub release_percentage: u8,
    pub dead_zone: u8,   // 0-30
    pub release_dead_zone: u8,   // 0-30
    pub rt_enabled: bool
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Device {
    #[serde_as(as = "[_; HALL_KEY_NUMS]")]
    pub keys: [KeyConfig; HALL_KEY_NUMS],
    #[serde_as(as = "[[_; ROW_SIZE]; TOTAL_KEYS]")]
    pub layer: KeyMatrix,
    pub jitters_elimination_time: u16,
    pub high_reportrate: bool,
    pub key_proof: bool,
    pub auto_calibration: bool,
    pub hall_filter: u8,
    pub max_brightness: u8,
    pub led_color: Srgb<u8>,
    pub socd_key_pairs: Vec<SOCDKeyPairs>,
    pub led_mode: LightingMode,
    pub sleep_timeout: u16
}

impl Default for Device {
    fn default() -> Self {
        cbor::Device::default().try_into().unwrap()
    }
}

impl TryFrom<cbor::Device> for Device {
    type Error = Error;
    fn try_from(cfg: cbor::Device) -> Result<Self, Self::Error> {
        let mut keys = [KeyConfig::default(); HALL_KEY_NUMS];
        for i in 0..HALL_KEY_NUMS {
            keys[i] = KeyConfig::from(cfg.KeyConfigs[i]);
        }
        let map: KeyMatrix = keymap::KeyMap::from(cfg.KeyMap).into();

        Ok(Device {
            keys,
            layer: map,
            high_reportrate: cfg.HighReportRate,
            key_proof: cfg.KeyProof,
            auto_calibration: cfg.AutoCalibration,
            hall_filter: cfg.HallFilter,
            jitters_elimination_time: cfg.JittersEliminationTime / 8,
            max_brightness: cfg.MaxBrightness,
            led_color: Srgb::from_u32::<Argb>(cfg.led_color),
            socd_key_pairs: cfg.socd_pairs[..cfg.socd_pair_count as usize].iter().map(|p| SOCDKeyPairs::from(*p)).collect(),
            led_mode: LightingMode::from_u8(cfg.led_mode).ok_or(
                Error::ConfigDataCheckFailed("led_mode", cfg.led_mode as usize),
            )?,
            sleep_timeout: cfg.sleep_timeout,
        })
    }
}

impl From<cbor::KeyRTConfig> for KeyConfig {
    fn from(cfg: cbor::KeyRTConfig) -> Self {
        Self {
            press_percentage: cfg.PressPercentage / 2,
            release_percentage: cfg.ReleasePercentage / 2,
            dead_zone: cfg.DeadZone / 2,
            release_dead_zone: cfg.ReleaseDeadZone / 2,
            rt_enabled: cfg.RtEnabled
        }
    }
}

impl From<cbor::SOCDPairConfig> for SOCDKeyPairs {
    fn from(cfg: cbor::SOCDPairConfig) -> Self {
        Self {
            key1: cfg.key1,
            key2: cfg.key2,
        }
    }
}
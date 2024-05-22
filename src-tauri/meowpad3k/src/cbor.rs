use crate::config::{self, LightingMode};
use meowpad::{KeyCode, KbReport};
use palette::rgb::channels::Argb;
use palette::Srgb;
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::io::Cursor;

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
    pub KeyConfigs: [KeyRTConfig; 3],
    #[serde(rename = "sbd")]
    pub SideBtnKeyData: [u8; 6],
    #[serde(rename = "jet")]
    pub JittersEliminationTime: u16,
    #[serde(rename = "cr")]
    pub ContinuousReport: bool,
    #[serde(rename = "kf")]
    pub KalmanFilter: bool,
}

/// speed 0-10
/// steep length 50 - 200
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Light {
    #[serde(rename = "c")]
    pub led_colors: [u32; 3],
    #[serde(rename = "km")]
    pub led_mode: u8,
    #[serde(rename = "mb")]
    pub max_brightness: u8,
    #[serde(rename = "st")]
    pub sleep_time: u16,
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
        let mut key_configs = [KeyRTConfig::default(); 3];
        for i in 0..3 {
            key_configs[i] = cfg.keys[i].into();
        }

        Keyboard {
            KeyConfigs: key_configs,
            SideBtnKeyData: cfg.side_btn.into_iter().collect::<KbReport>().into(),
            ContinuousReport: cfg.continuous_report,
            KalmanFilter: cfg.kalman_filter,
            JittersEliminationTime: cfg.jitters_elimination_time,
        }
    }
}

impl From<config::Light> for Light {
    fn from(cfg: config::Light) -> Self {
        let mut led_colors = [0u32; 3];
        for i in 0..3 {
            led_colors[i] = cfg.led_colors[i].into_u32::<Argb>();
        }

        Light {
            led_colors: led_colors,
            led_mode: cfg.lighting_mode as u8,
            max_brightness: cfg.max_brightness,
            sleep_time: cfg.sleep_time,
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        let led_colors = [
            Srgb::new(255, 255, 255).into_u32::<Argb>(),
            Srgb::new(255, 255, 255).into_u32::<Argb>(),
            Srgb::new(255, 255, 255).into_u32::<Argb>(),
        ];
        Self {
            led_colors: led_colors,
            led_mode: LightingMode::Solid as u8,
            max_brightness: 50,
            sleep_time: 120,
        }
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        let key_configs = [
            KeyRTConfig::with_key(KeyCode::Z),
            KeyRTConfig::with_key(KeyCode::X),
            KeyRTConfig::with_key(KeyCode::C),
            // KeyRTConfig::default(),
        ];
        Self {
            KeyConfigs: key_configs,
            SideBtnKeyData: KeyCode::Escape.to_report().into(),
            ContinuousReport: false,
            KalmanFilter: true,
            JittersEliminationTime: 15,
        }
    }
}

impl Default for KeyRTConfig {
    fn default() -> Self {
        Self {
            PressPercentage: 10,
            ReleasePercentage: 10,
            DeadZone: 20,
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

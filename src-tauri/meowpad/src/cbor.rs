use crate::{Config, KbReport, KeyCode, LightingMode, KeyConfig};
use ciborium;
use palette::rgb::channels::Argb;
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
pub struct KEYBOARD {
    #[serde(rename = "ks")]
    pub KeyConfigs: [KeyRTConfig; 4],
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
pub struct LED {
    #[serde(rename = "c")]
    pub LED_COLORS: [u32; 4],
    #[serde(rename = "km")]
    pub LED_KEY_MODE: u8,
    #[serde(rename = "mb")]
    pub LED_MAX_BRIGHTNESS: u8,
    #[serde(rename = "st")]
    pub LED_SLEEP_TIME: u16,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Default)]
#[allow(non_snake_case)]
pub struct CONFIG {
    #[serde(rename = "k")]
    pub Key: KEYBOARD,
    #[serde(rename = "l")]
    pub LED: LED,
}

impl CONFIG {
    pub fn from_cbor<T: AsRef<[u8]>>(
        data: T,
    ) -> Result<CONFIG, ciborium::de::Error<std::io::Error>> {
        Ok(ciborium::de::from_reader(Cursor::new(data))?)
    }

    pub fn to_cbor(self) -> Vec<u8> {
        let mut data = vec![];
        ciborium::ser::into_writer(&self, &mut data).unwrap();
        data
    }
}

impl Default for KeyRTConfig {
    fn default() -> Self {
        Self {
            PressPercentage: 20,
            ReleasePercentage: 25,
            DeadZone: 10,
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

impl Default for KEYBOARD {
    fn default() -> Self {
        let key_configs = [
            KeyRTConfig::with_key(KeyCode::Z),
            KeyRTConfig::with_key(KeyCode::X),
            KeyRTConfig::with_key(KeyCode::Grave),
            KeyRTConfig::with_key(KeyCode::Escape),
            // KeyRTConfig::default(),
            // KeyRTConfig::default(),
            // KeyRTConfig::default(),
        ];
        Self {
            KeyConfigs: key_configs,
            ContinuousReport: false,
            KalmanFilter: true,
            JittersEliminationTime: 15 * 8,
        }
    }
}

impl From<KeyConfig> for KeyRTConfig {
    fn from(cfg: KeyConfig) -> Self {
        Self {
            PressPercentage: cfg.press_percentage,
            ReleasePercentage: cfg.release_percentage,
            DeadZone: cfg.dead_zone,
            KeyData: cfg.key_data.into_iter().collect::<KbReport>().into(),
        }
    }
}

impl From<Config> for CONFIG {
    fn from(cfg: Config) -> Self {
        let mut led_colors = [0u32; 4];
        for i in 0..4 {
            led_colors[i] = cfg.led_colors[i].into_u32::<Argb>();
        }

        let mut key_configs = [KeyRTConfig::default(); 4];
        for i in 0..4 {
            key_configs[i] = cfg.keys[i].into();
        }

        Self {
            Key: KEYBOARD {
                KeyConfigs: key_configs,
                ContinuousReport: cfg.continuous_report,
                KalmanFilter: cfg.kalman_filter,
                JittersEliminationTime: cfg.jitters_elimination_time,
            },
            LED: LED {
                LED_COLORS: led_colors,
                LED_KEY_MODE: cfg.lighting_mode_key as u8,
                LED_MAX_BRIGHTNESS: cfg.max_brightness,
                ..Default::default()
            },
        }
    }
}

impl Default for LED {
    fn default() -> Self {
        let led_colors = [
            Srgb::new(255, 255, 255).into_u32::<Argb>(),
            Srgb::new(255, 255, 255).into_u32::<Argb>(),
            Srgb::new(255, 255, 255).into_u32::<Argb>(),
            Srgb::new(255, 255, 255).into_u32::<Argb>(),
        ];
        Self {
            LED_COLORS: led_colors,
            LED_KEY_MODE: LightingMode::Solid as u8,
            LED_MAX_BRIGHTNESS: 10,
            LED_SLEEP_TIME: 120,
        }
    }
}

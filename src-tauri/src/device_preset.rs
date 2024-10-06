use meowpad::keycode::KeyValue;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::device::DeviceInfoSerdi;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevicePreset {
    pub id: String,
    pub name: String,
    pub device: DevicePresetInfo,
    pub config: DevicePresetConfig
}

impl DevicePreset {
    pub fn new(name: &str, device: impl Into<DevicePresetInfo>, config: impl Into<DevicePresetConfig>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            device: device.into(),
            config: config.into()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevicePresetInfo {
    pub device_name: String,
    pub serial_number: Option<String>,
}

impl From<DeviceInfoSerdi> for DevicePresetInfo {
    fn from(value: DeviceInfoSerdi) -> Self {
        Self {
            device_name: value.device_name,
            serial_number: value.serial_number
        }
    }
}

impl From<&str> for DevicePresetInfo {
    fn from(value: &str) -> Self {
        Self {
            device_name: value.to_string(),
            serial_number: None
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevicePresetConfig {
    pub key_layers: Option<Vec<KeyLayer>>,
    pub key_configs: Option<Vec<KeyConfig>>,
}

impl From<meowboard::config::Device> for DevicePresetConfig {
    fn from(value: meowboard::config::Device) -> Self {
        let mut key_layers = vec![];
        let mut key_configs = vec![];

        key_layers.push(KeyLayer { keys: value.normal_layer.to_vec() });
        key_layers.push(KeyLayer { keys: value.fn_layer.to_vec() });

        for key in value.keys {
            key_configs.push(key.into());
        }

        Self { key_layers: Some(key_layers), key_configs: Some(key_configs) }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyLayer {
    pub keys: Vec<KeyValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct KeyConfig {
    pub press_percentage: u32,
    pub release_percentage: u32,
    pub dead_zone: u32,
    pub release_dead_zone: u32,
    pub rt_enabled: Option<bool>,
}

impl From<meowboard::config::KeyConfig> for KeyConfig {
    fn from(value: meowboard::config::KeyConfig) -> Self {
        Self {
            press_percentage: value.press_percentage as u32,
            release_percentage: value.release_percentage as u32,
            dead_zone: value.dead_zone as u32,
            release_dead_zone: value.release_dead_zone as u32,
            rt_enabled: Some(value.rt_enabled)
        }
    }
}

impl From<KeyConfig> for meowboard::config::KeyConfig {
    fn from(value: KeyConfig) -> Self {
        Self {
            press_percentage: value.press_percentage as u8,
            release_percentage: value.release_percentage as u8,
            dead_zone: value.dead_zone as u8,
            release_dead_zone: value.release_dead_zone as u8,
            rt_enabled: value.rt_enabled.unwrap_or(true)
        }
    }
}
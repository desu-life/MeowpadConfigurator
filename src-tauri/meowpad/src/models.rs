use num_derive::{FromPrimitive, ToPrimitive};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Default, FromPrimitive, ToPrimitive)]
pub enum KeyState {
    #[default]
    Pressed,
    Released,
    Calibrating
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Default)]
pub struct KeyRTStatus {
    pub adc_value: u16,
    pub linear_value: u16,
    pub press_percentage: u8,
    pub key_state: KeyState,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Default)]
pub struct KeyHallConfig {
    pub adc_min: u16,
    pub adc_max: u16,
    pub hall_middle: u16,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Default)]
pub struct DeviceStatus {
    pub key: bool,
    pub light: bool,
    pub hall: bool,
    pub enabled: bool,
}
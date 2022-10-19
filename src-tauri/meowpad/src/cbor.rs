use crate::{Config, KeyCode};
use anyhow::{anyhow, Result};
use byteorder::{ReadBytesExt, WriteBytesExt};
use ciborium;
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use serde_repr::*;
use serde_with::*;
use std::{io::Cursor, time::Duration};
use palette::rgb::channels::Rgba;
use palette::Srgb;

pub unsafe fn serialize_row<T: Sized>(src: &T) -> &[u8] {
    ::std::slice::from_raw_parts((src as *const T) as *const u8, ::std::mem::size_of::<T>())
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct KEYBOARD {
    #[serde(rename = "k1")]
    pub KEY_1: u8,
    #[serde(rename = "k2")]
    pub KEY_2: u8,
    #[serde(rename = "k3")]
    pub KEY_3: u8,
    #[serde(rename = "k4")]
    pub KEY_4: u8,
    #[serde(rename = "k5")]
    pub KEY_5: u8,
    #[serde(rename = "jet")]
    pub JittersEliminationTime: u16,
    /// 0 / 1   0是按下按键先消抖后发码
    #[serde(rename = "jm")]
    pub JittersEliminationMode: u8,
}

/// speed 0-10
/// steep length 50 - 200
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct LED {
    #[serde(rename = "1c")]
    pub LED1_COLOR: u32,
    #[serde(rename = "2c")]
    pub LED2_COLOR: u32,
    #[serde(rename = "3c")]
    pub LED3_COLOR: u32,
    #[serde(rename = "4c")]
    pub LED4_COLOR: u32,
    #[serde(rename = "slhc")]
    pub LED_SPEED_LIGHT_HIGH_COLOR: u32,
    #[serde(rename = "sllc")]
    pub LED_SPEED_LIGHT_LOW_COLOR: u32,
    #[serde(rename = "km")]
    pub LED_KEY_MODE: u8,
    #[serde(rename = "bm")]
    pub LED_BTM_MODE: u8,
    #[serde(rename = "lmx")]
    pub LED_LUM_MAX: u8,
    #[serde(rename = "lbmn")]
    pub LED_LUM_BREATH_MIN: u8,
    #[serde(rename = "lslmn")]
    pub LED_LUM_SPEED_LIGHT_MIN: u8,
    #[serde(rename = "lptgmn")]
    pub LED_LUM_PUSH_TO_GLOW_MIN: u8,
    #[serde(rename = "rcts")]
    pub LED_RAINBOW_CYCLE_TRANSITION_SPEED: u16,
    #[serde(rename = "rckt")]
    pub LED_RAINBOW_CYCLE_KEEP_TIME: u16,
    #[serde(rename = "blmxkt")]
    pub LED_BREATH_LUM_MAX_KEEP_TIME: u16,
    #[serde(rename = "blmnkt")]
    pub LED_BREATH_LUM_MIN_KEEP_TIME: u16,
    #[serde(rename = "blts")]
    pub LED_BREATH_LUM_TRANSITION_SPEED: u16,
    #[serde(rename = "ptgts")]
    pub LED_PUSH_TO_GLOW_TRANSITION_SPEED: u16,
    #[serde(rename = "slts")]
    pub LED_SPEED_LIGHT_TRANSITION_SPEED: u16,
    #[serde(rename = "slsl")]
    pub LED_SPEED_LIGHT_STEP_LENGTH: u16,
    /// 2-10
    #[serde(rename = "rbsc1")]
    pub LED_RAINBOW_BREATH_SWITCH_COLOR_COUNT: u16,
    #[serde(rename = "rbsc2")]
    pub LED_RAINBOW_BREATH_SYNC_COLOR_COUNT: u16,

}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct CONFIG {
    #[serde(rename = "k")]
    pub Key: KEYBOARD,
    #[serde(rename = "l")]
    pub LED: LED,
}

impl CONFIG {
    pub fn from_cbor<T: AsRef<[u8]>>(data: T) -> Result<CONFIG, ciborium::de::Error<std::io::Error>> {
        Ok(ciborium::de::from_reader(Cursor::new(data))?)
    }

    pub fn to_cbor(self) -> Vec<u8> {
        let mut data = vec![];
        ciborium::ser::into_writer(&self, &mut data).unwrap();
        data
    }

    pub unsafe fn from_raw(src: Vec<u8>) -> Self {
        std::ptr::read(src.as_ptr() as *const _)
    }
}

impl Default for KEYBOARD {
    fn default() -> Self {
        Self {
            KEY_1: KeyCode::KEY_Z as u8,
            KEY_2: KeyCode::KEY_X as u8,
            KEY_3: KeyCode::KEY_ESC as u8,
            KEY_4: KeyCode::KEY_F2 as u8,
            KEY_5: KeyCode::KEY_GRAVE as u8,
            JittersEliminationTime: 100,
            JittersEliminationMode: 1,
        }
    }
}

impl From<Config> for CONFIG {
    fn from(cfg: Config) -> Self {
        Self {
            Key: KEYBOARD {
                KEY_1: cfg.key_1 as u8,
                KEY_2: cfg.key_2 as u8,
                KEY_3: cfg.key_3 as u8,
                KEY_4: cfg.key_4 as u8,
                KEY_5: cfg.key_5 as u8,
                JittersEliminationTime: cfg.keyboard_jitters_elimination_time,
                ..Default::default()
            },
            LED: LED {
                LED1_COLOR: cfg.led_color_l.into_u32::<Rgba>(),
                LED2_COLOR: cfg.led_color_r.into_u32::<Rgba>(),
                LED3_COLOR: cfg.led_color_btm_l.into_u32::<Rgba>(),
                LED4_COLOR: cfg.led_color_btm_r.into_u32::<Rgba>(),
                LED_SPEED_LIGHT_HIGH_COLOR: cfg.speed_press_high_color.into_u32::<Rgba>(),
                LED_SPEED_LIGHT_LOW_COLOR: cfg.speed_press_low_color.into_u32::<Rgba>(),
                LED_KEY_MODE: cfg.lighting_mode_key as u8,
                LED_BTM_MODE: cfg.lighting_mode_btm as u8,
                LED_LUM_MAX: cfg.maximum_brightness,
                LED_LUM_BREATH_MIN: cfg.breath_minimum_brightness,
                LED_LUM_SPEED_LIGHT_MIN: cfg.press_light_minimum_brightness,        // ？
                LED_LUM_PUSH_TO_GLOW_MIN: cfg.press_light_minimum_brightness,       // ？
                LED_RAINBOW_CYCLE_TRANSITION_SPEED: cfg.rainbow_light_switching_speed.as_millis() as u16,   // ??
                LED_RAINBOW_CYCLE_KEEP_TIME: cfg.rainbow_light_switching_speed.as_millis() as u16,      // ??
                LED_BREATH_LUM_MAX_KEEP_TIME: cfg.breath_maximum_light_duration.as_millis() as u16,
                LED_BREATH_LUM_MIN_KEEP_TIME: cfg.breath_minimum_light_duration.as_millis() as u16,
                LED_BREATH_LUM_TRANSITION_SPEED: cfg.breath_interval.as_millis() as u16,
                LED_PUSH_TO_GLOW_TRANSITION_SPEED: cfg.press_light_duration.as_millis() as u16,
                LED_SPEED_LIGHT_TRANSITION_SPEED: cfg.speed_press_trans_speed.as_millis() as u16,
                LED_SPEED_LIGHT_STEP_LENGTH: cfg.press_light_step,
                ..Default::default()
            },
        }
    }
}

impl Default for LED {
    fn default() -> Self {
        Self {
            LED1_COLOR: Srgb::new(204, 255, 229).into_u32::<Rgba>(),
            LED2_COLOR: Srgb::new(255, 153, 204).into_u32::<Rgba>(),
            LED3_COLOR: Srgb::new(255, 153, 204).into_u32::<Rgba>(),
            LED4_COLOR: Srgb::new(255, 153, 204).into_u32::<Rgba>(),
            LED_SPEED_LIGHT_HIGH_COLOR: Srgb::new(255, 153, 204).into_u32::<Rgba>(),
            LED_SPEED_LIGHT_LOW_COLOR: Srgb::new(255, 153, 204).into_u32::<Rgba>(),
            LED_KEY_MODE: 7,
            LED_BTM_MODE: 5,
            LED_LUM_MAX: 100,
            LED_LUM_BREATH_MIN: 0,
            LED_LUM_SPEED_LIGHT_MIN: 100,
            LED_LUM_PUSH_TO_GLOW_MIN: 0,
            LED_RAINBOW_CYCLE_TRANSITION_SPEED: 10,
            LED_RAINBOW_CYCLE_KEEP_TIME: 10,
            LED_BREATH_LUM_MAX_KEEP_TIME: 100,
            LED_BREATH_LUM_MIN_KEEP_TIME: 0,
            LED_BREATH_LUM_TRANSITION_SPEED: 8,
            LED_PUSH_TO_GLOW_TRANSITION_SPEED: 1,
            LED_SPEED_LIGHT_TRANSITION_SPEED: 0,
            LED_SPEED_LIGHT_STEP_LENGTH: 20,
            LED_RAINBOW_BREATH_SWITCH_COLOR_COUNT: 2,
            LED_RAINBOW_BREATH_SYNC_COLOR_COUNT: 2
        }
    }
}

impl Default for CONFIG {
    fn default() -> Self {
        Self {
            Key: KEYBOARD::default(),
            LED: LED::default(),
        }
    }
}
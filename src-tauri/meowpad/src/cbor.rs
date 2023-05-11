use crate::{Config, KeyCode, KbReport, LightingMode};
use anyhow::Result;
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
pub struct KEYBOARD {
    #[serde(rename = "k1")]
    pub KEY_1: [u8; 4],
    #[serde(rename = "k2")]
    pub KEY_2: [u8; 4],
    #[serde(rename = "k3")]
    pub KEY_3: [u8; 4],
    #[serde(rename = "k4")]
    pub KEY_4: [u8; 4],
    #[serde(rename = "k5")]
    pub KEY_5: [u8; 4],
    #[serde(rename = "jet")]
    pub JittersEliminationTime: u16,
    /// 0 / 1   0是按下按键先消抖后发码
    #[serde(rename = "jm")]
    pub JittersEliminationMode: u8,
    #[serde(rename = "fkw")]
    pub FORCE_KEY_SWITCH: u8,
    #[serde(rename = "td", skip_serializing_if = "Option::is_none")]
    pub KeyTriggerDegree: Option<u8>,
    #[serde(rename = "rd", skip_serializing_if = "Option::is_none")]
    pub KeyReleaseDegree: Option<u8>,
    #[serde(rename = "dz", skip_serializing_if = "Option::is_none")]
    pub DeadZone: Option<u8>,
    #[serde(rename = "r", skip_serializing_if = "Option::is_none")]
    pub KeyScanRate: Option<u8>,
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
    /// 颜色变化的间隔时间
    #[serde(rename = "rcts")]
    pub LED_RAINBOW_CYCLE_TRANSITION_SPEED: u16,
    /// 每个颜色的停留时间
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
    #[serde(rename = "dsit")]
    pub DEVICE_SLEEP_IDLE_TIME: u16,
    #[serde(rename = "llsm")]
    pub LED_LUM_SLEEP_MAX: u8,
    #[serde(rename = "lksm")]
    pub LED_KEY_SLEEP_MODE: u8,
    #[serde(rename = "lbsm")]
    pub LED_BTM_SLEEP_MODE: u8,
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

impl KEYBOARD {
    fn default(is_wooting: bool) -> Self {
        if is_wooting {
            Self {
                KEY_1: KeyCode::Z.to_report().into(),
                KEY_2: KeyCode::X.to_report().into(),
                KEY_3: KeyCode::Escape.to_report().into(),
                KEY_4: KeyCode::F2.to_report().into(),
                KEY_5: KeyCode::Grave.to_report().into(),
                JittersEliminationTime: 100,
                JittersEliminationMode: 1,
                FORCE_KEY_SWITCH: false as u8,
                KeyReleaseDegree: Some(25),
                KeyTriggerDegree: Some(20),
                DeadZone: Some(10),
                KeyScanRate: Some(0)
            }
        } else {
            Self {
                KEY_1: KeyCode::Z.to_report().into(),
                KEY_2: KeyCode::X.to_report().into(),
                KEY_3: KeyCode::Escape.to_report().into(),
                KEY_4: KeyCode::F2.to_report().into(),
                KEY_5: KeyCode::Grave.to_report().into(),
                JittersEliminationTime: 100,
                JittersEliminationMode: 1,
                FORCE_KEY_SWITCH: false as u8,
                KeyReleaseDegree: None,
                KeyTriggerDegree: None,
                DeadZone: None,
                KeyScanRate: None
            }
        }
    }
}

impl From<Config> for CONFIG {
    fn from(cfg: Config) -> Self {
        Self {
            Key: KEYBOARD {
                KEY_1: cfg.key_1.into_iter().collect::<KbReport>().into(),
                KEY_2: cfg.key_2.into_iter().collect::<KbReport>().into(),
                KEY_3: cfg.key_3.into_iter().collect::<KbReport>().into(),
                KEY_4: cfg.key_4.into_iter().collect::<KbReport>().into(),
                KEY_5: cfg.key_5.into_iter().collect::<KbReport>().into(),
                JittersEliminationTime: cfg.keyboard_jitters_elimination_time,
                JittersEliminationMode: cfg.keyboard_jitters_elimination_mode as u8,
                FORCE_KEY_SWITCH: cfg.force_key_switch.into(),
                KeyReleaseDegree: cfg.key_release_degree,
                KeyTriggerDegree: cfg.key_trigger_degree,
                DeadZone: cfg.dead_zone,
                KeyScanRate: cfg.key_scan_rate,
            },
            LED: LED {
                LED1_COLOR: cfg.led_color_l.into_u32::<Argb>(),
                LED2_COLOR: cfg.led_color_r.into_u32::<Argb>(),
                LED3_COLOR: cfg.led_color_btm_r.into_u32::<Argb>(),
                LED4_COLOR: cfg.led_color_btm_l.into_u32::<Argb>(),
                LED_SPEED_LIGHT_HIGH_COLOR: cfg.speed_press_high_color.into_u32::<Argb>(),
                LED_SPEED_LIGHT_LOW_COLOR: cfg.speed_press_low_color.into_u32::<Argb>(),
                LED_KEY_MODE: cfg.lighting_mode_key as u8,
                LED_BTM_MODE: cfg.lighting_mode_btm as u8,
                LED_LUM_MAX: cfg.maximum_brightness,
                LED_LUM_BREATH_MIN: cfg.breath_minimum_brightness,
                LED_LUM_SPEED_LIGHT_MIN: cfg.speed_press_minimum_brightness,
                LED_LUM_PUSH_TO_GLOW_MIN: cfg.press_light_minimum_brightness,
                LED_RAINBOW_CYCLE_TRANSITION_SPEED: cfg.rainbow_light_switching_interval.as_millis() as u16,    // 两个同值
                LED_RAINBOW_CYCLE_KEEP_TIME: cfg.rainbow_light_switching_interval.as_millis() as u16,
                LED_BREATH_LUM_MAX_KEEP_TIME: cfg.breath_maximum_light_duration.as_millis() as u16,
                LED_BREATH_LUM_MIN_KEEP_TIME: cfg.breath_minimum_light_duration.as_millis() as u16,
                LED_BREATH_LUM_TRANSITION_SPEED: cfg.breath_interval.as_millis() as u16,
                LED_PUSH_TO_GLOW_TRANSITION_SPEED: cfg.press_light_duration.as_millis() as u16,
                LED_SPEED_LIGHT_TRANSITION_SPEED: cfg.speed_press_trans_speed.as_millis() as u16,
                LED_SPEED_LIGHT_STEP_LENGTH: cfg.speed_press_step,
                DEVICE_SLEEP_IDLE_TIME: cfg.device_sleep_idle_time.as_secs() as u16,
                LED_LUM_SLEEP_MAX: cfg.sleep_mode_maximum_brightness,
                LED_KEY_SLEEP_MODE: cfg.sleep_lighting_mode_key as u8,
                LED_BTM_SLEEP_MODE: cfg.sleep_lighting_mode_btm as u8,
                ..Default::default()
            },
        }
    }
}

impl Default for LED {
    fn default() -> Self {
        Self {
            LED1_COLOR: Srgb::new(255, 255, 255).into_u32::<Argb>(),
            LED2_COLOR: Srgb::new(255, 255, 255).into_u32::<Argb>(),
            LED3_COLOR: Srgb::new(255, 255, 255).into_u32::<Argb>(),
            LED4_COLOR: Srgb::new(255, 255, 255).into_u32::<Argb>(),
            LED_SPEED_LIGHT_HIGH_COLOR: Srgb::new(255, 0, 0).into_u32::<Argb>(),
            LED_SPEED_LIGHT_LOW_COLOR: Srgb::new(255, 255, 255).into_u32::<Argb>(),
            LED_KEY_MODE: LightingMode::PressAndLight as u8,
            LED_BTM_MODE: LightingMode::RainbowGradientSwitch as u8,
            LED_LUM_MAX: 80,
            LED_LUM_BREATH_MIN: 0,
            LED_LUM_SPEED_LIGHT_MIN: 100,
            LED_LUM_PUSH_TO_GLOW_MIN: 0,
            LED_RAINBOW_CYCLE_TRANSITION_SPEED: 25,
            LED_RAINBOW_CYCLE_KEEP_TIME: 25,
            LED_BREATH_LUM_MAX_KEEP_TIME: 100,
            LED_BREATH_LUM_MIN_KEEP_TIME: 0,
            LED_BREATH_LUM_TRANSITION_SPEED: 15,
            LED_PUSH_TO_GLOW_TRANSITION_SPEED: 1,
            LED_SPEED_LIGHT_TRANSITION_SPEED: 1,
            LED_SPEED_LIGHT_STEP_LENGTH: 50,
            LED_RAINBOW_BREATH_SWITCH_COLOR_COUNT: 2,
            LED_RAINBOW_BREATH_SYNC_COLOR_COUNT: 2,
            DEVICE_SLEEP_IDLE_TIME: 120,
            LED_LUM_SLEEP_MAX: 100,
            LED_KEY_SLEEP_MODE: LightingMode::Off as u8,
            LED_BTM_SLEEP_MODE: LightingMode::Off as u8,
        }
    }
}

impl CONFIG {
    pub fn default(is_wooting: bool) -> Self {
        Self {
            Key: KEYBOARD::default(is_wooting),
            LED: LED::default(),
        }
    }
}

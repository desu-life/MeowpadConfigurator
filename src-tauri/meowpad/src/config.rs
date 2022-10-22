use crate::{cbor::CONFIG, KeyCode, KbReport};
use anyhow::{anyhow, Result};
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use palette::rgb::channels::Argb;
use palette::Srgb;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use serde_with::*;
use std::time::Duration;

#[derive(
    Serialize_repr, Deserialize_repr, FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, PartialEq,
)]
#[repr(u8)]
pub enum LighingMode {
    Off = 0,
    Solid = 1,
    Breath = 2,
    RainbowBreathSwitch = 3,
    RainbowBreathSync = 4,
    RainbowGradientSwitch = 5,
    RainbowGradientSync = 6,
    PressAndLight = 7,
    SpeedPress = 8,
}

/// 0是按下按键先消抖后发码
/// 1先发码后消抖
#[derive(
    Serialize_repr, Deserialize_repr, FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, PartialEq,
)]
#[repr(u8)]
pub enum JitterEliminationMode {
    Active = 0,
    Lazy = 1,
}

#[serde_as]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Config {
    pub key_1: [KeyCode; 4],
    pub key_2: [KeyCode; 4],
    pub key_3: [KeyCode; 4],
    pub key_4: [KeyCode; 4],
    pub key_5: [KeyCode; 4],
    /// LED灯颜色
    pub led_color_l: Srgb<u8>,
    /// LED灯颜色
    pub led_color_r: Srgb<u8>,
    /// LED灯颜色
    pub led_color_btm_l: Srgb<u8>,
    /// LED灯颜色
    pub led_color_btm_r: Srgb<u8>,
    /// 灯效模式
    pub lighting_mode_key: LighingMode,
    /// 灯效模式
    pub lighting_mode_btm: LighingMode,
    /// 最大亮度(常亮、呼吸、渐变)
    pub maximum_brightness: u8,
    /// 最小亮度(呼吸+渐变灯ONLY)
    pub breath_minimum_brightness: u8,
    /// 呼吸灯最大亮度保持时间(ms)
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub breath_maximum_light_duration: Duration,
    /// 呼吸灯最小量度保持时间(ms)
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub breath_minimum_light_duration: Duration,
    /// 呼吸灯过渡间隔时间(ms)
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub breath_interval: Duration,
    /// 即按即亮最小亮度
    pub press_light_minimum_brightness: u8,
    /// 即按即亮衰减速度(ms)
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub press_light_duration: Duration,
    /// 渐变灯切换间隔时间(ms)
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub rainbow_light_switching_interval: Duration,
    /// 手速灯最快速颜色
    pub speed_press_high_color: Srgb<u8>,
    /// 手速灯最快速颜色
    pub speed_press_low_color: Srgb<u8>,
    /// 手速灯颜色切换速度(ms)
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub speed_press_trans_speed: Duration,
    /// 手速灯最小亮度
    pub speed_press_minimum_brightness: u8,
    /// 手速灯变速步长
    pub speed_press_step: u16,
    /// 后消抖延时 TIME*100*14/1000 单位us
    pub keyboard_jitters_elimination_time: u16,
    pub keyboard_jitters_elimination_mode: JitterEliminationMode,
}

impl TryFrom<CONFIG> for Config {
    type Error = anyhow::Error;
    fn try_from(cfg: CONFIG) -> Result<Self, Self::Error> {
        Ok(Config {
            key_1: KbReport::from(cfg.Key.KEY_1).try_into().map_err(|_| anyhow!("解析k1时报错"))?,
            key_2: KbReport::from(cfg.Key.KEY_2).try_into().map_err(|_| anyhow!("解析k2时报错"))?,
            key_3: KbReport::from(cfg.Key.KEY_3).try_into().map_err(|_| anyhow!("解析k3时报错"))?,
            key_4: KbReport::from(cfg.Key.KEY_4).try_into().map_err(|_| anyhow!("解析k4时报错"))?,
            key_5: KbReport::from(cfg.Key.KEY_5).try_into().map_err(|_| anyhow!("解析k5时报错"))?,
            led_color_l: Srgb::from_u32::<Argb>(cfg.LED.LED1_COLOR),
            led_color_r: Srgb::from_u32::<Argb>(cfg.LED.LED2_COLOR),
            led_color_btm_r: Srgb::from_u32::<Argb>(cfg.LED.LED3_COLOR),
            led_color_btm_l: Srgb::from_u32::<Argb>(cfg.LED.LED4_COLOR),
            lighting_mode_key: LighingMode::from_u8(cfg.LED.LED_KEY_MODE)
                .ok_or(anyhow!("解析key灯效模式时报错"))?,
            lighting_mode_btm: LighingMode::from_u8(cfg.LED.LED_BTM_MODE)
                .ok_or(anyhow!("解析btm灯效模式时报错"))?,
            maximum_brightness: cfg.LED.LED_LUM_MAX,
            breath_minimum_brightness: cfg.LED.LED_LUM_BREATH_MIN,
            breath_maximum_light_duration: Duration::from_millis(
                cfg.LED.LED_BREATH_LUM_MAX_KEEP_TIME as u64,
            ),
            breath_minimum_light_duration: Duration::from_millis(
                cfg.LED.LED_BREATH_LUM_MIN_KEEP_TIME as u64,
            ),
            breath_interval: Duration::from_millis(cfg.LED.LED_BREATH_LUM_TRANSITION_SPEED as u64),
            press_light_minimum_brightness: cfg.LED.LED_LUM_PUSH_TO_GLOW_MIN,
            press_light_duration: Duration::from_millis(
                cfg.LED.LED_PUSH_TO_GLOW_TRANSITION_SPEED as u64,
            ),
            rainbow_light_switching_interval: Duration::from_millis(
                cfg.LED.LED_RAINBOW_CYCLE_TRANSITION_SPEED as u64,
            ),
            speed_press_high_color: Srgb::from_u32::<Argb>(cfg.LED.LED_SPEED_LIGHT_HIGH_COLOR),
            speed_press_low_color: Srgb::from_u32::<Argb>(cfg.LED.LED_SPEED_LIGHT_LOW_COLOR),
            speed_press_trans_speed: Duration::from_millis(
                cfg.LED.LED_SPEED_LIGHT_TRANSITION_SPEED as u64,
            ),
            speed_press_minimum_brightness: cfg.LED.LED_LUM_SPEED_LIGHT_MIN,
            speed_press_step: cfg.LED.LED_SPEED_LIGHT_STEP_LENGTH,
            keyboard_jitters_elimination_time: cfg.Key.JittersEliminationTime,
            keyboard_jitters_elimination_mode: JitterEliminationMode::from_u8(cfg.Key.JittersEliminationMode).ok_or(anyhow!("解析消抖模式时报错"))?
        })
    }
}

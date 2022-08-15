use crate::{KeyCode, RGB, str_to_cur};
use anyhow::{anyhow, Result};
use byteorder::{ReadBytesExt, WriteBytesExt};
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use serde_repr::*;
use serde_with::*;
use std::{
    io::Cursor,
    time::Duration,
};

const WRITE_BUFFER: &str = "WC";

#[derive(Serialize_repr, Deserialize_repr, FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum LighingMode {
    Off = 0,
    Solid = 1,
    Breath = 2,
    RainbowBreath = 3,
    RainbowGradient = 4,
    PressAndLight = 5,
    SpeedPress = 6,
}

#[serde_as]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub key_1: KeyCode,
    pub key_2: KeyCode,
    pub key_3: KeyCode,
    pub key_4: KeyCode,
    pub key_5: KeyCode,
    /// LED灯颜色
    #[serde_as(as = "DisplayFromStr")]
    pub led_color_l: RGB<u8>,
    /// LED灯颜色
    #[serde_as(as = "DisplayFromStr")]
    pub led_color_r: RGB<u8>,
    /// 灯效模式
    pub lighting_mode: LighingMode,
    /// 最大亮度(常亮、呼吸、渐变)
    pub maximum_brightness: u8,
    /// 最小亮度(呼吸+渐变灯ONLY)
    pub minimum_brightness: u8,
    /// 呼吸灯最大亮度保持时间(ms),因为是u8，除以十存储
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub breath_maximum_light_duration: Duration,
    /// 呼吸灯最小量度保持时间(ms),除以十存储
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub breath_minimum_light_duration: Duration,
    /// 呼吸灯过渡间隔时间(ms),无需处理
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub breath_interval: Duration,
    /// 即按即亮最大亮度
    pub press_light_maximum_brightness: u8,
    /// 即按即亮最小亮度
    pub press_light_minimum_brightness: u8,
    /// 即按即亮衰减速度(ms),无需处理
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub press_light_duration: Duration,
    /// 渐变灯切换速度(ms),除以十存储
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub fade_light_switching_speed: Duration,
    /// 手速灯最快速颜色
    #[serde_as(as = "DisplayFromStr")]
    pub speed_press_color_l: RGB<u8>,
    /// 手速灯最快速颜色
    #[serde_as(as = "DisplayFromStr")]
    pub speed_press_color_r: RGB<u8>,
    /// 手速灯颜色切换速度(ms),无需处理,最大值100ms 最小1ms
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub color_switching_speed: Duration,
    /// 手速灯变速步长
    pub press_light_step: u8,
    /// 后消抖延时 TIME*100*14/1000 单位us
    pub keyboard_jitters_elimination_time: u8,
}

impl Config {
    pub(crate) fn parse(raw_data: &[u8]) -> Result<Config> {
        let mut cur = Cursor::new(raw_data);
        Ok(Config {
            key_1: KeyCode::from_u8(cur.read_u8()?).ok_or(anyhow!("解析k1时报错"))?,
            key_2: KeyCode::from_u8(cur.read_u8()?).ok_or(anyhow!("解析k2时报错"))?,
            key_3: KeyCode::from_u8(cur.read_u8()?).ok_or(anyhow!("解析k3时报错"))?,
            key_4: KeyCode::from_u8(cur.read_u8()?).ok_or(anyhow!("解析k4时报错"))?,
            key_5: KeyCode::from_u8(cur.read_u8()?).ok_or(anyhow!("解析k5时报错"))?,
            led_color_l: RGB {
                r: cur.read_u8()?,
                g: cur.read_u8()?,
                b: cur.read_u8()?,
            },
            led_color_r: RGB {
                r: cur.read_u8()?,
                g: cur.read_u8()?,
                b: cur.read_u8()?,
            },
            lighting_mode: LighingMode::from_u8(cur.read_u8()?)
                .ok_or(anyhow!("解析灯效模式时报错"))?,
            maximum_brightness: cur.read_u8()?,
            minimum_brightness: cur.read_u8()?,
            // 将10乘回去
            breath_maximum_light_duration: Duration::from_millis(cur.read_u8()? as u64 * 10),
            breath_minimum_light_duration: Duration::from_millis(cur.read_u8()? as u64 * 10),
            breath_interval: Duration::from_millis(cur.read_u8()? as u64),
            press_light_maximum_brightness: cur.read_u8()?,
            press_light_minimum_brightness: cur.read_u8()?,
            press_light_duration: Duration::from_millis(cur.read_u8()? as u64),
            fade_light_switching_speed: Duration::from_millis(cur.read_u8()? as u64 * 10),
            speed_press_color_l: RGB {
                r: cur.read_u8()?,
                g: cur.read_u8()?,
                b: cur.read_u8()?,
            },
            speed_press_color_r: RGB {
                r: cur.read_u8()?,
                g: cur.read_u8()?,
                b: cur.read_u8()?,
            },
            color_switching_speed: Duration::from_millis(cur.read_u8()? as u64),
            press_light_step: cur.read_u8()?,
            keyboard_jitters_elimination_time: cur.read_u8()?,
        })
    }

    pub(crate) fn build(&self) -> [u8; 65] {
        let mut cur = str_to_cur(WRITE_BUFFER);
        cur.write_u8(self.key_1 as u8).unwrap();
        cur.write_u8(self.key_2 as u8).unwrap();
        cur.write_u8(self.key_3 as u8).unwrap();
        cur.write_u8(self.key_4 as u8).unwrap();
        cur.write_u8(self.key_5 as u8).unwrap();
        cur.write_u8(self.led_color_l.r).unwrap();
        cur.write_u8(self.led_color_l.g).unwrap();
        cur.write_u8(self.led_color_l.b).unwrap();
        cur.write_u8(self.led_color_r.r).unwrap();
        cur.write_u8(self.led_color_r.g).unwrap();
        cur.write_u8(self.led_color_r.b).unwrap();
        cur.write_u8(self.lighting_mode as u8).unwrap();
        cur.write_u8(self.maximum_brightness).unwrap();
        cur.write_u8(self.minimum_brightness).unwrap();
        // 除以十,要先除再转换成u8不然会溢出
        cur.write_u8((self.breath_maximum_light_duration.as_millis() / 10) as u8)
            .unwrap();
        cur.write_u8((self.breath_minimum_light_duration.as_millis() / 10) as u8)
            .unwrap();
        cur.write_u8(self.breath_interval.as_millis() as u8)
            .unwrap();
        cur.write_u8(self.press_light_maximum_brightness).unwrap();
        cur.write_u8(self.press_light_minimum_brightness).unwrap();
        cur.write_u8(self.press_light_duration.as_millis() as u8)
            .unwrap();
        cur.write_u8((self.fade_light_switching_speed.as_millis() / 10) as u8)
            .unwrap();
        cur.write_u8(self.speed_press_color_l.r).unwrap();
        cur.write_u8(self.speed_press_color_l.g).unwrap();
        cur.write_u8(self.speed_press_color_l.b).unwrap();
        cur.write_u8(self.speed_press_color_r.r).unwrap();
        cur.write_u8(self.speed_press_color_r.g).unwrap();
        cur.write_u8(self.speed_press_color_r.b).unwrap();
        cur.write_u8(self.color_switching_speed.as_millis() as u8)
            .unwrap();
        cur.write_u8(self.press_light_step).unwrap();
        cur.write_u8(self.keyboard_jitters_elimination_time)
            .unwrap();
        cur.into_inner()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_1: KeyCode::KEY_Z,
            key_2: KeyCode::KEY_X,
            key_3: KeyCode::KEY_ESC,
            key_4: KeyCode::KEY_F2,
            key_5: KeyCode::KEY_GRAVE,
            led_color_l: RGB {
                r: 204,
                g: 255,
                b: 229
            },
            led_color_r: RGB {
                r: 255,
                g: 153,
                b: 204
            },
            lighting_mode: LighingMode::Solid,
            maximum_brightness: 255,
            minimum_brightness: 5,
            breath_maximum_light_duration: Duration::from_millis(1200),
            breath_minimum_light_duration: Duration::from_millis(40),
            breath_interval: Duration::from_millis(8),
            press_light_maximum_brightness: 255,
            press_light_minimum_brightness: 0,
            press_light_duration: Duration::from_millis(5),
            fade_light_switching_speed: Duration::from_millis(1200),
            speed_press_color_l: RGB {
                r: 204,
                g: 255,
                b: 229
            },
            speed_press_color_r: RGB {
                r: 255,
                g: 153,
                b: 204
            },
            color_switching_speed: Duration::from_millis(2),
            press_light_step: 50,
            keyboard_jitters_elimination_time: 12,
        }
    }
}
use std::fmt::Debug;

use crate::{Config, str_to_cur};
use anyhow::Result;
use hidapi::HidDevice;
use log::*;
use pretty_hex::*;

const GET_DEVICE_NAME: &str = "get_device_name";
const GET_FIRMWARE_VERSION: &str = "get_firmware_version";
const GET_CONFIG: &str = "get_config";
const FLUSH_CONFIG: &str = "flush_config";
const PING: &str = "ping";

pub struct Meowpad {
    device: HidDevice,
    config: Option<Config>,
}

impl Debug for Meowpad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Meowpad")
    }
}


impl Meowpad {
    pub fn new(device: HidDevice) -> Meowpad {
        Meowpad {
            device,
            config: None,
        }
    }

    pub fn config(&self) -> Config {
        self.config
            .expect("未获取配置，请先使用load_config获取当前配置")
    }

    pub fn map_config<F>(&mut self, f: F)
    where
        F: FnMut(&mut Config),
    {
        self.config.iter_mut().for_each(f);
    }

    pub fn load_config(&mut self) -> Result<()> {
        let cur = str_to_cur(GET_CONFIG);
        self.write(cur.get_ref())?;
        let (res, size) = self.read()?;
        self.config = Some(Config::parse(&res[..size])?);
        Ok(())
    }

    pub fn ping(&self) -> Result<bool> {
        let cur = str_to_cur(PING);
        self.write(cur.get_ref())?;
        let (res, _) = self.read_timeout(1000)?;
        Ok(String::from_utf8_lossy(&res[..4]) == "pong")
    }

    pub fn get_device_name(&self) -> Result<String> {
        let cur = str_to_cur(GET_DEVICE_NAME);
        self.write(cur.get_ref())?;
        let (res, size) = self.read()?; // 读取
        Ok(String::from_utf8_lossy(&res[..size])
            .trim_end_matches('\0')
            .to_owned())
    }

    pub fn get_firmware_version(&self) -> Result<String> {
        let cur = str_to_cur(GET_FIRMWARE_VERSION);
        self.write(cur.get_ref())?;
        let (res, size) = self.read()?; // 读取
        Ok(String::from_utf8_lossy(&res[..size])
            .trim_end_matches('\0')
            .to_owned())
    }

    pub fn write_config(&self) -> Result<()> {
        let config = self.config();
        self.write(&config.build())?;
        info!("写入配置：{:?}", config);
        self.read()?;
        self.flush()?;
        Ok(())
    }

    pub fn reset_config(&mut self) -> Result<()> {
        let config = Config::default();
        self.write(&config.build())?;
        self.read()?;
        self.flush()?;
        self.config = Some(config);
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        let cur = str_to_cur(FLUSH_CONFIG);
        self.write(cur.get_ref())?;
        Ok(())
    }

    fn write(&self, data: &[u8]) -> Result<usize> {
        debug!("发送：{:?}", data[1..].hex_dump());
        Ok(self.device.write(data)?)
    }

    fn read_timeout(&self, timeout: i32) -> Result<([u8; 128], usize)> {
        let mut buf = [0u8; 128];
        let size = self.device.read_timeout(&mut buf[..], timeout)?;
        debug!("返回：{:?}", buf[..size].hex_dump());
        Ok((buf, size))
    }

    fn read(&self) -> Result<([u8; 128], usize)> {
        let mut buf = [0u8; 128];
        let size = self.device.read(&mut buf[..])?;

        debug!("返回：{:?}", buf[..size].hex_dump());
        Ok((buf, size))
    }
}

#[cfg(test)]
mod hid_tests {
    use crate::*;
    use pretty_hex::*;

    #[test]
    fn config() {
        let config = Config::parse(&[
            0x1D, 0x1B, 0x29, 0x3B, 0x35, 204, 255, 229, 255, 153, 204, 3, 255, 5, 120, 4, 8, 255,
            0, 5, 120, 204, 255, 229, 255, 153, 204, 2, 50, 12,
        ])
        .unwrap();
        println!("{:#?}", config);
        assert!(config.key_1 == KeyCode::KEY_Z);
        assert!(config.key_2 == KeyCode::KEY_X);
        assert!(config.lighting_mode == LighingMode::RainbowBreath);
        assert!(config.led_color_l == RGB::new(204, 255, 229));
        println!("{:?}", config.build().hex_dump());
        let config2 = Config::parse(&config.build()[3..]).unwrap();
        println!("{:#?}", config2);
        assert!(config == config2);
        assert!(config.build().starts_with(&[
            0x0, 0x57, 0x43, 0x1D, 0x1B, 0x29, 0x3B, 0x35, 204, 255, 229, 255, 153, 204, 3, 255, 5,
            120, 4, 8, 255, 0, 5, 120, 204, 255, 229, 255, 153, 204, 2, 50, 12
        ]));
    }
}

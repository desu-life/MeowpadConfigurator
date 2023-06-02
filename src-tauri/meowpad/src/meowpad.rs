use crate::{
    cbor::CONFIG,
    packet::{Packet, PacketID},
};
use anyhow::{anyhow, Result};
use byteorder::{BigEndian, ReadBytesExt};
use hidapi_rusb::HidDevice;
use log::*;
use num::FromPrimitive;
use pretty_hex::*;
use rand::RngCore;
use std::{fmt::Debug, fs, io::Cursor, thread, time::Duration};

type Config = CONFIG;

pub struct DebugMeowpad<'a>(&'a HidDevice, [u8; 64]);

impl<'a> DebugMeowpad<'a> {
    pub fn next(&mut self) -> Option<&[u8]> {
        let size = self.0.read_timeout(&mut self.1, 5).ok()?;
        Some(&self.1[..size])
    }
}

pub struct Meowpad {
    pub config: Option<Config>,
    pub device_name: Option<String>,
    pub firmware_version: Option<String>,
    pub is_hs: bool,
    device: HidDevice,
    key: Vec<u8>,
}

impl Debug for Meowpad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Meowpad {{ config: {:?} }}", self.config))
    }
}

impl Meowpad {
    pub fn new(device: HidDevice, is_hs: bool, path: impl AsRef<std::path::Path>) -> Meowpad {
        let path = path.as_ref();
        let mut data = match fs::read(path) {
            Ok(data) => {
                if data.len() != 64 || !data.starts_with(&[0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF]) {
                    warn!("错误的密钥，正在重置");
                    fs::remove_file(path).expect("错误的密钥 / 重置密钥失败");
                    return Self::new(device, is_hs, path);
                }
                data
            }
            Err(_) => {
                let mut data = vec![0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF];
                let mut tmp = [0u8; 58];
                rand::thread_rng().fill_bytes(&mut tmp);
                data.append(&mut tmp.to_vec());
                fs::write(path, &data).expect("初始化数据失败");
                data
            }
        };

        let mut key = vec![0];
        key.append(&mut data);
        device.set_blocking_mode(true).unwrap();
        Meowpad {
            key,
            device,
            is_hs,
            config: None,
            device_name: None,
            firmware_version: None,
        }
    }

    pub fn default_config(&self) -> Config {
        Config::default(self.is_hs)
    }

    pub fn config(&self) -> Config {
        self.config
            .expect("未获取配置，请先使用load_config获取当前配置")
    }

    pub fn load_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetConfig, []))?;
        let packet = self.read()?;
        self.config = Some(Config::from_cbor(packet.data)?);
        Ok(())
    }

    pub fn ping(&self) -> Result<bool> {
        self.write(Packet::new(PacketID::Ping, []))?;
        let packet = self.read_timeout(1000)?;
        if packet.id == PacketID::Ping {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_device_name(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetDeviceName, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            self.device_name = Some(String::from_utf8(packet.data)?);
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(anyhow!("Unexcepted Response"))
        }
    }

    pub fn get_firmware_version(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetFirmwareVersion, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            self.firmware_version = Some(String::from_utf8(packet.data)?);
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(anyhow!("Unexcepted Response"))
        }
    }

    pub fn erase_firmware(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::EraseFirmware, []))?;
        Ok(())
    }

    pub fn debug_mode(&mut self) -> Result<DebugMeowpad> {
        self.write(Packet::new(PacketID::DebugMode, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            Ok(DebugMeowpad(&self.device, [0u8; 64]))
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(anyhow!("Unexcepted Response"))
        }
    }

    /// 返回值第一个u8是触发推荐值，第二个是释放推荐值，第三个是死区推荐值
    pub fn get_auto_config(&mut self) -> Result<(u8, u8, u8)> {
        assert!(self.is_hs, "调用错误，此方法为hs版专属");
        self.write(Packet::new(PacketID::AutoConfig, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            Ok((packet.data[0], packet.data[1], packet.data[2]))
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(anyhow!("Unexcepted Response"))
        }
    }

    pub fn write_config(&self) -> Result<()> {
        let config = self.config();
        info!("写入配置：{:?}", config);
        self.write(Packet::new(PacketID::WriteConfig, config.to_cbor()))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(anyhow!("在写入配置时出错"))
        }
    }

    pub fn reset_config(&mut self) -> Result<()> {
        self.config = Some(Config::default(self.is_hs));
        self.write_config()?;
        Ok(())
    }

    pub fn calibration_key(&self) -> Result<()> {
        assert!(self.is_hs, "调用错误，此方法为hs版专属");
        self.write(Packet::new(PacketID::CalibrationKey, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(anyhow!("在校准轴体时出错"))
        }
    }

    pub fn get_calibration_key_result(&self, timeout: i32) -> Result<()> {
        assert!(self.is_hs, "调用错误，此方法为hs版专属");
        let packet = self.read_timeout(timeout)?; // 读取
        if packet.id == PacketID::Ok {
            Ok(())
        } else if packet.id == PacketID::Null {
            Err(anyhow!("校准超时"))
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(anyhow!("在校准轴体时出错"))
        }
    }

    pub fn flush(&self) -> Result<()> {
        self.write(Packet::new(PacketID::FlushConfig, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(anyhow!("在刷新配置时出错"))
        }
    }

    fn write_head(&self) -> Result<()> {
        debug!("发送头包：{:?}", self.key.hex_dump());
        self.device.write(&self.key)?;
        thread::sleep(Duration::from_millis(50));
        let packet = self.read()?;
        if packet.id == PacketID::Ok {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            warn!("密钥不正确，请重新插拔meowpad以重置");
            Err(anyhow!("密钥不正确，请重新插拔meowpad以重置"))
        }
    }

    fn write(&self, packet: Packet) -> Result<()> {
        debug!("发送：{:?}", packet);
        debug!("总数据大小：{}", packet.data.len());
        for v in packet.build_packets() {
            self.write_head()?;
            debug!("raw：{:?}", v.hex_dump());
            self.device.write(&v)?;
            thread::sleep(Duration::from_millis(50));
        }
        Ok(())
    }

    fn read_timeout(&self, timeout: i32) -> Result<Packet> {
        let mut buf = Cursor::new([0u8; 64]);
        self.device.read_timeout(buf.get_mut(), timeout)?;
        let packet_id = PacketID::from_u8(buf.read_u8()?).ok_or(anyhow!("解析数据包ID时报错"))?;
        let packet_len = buf.read_u16::<BigEndian>()? as usize;
        let mut data = Vec::with_capacity(packet_len);
        let mut read_bytes = 0;
        let mut packet_num = 1;
        loop {
            if read_bytes < packet_len {
                match buf.read_u8() {
                    Ok(b) => {
                        read_bytes += 1;
                        data.push(b)
                    }
                    Err(_) => {
                        // cur已经遍历结束
                        // reset buffer
                        unsafe { std::ptr::write_volatile(buf.get_mut(), [0u8; 64]) }
                        buf.set_position(0);
                        self.write(Packet::new(packet_id, [packet_num]))?;
                        self.device.read_timeout(buf.get_mut(), timeout)?;
                        packet_num += 1;
                    }
                }
            } else {
                break;
            }
        }

        // 处理完毕
        debug!("返回包: {:?}\n内容: {:?}", packet_id, data.hex_dump());
        Ok(Packet::new(packet_id, data))
    }

    pub fn read(&self) -> Result<Packet> {
        let mut buf = Cursor::new([0u8; 64]);
        self.device.read(buf.get_mut())?;
        let packet_id = PacketID::from_u8(buf.read_u8()?).ok_or(anyhow!("解析数据包ID时报错"))?;
        let packet_len = buf.read_u16::<BigEndian>()? as usize;
        let mut data = Vec::with_capacity(packet_len);
        let mut read_bytes = 0;
        let mut packet_num = 1;
        loop {
            if read_bytes < packet_len {
                match buf.read_u8() {
                    Ok(b) => {
                        read_bytes += 1;
                        data.push(b)
                    }
                    Err(_) => {
                        // cur已经遍历结束
                        // reset buffer
                        unsafe { std::ptr::write_volatile(buf.get_mut(), [0u8; 64]) }
                        buf.set_position(0);
                        self.write(Packet::new(packet_id, [packet_num]))?;
                        self.device.read(buf.get_mut())?;
                        packet_num += 1;
                    }
                }
            } else {
                break;
            }
        }

        // 处理完毕
        debug!("返回包: {:?}\n内容: {:?}", packet_id, data.hex_dump());
        Ok(Packet::new(packet_id, data))
    }
}

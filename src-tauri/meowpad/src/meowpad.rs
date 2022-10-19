use crate::{
    cbor::CONFIG,
    packet::{Packet, PacketID},
};
use anyhow::{anyhow, Result};
use byteorder::{BigEndian, ReadBytesExt};
use hidapi::HidDevice;
use log::*;
use num::FromPrimitive;
use pretty_hex::*;
use rand::RngCore;
use std::{fmt::Debug, fs, io::Cursor, thread, time::Duration};

type Config = CONFIG;

pub struct Meowpad {
    device: HidDevice,
    config: Option<Config>,
    key: Vec<u8>,
}

impl Debug for Meowpad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Meowpad {{ config: {:?} }}", self.config))
    }
}

impl Meowpad {
    pub fn new(device: HidDevice) -> Meowpad {
        let mut data = match fs::read(".meowkey") {
            Ok(data) => {
                if data.len() != 64 || !data.starts_with(&[0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF]) {
                    warn!("错误的密钥，正在重置");
                    fs::remove_file(".meowkey").expect("错误的密钥 / 重置密钥失败");
                    return Self::new(device);
                }
                data
            }
            Err(_) => {
                let mut data = vec![0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF];
                let mut tmp = [0u8; 58];
                rand::thread_rng().fill_bytes(&mut tmp);
                data.append(&mut tmp.to_vec());
                fs::write(".meowkey", &data).unwrap();
                data
            }
        };

        let mut key = vec![0];
        key.append(&mut data);
        device.set_blocking_mode(true).unwrap();
        Meowpad {
            key,
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

    pub fn get_device_name(&self) -> Result<String> {
        self.write(Packet::new(PacketID::GetDeviceName, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            Ok(String::from_utf8(packet.data)?)
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(anyhow!("Unexcepted Response"))
        }
    }

    pub fn get_firmware_version(&self) -> Result<String> {
        self.write(Packet::new(PacketID::GetFirmwareVersion, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            Ok(String::from_utf8(packet.data)?)
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
        self.config = Some(Config::default());
        self.write_config()?;
        Ok(())
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
        let packet = self.read_timeout(1000)?;
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

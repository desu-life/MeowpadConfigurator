use crate::{
    cbor::CborConfig,
    packet::{Packet, PacketID},
    error::*,
};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use hidapi_rusb::HidDevice;
use log::*;
use num::FromPrimitive;
use pretty_hex::*;
use std::{fmt::Debug, io::{Cursor, Write}, thread, time::Duration};
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

pub struct Meowpad {
    pub config: Option<CborConfig>,
    pub device_name: Option<String>,
    pub firmware_version: Option<String>,
    device: HidDevice,
}

impl Debug for Meowpad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Meowpad {{ config: {:?} }}", self.config))
    }
}

impl Meowpad {
    pub fn new(device: HidDevice) -> Meowpad {

        device.set_blocking_mode(true).unwrap();
        Meowpad {
            device,
            config: None,
            device_name: None,
            firmware_version: None,
        }
    }

    pub fn default_config(&self) -> CborConfig {
        CborConfig::default()
    }

    pub fn config(&self) -> CborConfig {
        self.config
            .expect("未获取配置，请先使用load_config获取当前配置")
    }

    pub fn load_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetConfig, []))?;
        let packet = self.read()?;
        self.config = Some(CborConfig::from_cbor(packet.data)?);
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
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub fn debug(&mut self) -> Result<[KeyRTStatus; 4]> {
        self.write(Packet::new(PacketID::Debug, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            let mut keys: [KeyRTStatus; 4] = Default::default();
            let mut cur = Cursor::new(packet.data);
            for key in keys.iter_mut() {
                key.adc_value = cur.read_u16::<BigEndian>()?;
                key.linear_value = cur.read_u16::<BigEndian>()?;
                key.press_percentage = cur.read_u16::<BigEndian>()? as u8;
                key.key_state = KeyState::from_u16(cur.read_u16::<BigEndian>()?).ok_or(Error::InvalidPacket)?;
            }
            Ok(keys)
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
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
            Err(Error::UnexceptedResponse(packet))
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
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub fn reset_config(&mut self) -> Result<()> {
        self.config = Some(self.default_config());
        self.write_config()?;
        Ok(())
    }

    pub fn reset_middle_point(&self) -> Result<()> {
        self.write(Packet::new(PacketID::SetMiddlePoint, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::Other("在校准轴体时出错"))
        }
    }

    pub fn calibration_key(&self) -> Result<()> {
        self.write(Packet::new(PacketID::CalibrationKey, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::Other("在校准轴体时出错"))
        }
    }

    pub fn erase_firmware(&self)  -> Result<()> {
        self.write(Packet::new(PacketID::EraseFirmware, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::Other("数据交互时出错"))
        }
    }

    fn write(&self, packet: Packet) -> Result<()> {
        debug!("发送：{:?}", packet);
        debug!("总数据大小：{}", packet.data.len());
        for v in packet.build_packets() {
            debug!("raw：{:?}", v.hex_dump());
            self.device.write(&v)?;
            thread::sleep(Duration::from_millis(50));
        }
        Ok(())
    }

    fn read_timeout(&self, timeout: i32) -> Result<Packet> {
        let mut buf = Cursor::new([0u8; 64]);
        self.device.read_timeout(buf.get_mut(), timeout)?;
        // debug!("收到数据包: {:?}", buf.get_ref().hex_dump());
        let packet_id = PacketID::from_u8(buf.read_u8()?).ok_or(Error::InvalidPacket)?;
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
        debug!("收到数据包: {:?}", buf.get_ref().hex_dump());
        let packet_id = PacketID::from_u8(buf.read_u8()?).ok_or(Error::InvalidPacket)?;
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

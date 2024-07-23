use crate::{
    cbor, packet_id::PacketID
};
use meowpad::{Packet, error::Error, Result, models::*, Device};
use byteorder::{BigEndian, ReadBytesExt};
use log::*;
use num::FromPrimitive;
use pretty_hex::*;
use std::{io::Cursor, thread, time::Duration};
use crate::cbor::CborConvertor;

pub struct Meowboard<D: Device> {
    pub key_config: Option<cbor::Device>,
    pub device_name: Option<String>,
    pub firmware_version: Option<String>,
    device: D,
}


impl<D: Device> Meowboard<D> {
    pub fn new(device: D) -> Meowboard<D> {
        Meowboard {
            device,
            key_config: None,
            device_name: None,
            firmware_version: None,
        }
    }

    pub fn ping(&self) -> Result<bool> {
        self.write(Packet::new(PacketID::Ping, []))?;
        let packet = self.read_timeout(1000)?;
        if packet.id == PacketID::Ping as u8 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_device_name(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetDeviceName, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            self.device_name = Some(String::from_utf8(packet.data)?);
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }
    
    pub fn get_firmware_version(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetFirmwareVersion, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            self.firmware_version = Some(String::from_utf8(packet.data)?);
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    /// (按键配置状态，灯光配置状态，按键校准状态，按键是否启用)
    pub fn get_status(&mut self) -> Result<DeviceStatus> {
        self.write(Packet::new(PacketID::GetStatus, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            Ok(DeviceStatus {
                key: packet.data[0] != 0,
                hall: packet.data[1] != 0,
                enabled: packet.data[2] != 0,
                light: None,
            })
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }


    pub fn get_debug_value_part(&mut self, index: u8) -> Result<[KeyRTStatus; 8]> {
        let p = Packet::new(PacketID::Debug, [index]);
        self.write(p)?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            let mut keys = [KeyRTStatus::default(); 8];
            let mut cur = Cursor::new(packet.data);
            for key in keys.iter_mut() {
                key.adc_value = cur.read_u16::<BigEndian>()?;
                key.linear_value = cur.read_u16::<BigEndian>()?;
                key.press_percentage = cur.read_u8()? as u8;
                key.key_state = KeyState::from_u8(cur.read_u8()?).ok_or(Error::InvalidPacket)?;
            }
            Ok(keys)
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub fn get_debug_value(&mut self) -> Result<[KeyRTStatus; 64]> {
        let mut keys = [KeyRTStatus::default(); 64];
        for i in 0..8 {
            let part = self.get_debug_value_part(i as u8)?;
            keys[i * 8..(i + 1) * 8].copy_from_slice(&part);
        }
        Ok(keys)
    }

    pub fn get_keystates(&mut self) -> Result<[KeyState; 64]> {
        let mut index = 0;
        let mut keys = [KeyState::default(); 64];
        self.write(Packet::new(PacketID::DebugKeyState, [0]))?;
        let data = self.read()?.data;
        for i in 0..32 {
            keys[index] = KeyState::from_u8(data[i]).ok_or(Error::InvalidPacket)?;
            index += 1;
        }
        self.write(Packet::new(PacketID::DebugKeyState, [1]))?;
        let data = self.read()?.data;
        for i in 0..32 {
            keys[index] = KeyState::from_u8(data[i]).ok_or(Error::InvalidPacket)?;
            index += 1;
        }
        Ok(keys)
    }
    
    pub fn get_key_calibrate_status(&mut self) -> Result<[bool; 64]> {
        let mut index = 0;
        let mut keys = [false; 64];
        self.write(Packet::new(PacketID::CalibrateKeyStatus, [0]))?;
        let data = self.read()?.data;
        for i in 0..32 {
            keys[index] = data[i] != 0;
            index += 1;
        }
        self.write(Packet::new(PacketID::CalibrateKeyStatus, [1]))?;
        let data = self.read()?.data;
        for i in 0..32 {
            keys[index] = data[i] != 0;
            index += 1;
        }
        Ok(keys)
    }

    pub fn get_keyvalues(&mut self) -> Result<[u16; 64]> {
        let mut index = 0;
        let mut keys = [0u16; 64];
        self.write(Packet::new(PacketID::DebugValue, [0]))?;
        let mut cur = Cursor::new(self.read()?.data);
        for _ in 0..30 {
            keys[index] = cur.read_u16::<BigEndian>()?;
            index += 1;
        }
        self.write(Packet::new(PacketID::DebugValue, [1]))?;
        let mut cur = Cursor::new(self.read()?.data);
        for _ in 0..30 {
            keys[index] = cur.read_u16::<BigEndian>()?;
            index += 1;
        }
        self.write(Packet::new(PacketID::DebugValue, [2]))?;
        let mut cur = Cursor::new(self.read()?.data);
        for _ in 0..4 {
            keys[index] = cur.read_u16::<BigEndian>()?;
            index += 1;
        }
        Ok(keys)
    }

    pub fn get_hall_config_part(&mut self, index: u8) -> Result<[KeyHallConfig; 8]> {
        self.write(Packet::new(PacketID::GetHallConfig, [index]))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            let mut keys = [KeyHallConfig::default(); 8];
            let mut cur = Cursor::new(packet.data);
            for key in keys.iter_mut() {
                key.adc_max = cur.read_u16::<BigEndian>()?;
                key.adc_min = cur.read_u16::<BigEndian>()?;
                key.hall_middle = cur.read_u16::<BigEndian>()?;
            }
            Ok(keys)
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub fn get_hall_config(&mut self) -> Result<[KeyHallConfig; 64]> {
        let mut keys = [KeyHallConfig::default(); 64];
        for i in 0..8 {
            let part = self.get_hall_config_part(i as u8)?;
            keys[i * 8..(i + 1) * 8].copy_from_slice(&part);
        }
        Ok(keys)
    }


    pub fn load_key_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetKeyConfig, []))?;
        let packet = self.read()?;
        self.key_config = Some(cbor::Device::from_cbor(packet.data)?);
        Ok(())
    }


    pub fn set_key_config(&self) -> Result<()> {
        let config = self.key_config.ok_or(Error::EmptyConfig)?;
        // debug!("写入键盘配置：{:?}", config);
        self.write(Packet::new(PacketID::SetKeyConfig, config.to_cbor()))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub fn save_key_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::SaveKeyConfig, []))?;
        let packet = self.read()?;
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub fn clear_key_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::ClearKeyConfig, []))?;
        let packet = self.read()?;
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub fn clear_hall_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::ClearHallConfig, []))?;
        let packet = self.read()?;
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub fn reset_middle_point(&self) -> Result<()> {
        self.write(Packet::new(PacketID::SetMiddlePoint, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::Other("中点设置出错"))
        }
    }

    pub fn calibration_key(&self, key_indexs: &[u8]) -> Result<()> {
        self.write(Packet::new(PacketID::CalibrationKey, key_indexs))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::Other("校准轴体出错"))
        }
    }

    pub fn erase_firmware(&self)  -> Result<()> {
        self.write(Packet::new(PacketID::EraseFirmware, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::Other("数据交互时出错"))
        }
    }

    pub fn reset_device(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::Reset, []))?;
        let packet = self.read()?;
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    fn write(&self, packet: Packet) -> Result<()> {
        debug!("发送：{:?}", packet);
        debug!("总数据大小：{}", packet.data.len());
        for v in packet.build_packets() {
            // debug!("raw：{:?}", v.hex_dump());
            self.device.write(&v)?;
        }
        Ok(())
    }

    fn read_timeout(&self, timeout: i32) -> Result<Packet> {
        let mut buf = Cursor::new([0u8; 64]);
        self.device.read_timeout(buf.get_mut(), timeout)?;
        debug!("收到数据包: {:?}", buf.get_ref().hex_dump());
        let packet_id = PacketID::from_u8(buf.read_u8()?).ok_or(Error::InvalidPacket)?;
        let packet_len = buf.read_u16::<BigEndian>()? as usize;
        let mut data = Vec::with_capacity(packet_len);
        let mut read_bytes = 0;
        // let mut packet_num = 1;
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
                        // packet_num += 1;
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

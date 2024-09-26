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

pub struct Meowpad<D: Device> {
    pub key_config: Option<cbor::Keyboard>,
    pub light_config: Option<cbor::Light>,
    pub device_name: Option<String>,
    pub firmware_version: Option<String>,
    device: D,
}


impl<D: Device> Meowpad<D> {
    pub fn new(device: D) -> Meowpad<D> {
        Meowpad {
            device,
            key_config: None,
            light_config: None,
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
                light: Some(packet.data[1] != 0),
                hall: packet.data[2] != 0,
                enabled: packet.data[3] != 0,
            })
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub fn get_debug_value(&mut self) -> Result<[KeyRTStatus; 4]> {
        self.write(Packet::new(PacketID::Debug, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            let mut keys: [KeyRTStatus; 4] = Default::default();
            let mut cur = Cursor::new(packet.data);
            for key in keys.iter_mut() {
                key.adc_value = cur.read_u16::<BigEndian>()?;
                key.linear_value = cur.read_u16::<BigEndian>()?;
                key.press_percentage = cur.read_u16::<BigEndian>()? as f32;
                key.key_state = KeyState::from_u16(cur.read_u16::<BigEndian>()?).ok_or(Error::InvalidPacket)?;
            }
            let rem = cur.remaining_slice();
            if !rem.is_empty() {
                println!("{:?}", cur.remaining_slice());
            }
            Ok(keys)
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub fn get_hall_config(&mut self) -> Result<[KeyHallConfig; 4]> {
        self.write(Packet::new(PacketID::GetHallConfig, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
            let mut keys: [KeyHallConfig; 4] = Default::default();
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


    pub fn load_key_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetKeyConfig, []))?;
        let packet = self.read()?;
        self.key_config = Some(cbor::Keyboard::from_cbor(packet.data)?);
        Ok(())
    }
    
    pub fn load_light_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetLightConfig, []))?;
        let packet = self.read()?;
        self.light_config = Some(cbor::Light::from_cbor(packet.data)?);
        Ok(())
    }

    pub fn set_key_config(&self) -> Result<()> {
        let config = self.key_config.ok_or(Error::EmptyConfig)?;
        debug!("写入键盘配置：{:?}", config);
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

    pub fn set_light_config(&self) -> Result<()> {
        let config = self.light_config.ok_or(Error::EmptyConfig)?;
        debug!("写入灯光配置：{:?}", config);
        self.write(Packet::new(PacketID::SetLightConfig, config.to_cbor()))?;
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
    
    pub fn save_light_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::SaveLightConfig, []))?;
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
    
    pub fn clear_light_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::ClearLightConfig, []))?;
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
            Err(Error::Other("在校准轴体时出错"))
        }
    }

    pub fn calibration_key(&self) -> Result<()> {
        self.write(Packet::new(PacketID::CalibrationKey, []))?;
        let packet = self.read()?; // 读取
        if packet.id == PacketID::Ok as u8 {
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
            debug!("raw：{:?}", v.hex_dump());
            self.device.write(&v)?;
            thread::sleep(Duration::from_millis(5));
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

use crate::{
    cbor, config, packet_id::PacketID
};
use meowpad::{Packet, error::Error, Result, models::*, Device};
use byteorder::{BigEndian, ReadBytesExt};
use log::*;
use num::FromPrimitive;
use pretty_hex::*;
use std::{io::Cursor, thread, time::Duration};
use crate::cbor::CborConvertor;

pub struct Meowboard<D: Device> {
    pub device_name: Option<String>,
    pub firmware_version: Option<String>,
    pub is_old_firmware: bool,
    pub device: D,
}

impl<D: Device> Meowboard<D> {
    pub fn new(device: D) -> Meowboard<D> {
        Meowboard {
            device,
            device_name: None,
            firmware_version: None,
            is_old_firmware: false,
        }
    }

    pub async fn ping(&self) -> Result<bool> {
        self.write(Packet::new(PacketID::Ping, [])).await?;
        let packet = self.read_timeout(1000).await?;
        log::info!("Ping response: id={:?}, data={:?}", packet.id, packet.data.hex_dump());
        if packet.id == PacketID::Ping as u8 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn get_device_name(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetDeviceName, [])).await?;
        let packet = self.read().await?; // 读取
        if packet.id == PacketID::Ok as u8 {
            self.device_name = Some(String::from_utf8(packet.data)?);
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }
    
    pub async fn get_firmware_version(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetFirmwareVersion, [])).await?;
        let packet = self.read().await?; // 读取
        if packet.id == PacketID::Ok as u8 {
            let version = String::from_utf8(packet.data)?;
            self.is_old_firmware = version == "0.1.2";
            self.firmware_version = Some(version);
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    /// (按键配置状态，灯光配置状态，按键校准状态，按键是否启用)
    pub async fn get_status(&mut self) -> Result<DeviceStatus> {
        self.write(Packet::new(PacketID::GetStatus, [])).await?;
        let packet = self.read().await?; // 读取
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


    pub async fn get_debug_value_part(&mut self, index: u8) -> Result<[KeyRTStatus; 8]> {
        let p = Packet::new(PacketID::Debug, [index]);
        self.write_no_delay(p).await?;
        let packet = self.read().await?; // 读取
        if packet.id == PacketID::Ok as u8 {
            let mut keys = [KeyRTStatus::default(); 8];
            let mut cur = Cursor::new(packet.data);
            for key in keys.iter_mut() {
                key.adc_value = cur.read_u16::<BigEndian>()?;
                key.linear_value = cur.read_u16::<BigEndian>()?;
                key.press_percentage = cur.read_u8()? as f32 / 2f32;
                key.key_state = KeyState::from_u8(cur.read_u8()?).ok_or(Error::InvalidPacket)?;
            }
            Ok(keys)
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub async fn get_debug_value(&mut self) -> Result<[KeyRTStatus; 64]> {
        let mut keys = [KeyRTStatus::default(); 64];
        for i in 0..8 {
            let part = self.get_debug_value_part(i as u8).await?;
            keys[i * 8..(i + 1) * 8].copy_from_slice(&part);
        }
        Ok(keys)
    }

    pub async fn get_keystates(&mut self) -> Result<[KeyState; 64]> {
        let mut index = 0;
        let mut keys = [KeyState::default(); 64];
        self.write(Packet::new(PacketID::DebugKeyState, [0])).await?;
        let data = self.read().await?.data;
        for i in 0..32 {
            
            keys[index] = KeyState::from_u8(*data.get(i).ok_or(Error::InvalidPacket)?).ok_or(Error::InvalidPacket)?;
            index += 1;
        }
        self.write(Packet::new(PacketID::DebugKeyState, [1])).await?;
        let data = self.read().await?.data;
        for i in 0..32 {
            keys[index] = KeyState::from_u8(*data.get(i).ok_or(Error::InvalidPacket)?).ok_or(Error::InvalidPacket)?;
            index += 1;
        }
        Ok(keys)
    }
    
    pub async fn get_key_calibrate_status(&mut self) -> Result<[bool; 64]> {
        let mut index = 0;
        let mut keys = [false; 64];
        self.write(Packet::new(PacketID::CalibrateKeyStatus, [0])).await?;
        let data = self.read().await?.data;
        for i in 0..32 {
            keys[index] = *data.get(i).ok_or(Error::InvalidPacket)? != 0;
            index += 1;
        }
        self.write(Packet::new(PacketID::CalibrateKeyStatus, [1])).await?;
        let data = self.read().await?.data;
        for i in 0..32 {
            keys[index] = *data.get(i).ok_or(Error::InvalidPacket)? != 0;
            index += 1;
        }
        Ok(keys)
    }

    pub async fn get_keyvalues(&mut self) -> Result<[u16; 64]> {
        let mut index = 0;
        let mut keys = [0u16; 64];
        self.write(Packet::new(PacketID::DebugValue, [0])).await?;
        let mut cur = Cursor::new(self.read().await?.data);
        for _ in 0..30 {
            keys[index] = cur.read_u16::<BigEndian>()?;
            index += 1;
        }
        self.write(Packet::new(PacketID::DebugValue, [1])).await?;
        let mut cur = Cursor::new(self.read().await?.data);
        for _ in 0..30 {
            keys[index] = cur.read_u16::<BigEndian>()?;
            index += 1;
        }
        self.write(Packet::new(PacketID::DebugValue, [2])).await?;
        let mut cur = Cursor::new(self.read().await?.data);
        for _ in 0..4 {
            keys[index] = cur.read_u16::<BigEndian>()?;
            index += 1;
        }
        Ok(keys)
    }

    pub async fn get_hall_config_part(&mut self, index: u8) -> Result<[KeyHallConfig; 8]> {
        self.write(Packet::new(PacketID::GetHallConfig, [index])).await?;
        let packet = self.read().await?; // 读取
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

    pub async fn get_hall_config(&mut self) -> Result<[KeyHallConfig; 64]> {
        let mut keys = [KeyHallConfig::default(); 64];
        for i in 0..8 {
            let part = self.get_hall_config_part(i as u8).await?;
            keys[i * 8..(i + 1) * 8].copy_from_slice(&part);
        }
        Ok(keys)
    }


    pub async fn load_key_config(&mut self) -> Result<config::Device> {
        self.write(Packet::new(PacketID::GetKeyConfig, [])).await?;
        let packet = self.read().await?;
        if self.is_old_firmware {
            Ok(cbor::DeviceOld::from_cbor(packet.data)?.try_into()?)
        } else {
            Ok(cbor::Device::from_cbor(packet.data)?.try_into()?)
        }
    }


    pub async fn set_key_config(&self, config: config::Device) -> Result<()> {
        let config = if self.is_old_firmware {
            cbor::DeviceOld::from(config).to_cbor()
        } else {
            cbor::Device::from(config).to_cbor()
        };
        // debug!("写入键盘配置：{:?}", config);
        self.write(Packet::new(PacketID::SetKeyConfig, config)).await?;
        let packet = self.read().await?; // 读取
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub async fn save_key_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::SaveKeyConfig, [])).await?;
        let packet = self.read().await?;
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub async fn clear_key_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::ClearKeyConfig, [])).await?;
        let packet = self.read().await?;
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub async fn clear_hall_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::ClearHallConfig, [])).await?;
        let packet = self.read().await?;
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    pub async fn reset_middle_point(&self) -> Result<()> {
        self.write(Packet::new(PacketID::SetMiddlePoint, [])).await?;
        let packet = self.read().await?; // 读取
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::Other("中点设置出错"))
        }
    }

    pub async fn calibration_key(&self, key_indexs: &[u8]) -> Result<()> {
        self.write(Packet::new(PacketID::CalibrationKey, key_indexs)).await?;
        let packet = self.read().await?; // 读取
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::Other("校准轴体出错"))
        }
    }

    pub async fn erase_firmware(&self)  -> Result<()> {
        self.write(Packet::new(PacketID::EraseFirmware, [])).await?;
        let packet = self.read().await?; // 读取
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::Other("数据交互时出错"))
        }
    }

    pub async fn reset_device(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::Reset, [])).await?;
        let packet = self.read().await?;
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
        }
    }

    async fn write_no_delay(&self, packet: Packet) -> Result<()> {
        debug!("发送：{:?}", packet);
        debug!("总数据大小：{}", packet.data.len());
        for v in packet.build_packets() {
            // debug!("raw：{:?}", v.hex_dump());
            self.device.write(&v).await?;
        }
        Ok(())
    }

    async fn write(&self, packet: Packet) -> Result<()> {
        debug!("发送：{:?}", packet);
        debug!("总数据大小：{}", packet.data.len());
        for v in packet.build_packets() {
            // debug!("raw：{:?}", v.hex_dump());
            self.device.write(&v).await?;
            self.device.delay(5).await?;
        }
        Ok(())
    }

    async fn read_timeout(&self, timeout: i32) -> Result<Packet> {
        let mut buf = Cursor::new([0u8; 64]);
        let num = self.device.read_timeout(buf.get_mut(), timeout).await?;
        debug!("第一次读取: {} bytes, data: {:?}", num, buf.get_ref().hex_dump());
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
                        buf.get_mut().fill(0);
                        buf.set_position(0);
                        self.write(Packet::new(packet_id, [packet_num])).await?;
                        self.device.read_timeout(buf.get_mut(), timeout).await?;
                        debug!("再次读取: {} bytes, data: {:?}", num, buf.get_ref().hex_dump());
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

    pub async fn read(&self) -> Result<Packet> {
        let mut buf = Cursor::new([0u8; 64]);
        self.device.read(buf.get_mut()).await?;
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
                        buf.get_mut().fill(0);
                        buf.set_position(0);
                        self.write(Packet::new(packet_id, [packet_num])).await?;
                        self.device.read(buf.get_mut()).await?;
                        debug!("再次读取: {} bytes, data: {:?}", packet_num, buf.get_ref().hex_dump());
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

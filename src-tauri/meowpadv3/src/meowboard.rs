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

pub const HALL_KEY_NUMS: usize = 7;
pub const NORMAL_KEY_NUMS: usize = 1;
pub const TOTAL_KEYS: usize = HALL_KEY_NUMS + NORMAL_KEY_NUMS;
pub const MAX_SOCD_PAIRS: usize = 5;

pub struct MeowpadV3<D: Device> {
    pub key_config: Option<cbor::Device>,
    pub device_name: Option<String>,
    pub firmware_version: Option<String>,
    pub device: D,
}


impl<D: Device> MeowpadV3<D> {
    pub fn new(device: D) -> MeowpadV3<D> {
        MeowpadV3 {
            device,
            key_config: None,
            device_name: None,
            firmware_version: None,
        }
    }

    pub async fn ping(&self) -> Result<bool> {
        self.write(Packet::new(PacketID::Ping, [])).await?;
        let packet = self.read_timeout(1000).await?;
        if packet.id == PacketID::Ping as u8 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn toggle_keyboard(&self) -> Result<()> {
        self.write(Packet::new(PacketID::ToggleKeyboard, [])).await?;
        let packet = self.read().await?;
        if packet.id == PacketID::Ok as u8 {
            Ok(())
        } else {
            dbg!(packet.id);
            dbg!(packet.data.hex_dump());
            Err(Error::UnexceptedResponse(packet))
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
            self.firmware_version = Some(String::from_utf8(packet.data)?);
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


    pub async fn get_debug_value_part(&mut self, index: u8) -> Result<[KeyRTStatus; TOTAL_KEYS]> {
        let p = Packet::new(PacketID::Debug, [index]);
        self.write_no_delay(p).await?;
        let packet = self.read().await?; // 读取
        if packet.id == PacketID::Ok as u8 {
            let mut keys = [KeyRTStatus::default(); TOTAL_KEYS];
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

    pub async fn get_debug_value(&mut self) -> Result<[KeyRTStatus; TOTAL_KEYS]> {
        let part = self.get_debug_value_part(0).await?;
        Ok(part)
    }

    pub async fn get_keystates(&mut self) -> Result<[KeyState; TOTAL_KEYS]> {
        let mut index = 0;
        let mut keys = [KeyState::default(); TOTAL_KEYS];
        self.write(Packet::new(PacketID::DebugKeyState, [0])).await?;
        let data = self.read().await?.data;
        for i in 0..TOTAL_KEYS {
            keys[index] = KeyState::from_u8(*data.get(i).ok_or(Error::InvalidPacket)?).ok_or(Error::InvalidPacket)?;
            index += 1;
        }
        Ok(keys)
    }
    
    pub async fn get_key_calibrate_status(&mut self) -> Result<[bool; HALL_KEY_NUMS]> {
        let mut index = 0;
        let mut keys = [false; HALL_KEY_NUMS];
        self.write(Packet::new(PacketID::CalibrateKeyStatus, [0])).await?;
        let data = self.read().await?.data;
        for i in 0..HALL_KEY_NUMS {
            keys[index] = *data.get(i).ok_or(Error::InvalidPacket)? != 0;
            index += 1;
        }
        Ok(keys)
    }

    pub async fn get_keyvalues(&mut self) -> Result<[u16; HALL_KEY_NUMS]> {
        let mut index = 0;
        let mut keys = [0u16; HALL_KEY_NUMS];
        self.write(Packet::new(PacketID::DebugValue, [0])).await?;
        let mut cur = Cursor::new(self.read().await?.data);
        for _ in 0..HALL_KEY_NUMS {
            keys[index] = cur.read_u16::<BigEndian>()?;
            index += 1;
        }
        Ok(keys)
    }

    pub async fn get_hall_config_part(&mut self, index: u8) -> Result<[KeyHallConfig; HALL_KEY_NUMS]> {
        self.write(Packet::new(PacketID::GetHallConfig, [index])).await?;
        let packet = self.read().await?; // 读取
        if packet.id == PacketID::Ok as u8 {
            let mut keys = [KeyHallConfig::default(); HALL_KEY_NUMS];
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

    pub async fn get_hall_config(&mut self) -> Result<[KeyHallConfig; HALL_KEY_NUMS]> {
        let part = self.get_hall_config_part(0).await?;
        Ok(part)
    }


    pub async fn load_key_config(&mut self) -> Result<()> {
        self.write(Packet::new(PacketID::GetKeyConfig, [])).await?;
        let packet = self.read().await?;
        self.key_config = Some(cbor::Device::from_cbor(packet.data)?);
        Ok(())
    }


    pub async fn set_key_config(&self) -> Result<()> {
        let config = self.key_config.ok_or(Error::EmptyConfig)?;
        // debug!("写入键盘配置：{:?}", config);
        self.write(Packet::new(PacketID::SetKeyConfig, config.to_cbor())).await?;
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

    pub async fn calibration_key(&self, key_indexs: Option<&[u8]>) -> Result<()> {
        let key_indexs = key_indexs.map_or(Vec::new(), Into::into);
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
        self.device.read_timeout(buf.get_mut(), timeout).await?;
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
                        self.device.read_timeout(buf.get_mut(), timeout).await?;
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
                        unsafe { std::ptr::write_volatile(buf.get_mut(), [0u8; 64]) }
                        buf.set_position(0);
                        self.write(Packet::new(packet_id, [packet_num])).await?;
                        self.device.read(buf.get_mut()).await?;
                        debug!("收到数据包: {:?}", buf.get_ref().hex_dump());
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

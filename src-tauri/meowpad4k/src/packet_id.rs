use byteorder::{BigEndian, WriteBytesExt};
use num_derive::{FromPrimitive, ToPrimitive};
use serde::Deserialize;
use std::io::Cursor;
use std::io::Write;

#[derive(Deserialize, FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum PacketID {
    Null = 0,
    Ok   = 1,
    Bad  = 2,
    Ping = 3,

    // 弃用
    SetConfig = 5,
    GetConfig = 6,

    // 其他
    GetFirmwareVersion = 7,
    GetDeviceName      = 8,
    CalibrationKey     = 9,
    EraseFirmware      = 10,
    Debug              = 11,
    AutoConfig         = 12, // todo
    SetMiddlePoint     = 13,
    ToggleKeyboard     = 14,
    Reset              = 15,
    GetStatus          = 16,

    // 配置部分
    GetKeyConfig     = 100,
    GetLightConfig   = 101,
    SetKeyConfig     = 102,
    SetLightConfig   = 103,
    SaveKeyConfig    = 104,
    SaveLightConfig  = 105,
    ClearKeyConfig   = 106,
    ClearLightConfig = 107,
    GetHallConfig    = 108,
    ClearHallConfig  = 109,
}

impl std::fmt::Display for PacketID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl Into<u8> for PacketID {
    fn into(self) -> u8 {
        self as u8
    }
}
use thiserror::Error;

pub type IAPResult<T> = Result<T, IAPError>;

#[derive(Error, Debug)]
pub enum IAPError {
    #[error("device error: {0}")]
    DeviceError(#[from] meowpad::error::Error),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("invalid data, {0}")]
    InvalidData(#[from] scroll::Error),
    #[error("ack header error: expected 0x{expected:08X}, actual 0x{actual:08X}")]
    InvalidAckHeader {
        expected: u8,
        actual: u8,
    },
    #[error("crc error: address: 0x{address:08X}, expected 0x{expected:08X}, actual 0x{actual:08X}")]
    InvalidCRC {
        address: u32,
        expected: u32,
        actual: u32,
    },
    #[error("buffer full")]
    BufferFull
}
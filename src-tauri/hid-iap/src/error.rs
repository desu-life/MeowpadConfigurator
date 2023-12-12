use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("command failed {0:?}")]
    CommandFailed(crate::packet::IAPCommand),
    #[error("device_disconnected")]
    Disconnect(#[from] hidapi_rusb::HidError),
    #[error("no_firmware_data")]
    NoFirmwareData,
    #[error("{0}")]
    Other(&'static str),
}
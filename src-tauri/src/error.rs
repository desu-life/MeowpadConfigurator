use thiserror::Error;
use serde_with::{serde_as, DisplayFromStr};
pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Error, Debug, serde::Serialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Error {
    #[error(transparent)]
    Iap(#[from] #[serde_as(as = "DisplayFromStr")] hid_iap::error::Error),
    #[error(transparent)]
    Meowpad(#[from] #[serde_as(as = "DisplayFromStr")] meowpad::error::Error),
    #[error("network error: {0}")]
    Network(#[from] #[serde_as(as = "DisplayFromStr")] reqwest::Error),
    #[error("io error: {0}")]
    Io(#[from] #[serde_as(as = "DisplayFromStr")] std::io::Error),
    #[error("无法找到设备，请尝试重新插拔Meowpad")]
    DeviceNotFound,
    #[error("设备未连接")]
    DeviceDissconnected,
}
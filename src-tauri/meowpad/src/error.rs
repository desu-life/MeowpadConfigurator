use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid_packet")]
    InvalidPacket,
    #[error("empty_config")]
    EmptyConfig,
    #[error("device_disconnected")]
    Disconnect,
    #[error("config_cbor_parse_failed")]
    ConfigCborParseFailed(#[from] ciborium::de::Error<std::io::Error>),
    #[error("config_data_check_failed, key: {0}, data: {1}")]
    ConfigDataCheckFailed(&'static str, usize),
    #[error("read_packet_failed")]
    ReadFailed(#[from] std::io::Error),
    #[error("unexcepted_response, packet_id: {}", .0.id)]
    UnexceptedResponse(crate::packet::Packet),
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    Other(&'static str),
}
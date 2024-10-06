#![feature(cursor_remaining)]

pub mod cbor;
pub mod config;
mod meowboard;
mod packet_id;
mod keymap;

pub use crate::meowboard::*;
pub use crate::packet_id::PacketID;

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_hex::*;
    use std::io::{Cursor, Read};
    use crate::cbor::CborConvertor;


    #[test]
    fn cbor_kb() {
        let c = cbor::Device::default().to_cbor();
        dbg!(c.hex_dump());
        assert!(c.len() <= 1);
    }

}

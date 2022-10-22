pub mod cbor;
mod config;
pub mod keycode;
mod meowpad;
mod packet;

pub use crate::config::*;
pub use crate::meowpad::*;
pub use crate::packet::PacketID;
use keycode::*;

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_hex::*;
    use std::io::{Cursor, Read};

    #[test]
    fn build_packet() {
        use packet::*;
        let packet = Packet::new(
            PacketID::GetConfig,
            [
                0x1D, 0x1B, 0x29, 0x3B, 0x35, 204, 255, 229, 255, 153, 204, 3, 255, 5, 120, 4, 8,
                255, 0, 5, 120, 204, 255, 229, 255, 153, 204, 2, 50, 12, 0x1D, 0x1B, 0x29, 0x3B,
                0x35, 204, 255, 229, 255, 153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255,
                229, 255, 153, 204, 2, 50, 12, 0x1D, 0x1B, 0x29, 0x3B, 0x35, 204, 255, 229, 255,
                153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255, 229, 255, 153, 204, 2,
                0x35, 204, 255, 229, 255, 153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255,
                229, 255, 153, 204, 2, 50, 12, 0x1D, 0x1B, 0x29, 0x3B, 0x35, 204, 255, 229, 255,
                153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255, 229, 255, 153, 204, 2,
                0x35, 204, 255, 229, 255, 153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255,
                229, 255, 153, 204, 2, 50, 12, 0x1D, 0x1B, 0x29, 0x3B, 0x35, 204, 255, 229, 255,
                153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255, 229, 255, 153, 204, 2,
                50, 12,
            ],
        );
        // let packets: Vec<[u8; 65]> = packet.iter().collect();
        dbg!(packet.to_string());
        for p in packet.build_packets() {
            dbg!(p.hex_dump());
        }
    }

    #[test]
    fn read_packets() {
        use byteorder::{BigEndian, ReadBytesExt};
        use num::FromPrimitive;
        use packet::*;
        // 生成packet
        let packet = Packet::new(
            PacketID::GetConfig,
            [
                0x1D, 0x1B, 0x29, 0x3B, 0x35, 204, 255, 229, 255, 153, 204, 3, 255, 5, 120, 4, 8,
                255, 0, 5, 120, 204, 255, 229, 255, 153, 204, 2, 50, 12, 0x1D, 0x1B, 0x29, 0x3B,
                0x35, 204, 255, 229, 255, 153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255,
                229, 255, 153, 204, 2, 50, 12, 0x1D, 0x1B, 0x29, 0x3B, 0x35, 204, 255, 229, 255,
                153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255, 229, 255, 153, 204, 2,
                0x35, 204, 255, 229, 255, 153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255,
                229, 255, 153, 204, 2, 50, 12, 0x1D, 0x1B, 0x29, 0x3B, 0x35, 204, 255, 229, 255,
                153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255, 229, 255, 153, 204, 2,
                0x35, 204, 255, 229, 255, 153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255,
                229, 255, 153, 204, 2, 50, 12, 0x1D, 0x1B, 0x29, 0x3B, 0x35, 204, 255, 229, 255,
                153, 204, 3, 255, 5, 120, 4, 8, 255, 0, 5, 120, 204, 255, 229, 255, 153, 204, 2,
                50, 12,
            ],
        );
        let mut packet_builder = packet.clone().build_packets();
        // 快捷fn
        fn read_packet(builder: &mut PacketBuilder, buf: &mut [u8]) {
            let mut binding = builder.next().unwrap().to_vec();
            let mut _c = Cursor::<&mut [u8]>::new(binding.as_mut());
            _c.set_position(1);
            _c.read_exact(buf).unwrap();
        }

        let mut buf = Cursor::new([0u8; 64]);
        read_packet(&mut packet_builder, buf.get_mut());
        let packet_id = PacketID::from_u8(buf.read_u8().unwrap()).expect("解析数据包ID时报错");
        let packet_len = buf.read_u16::<BigEndian>().unwrap() as usize;
        let mut data = Vec::with_capacity(packet_len);
        let mut read_bytes = 0;
        loop {
            if read_bytes < packet_len {
                match buf.read_u8() {
                    Ok(b) => {
                        read_bytes += 1;
                        data.push(b)
                    }
                    Err(_) => {
                        // cur已经遍历结束
                        buf.get_mut().iter_mut().for_each(|b| *b = 0);
                        buf.set_position(0);
                        read_packet(&mut packet_builder, buf.get_mut());
                    }
                }
            } else {
                break;
            }
        }
        let packet_recv = Packet::new(packet_id, data);
        dbg!(packet_recv.to_string());
        dbg!(packet_recv.data.hex_dump());
        assert!(packet_recv == packet)
    }

    #[test]
    fn kbparse() {
        use num::FromPrimitive;
        let mut r = KbReport::default();
        r.pressed(KeyCode::A);
        r.pressed(KeyCode::B);
        r.pressed(KeyCode::C);
        r.pressed(KeyCode::LShift);
        r.pressed(KeyCode::LAlt);
        
        let r: [KeyCode; 4] = r.try_into().unwrap();
        dbg!(r);
        assert_eq!(r, [KeyCode::A, KeyCode::B, KeyCode::C, KeyCode::LShift]);
        
        let mut r = KbReport::default();
        let r: [KeyCode; 4] = r.try_into().unwrap();
        dbg!(r);
        assert_eq!(r, [KeyCode::No, KeyCode::No, KeyCode::No, KeyCode::No]);
    }
}

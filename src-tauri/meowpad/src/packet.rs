use byteorder::{BigEndian, WriteBytesExt};
use num_derive::{FromPrimitive, ToPrimitive};
use serde::Deserialize;
use std::io::Cursor;
use std::io::Write;

#[derive(Deserialize, FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum PacketID {
    Null = 0,
    Ok = 1,
    Bad = 2,
    Ping = 3,
    FlushConfig = 4,
    WriteConfig = 5,
    GetConfig = 6,
    GetFirmwareVersion = 7,
    GetDeviceName = 8,
    CalibrationKey = 9,
}

impl std::fmt::Display for PacketID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PacketID {
    pub fn to_packet(self, data: impl Into<Vec<u8>>) -> Packet {
        Packet::new(self, data)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Packet {
    pub id: PacketID,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn new(packet_id: PacketID, data: impl Into<Vec<u8>>) -> Self {
        let data = data.into();
        Self {
            id: packet_id,
            data: data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn build_vec(self) -> Vec<u8> {
        let body_len = self.len();
        // 1        2         total: 3bytes
        // packetID packetBodyLength
        let mut cur = Cursor::new(Vec::with_capacity(3 + body_len));
        cur.write_u8(self.id as u8).unwrap();
        cur.write_u16::<BigEndian>(body_len as u16).unwrap();
        cur.write(self.data.as_slice()).unwrap();
        cur.into_inner()
    }

    pub fn build_packets(self) -> PacketBuilder {
        let vec = self.build_vec();
        PacketBuilder {
            len: vec.len(),
            inner: vec,
            pos: 0,
        }
        .into_iter()
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet (id: {}, len: {})", self.id, self.data.len())
    }
}

impl IntoIterator for Packet {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.build_vec().into_iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PacketBuilder {
    inner: Vec<u8>,
    len: usize,
    pos: usize,
}

impl Iterator for PacketBuilder {
    // We can refer to this type using Self::Item
    type Item = [u8; 65];
    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        if self.len > self.pos {
            let mut arr = [0u8; 65];
            let mut arr_iter = arr.iter_mut().skip(1);
            if self.pos >= 64 {
                arr_iter.next().map(|v| *v = 0xFE).unwrap()
            }
            for p in arr_iter {
                match self.inner.get(self.pos) {
                    Some(&v) => {
                        *p = v;
                        self.pos += 1;
                    }
                    None => break,
                }
            }
            Some(arr)
        } else {
            None
        }
    }
}

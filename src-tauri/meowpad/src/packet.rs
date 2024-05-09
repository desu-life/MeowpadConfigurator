use byteorder::{BigEndian, WriteBytesExt};
use std::io::Cursor;
use std::io::Write;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Packet {
    pub id: u8,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn new(packet_id: impl Into<u8>, data: impl Into<Vec<u8>>) -> Self {
        let data = data.into();
        Self {
            id: packet_id.into(),
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
        cur.write_u8(self.id).unwrap();
        cur.write_u16::<BigEndian>(body_len as u16).unwrap();
        cur.write(self.data.as_slice()).unwrap();
        cur.into_inner()
    }

    fn build_vec_large(self) -> Vec<u8> {
        let body_len = self.len().max(62);
        // 1        2         total: 3bytes
        // packetID packetBodyLength
        let mut cur = Cursor::new(Vec::with_capacity(3 + body_len));
        cur.write_u8(self.id).unwrap();
        cur.write_u16::<BigEndian>(body_len as u16).unwrap();
        cur.write(self.data.as_slice()).unwrap();
        cur.get_mut().resize(3 + body_len, 0);
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

    pub fn build_packets_large(self) -> PacketBuilder {
        let vec = self.build_vec_large();
        PacketBuilder {
            len: vec.capacity(),
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

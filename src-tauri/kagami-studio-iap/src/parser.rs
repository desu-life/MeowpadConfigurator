
use crate::crc32;


#[derive(Debug, Clone)]
pub struct FilePartInfo<const N: usize> {
    pub offset: u32,
    pub length: usize,
    pub data: [u8; N],
}

impl<const N: usize> crc32::CRC32 for FilePartInfo<N> {

    fn crc32(&self) -> u32 {
        crc32::crc32(&self.data)
    }
}

impl<const N: usize> FilePartInfo<N> {
    pub fn new(offset: u32, length: usize) -> Self {
        let data = [0xFFu8; N];
        Self {
            offset,
            length,
            data,
        }
    }
}

pub struct HexParser<'a, const PART_SIZE: usize> {
    parts: Vec<FilePartInfo<PART_SIZE>>,
    data: &'a str,
}

impl<'a, const PART_SIZE: usize> HexParser<'a, PART_SIZE> {
    pub fn new(string: &'a str) -> Self {
        Self {
            parts: Vec::new(),
            data: string,
        }
    }

    fn parse_data(&mut self, offset: u32, value: &[u8]) {
        if value.is_empty() {
            return;
        }

        let mut remaining_data = value;
        let mut current_offset = offset;

        while !remaining_data.is_empty() {
            let part_offset = current_offset & !(PART_SIZE as u32 - 1);

            let need_new_part = self.parts.last().map(|p| {
                part_offset != p.offset
            }).unwrap_or(true);

            if need_new_part {
                self.parts.push(FilePartInfo::new(part_offset, PART_SIZE));
            }

            let part = self.parts.last_mut().unwrap();
            let offset_in_part = (current_offset - part.offset) as usize;
            let available_space = PART_SIZE - offset_in_part;
            let bytes_to_copy = remaining_data.len().min(available_space);

            part.data[offset_in_part..offset_in_part + bytes_to_copy].copy_from_slice(&remaining_data[..bytes_to_copy]);

            remaining_data = &remaining_data[bytes_to_copy..];
            current_offset += bytes_to_copy as u32;
        }
    }

    pub fn parse(mut self) -> Result<Vec<FilePartInfo<PART_SIZE>>, ihex::ReaderError> {
        let ihex = ihex::Reader::new(self.data);
        let mut base_addr = 0;
        for r in ihex {
            match r? {
                ihex::Record::Data { offset, value } => {
                    let offset = offset as u32 + base_addr;
                    self.parse_data(offset, &value);
                }
                ihex::Record::EndOfFile => break,
                ihex::Record::ExtendedLinearAddress(a) => base_addr = (a as u32) << 16,
                _ => (),
            }
        }

        Ok(self.parts)
    }
}


impl<const PART_SIZE: usize> TryFrom<HexParser<'_, PART_SIZE> > for Vec<FilePartInfo<PART_SIZE>> {
    type Error = ihex::ReaderError;
    fn try_from(value: HexParser<'_, PART_SIZE>) -> Result<Self, Self::Error> {
        value.parse()
    }
}


pub struct BinParser<'a, const PART_SIZE: usize> {
    pub base_addr: u32,
    pub parts: Vec<FilePartInfo<PART_SIZE>>,
    pub data: &'a [u8],
}


impl<'a, const PART_SIZE: usize> BinParser<'a, PART_SIZE> {
    pub fn new(data: &'a [u8], base_addr: u32) -> Self {
        Self {
            base_addr,
            parts: Vec::new(),
            data,
        }
    }

    pub fn parse(mut self) -> Result<Vec<FilePartInfo<PART_SIZE>>, std::io::Error> {
        if self.data.is_empty() {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }

        let mut remaining_data = self.data;
        let mut current_offset = self.base_addr;

        while !remaining_data.is_empty() {
            let part_offset = current_offset & !(PART_SIZE as u32 - 1);

            let need_new_part = self.parts.last().map(|p| {
                part_offset != p.offset
            }).unwrap_or(true);

            if need_new_part {
                self.parts.push(FilePartInfo::new(part_offset, PART_SIZE));
            }

            let part = self.parts.last_mut().unwrap();
            let offset_in_part = (current_offset - part.offset) as usize;
            let available_space = PART_SIZE - offset_in_part;
            let bytes_to_copy = remaining_data.len().min(available_space);

            part.data[offset_in_part..offset_in_part + bytes_to_copy].copy_from_slice(&remaining_data[..bytes_to_copy]);

            remaining_data = &remaining_data[bytes_to_copy..];
            current_offset += bytes_to_copy as u32;
        }

        Ok(self.parts)
    }
}
use crate::{
    crc32::CRC32,
    error::{IAPError, IAPResult},
    parser::FilePartInfo,
};
use meowpad::Device;
use scroll::{Pread, Pwrite};


pub const USBCOMMOND_MSB: u8 = 0x5A;
pub const USBCOMMOND_IAPMODE: u8 = 0xA0;
pub const USBCOMMOND_BEGINDOWN: u8 = 0xA1;
pub const USBCOMMOND_ADDRESS: u8 = 0xA2;
pub const USBCOMMOND_SENDDATA: u8 = 0xA3;
pub const USBCOMMOND_SENDEND: u8 = 0xA4;
pub const USBCOMMOND_CRC: u8 = 0xA5;
pub const USBCOMMOND_JUMPAPP: u8 = 0xA6;
pub const USBCOMMOND_GETAPPADDR: u8 = 0xA7;

pub const USBCOMMOND_ACK_MSB: u8 = 0xFF;
pub const USBCOMMOND_ACK_LSB: u8 = 0x00;
// pub const USBCOMMOND_NACK_MSB: u8 = 0x00;
// pub const USBCOMMOND_NACK_LSB: u8 = 0xFF;

fn check_ack_header(command: u8, buffer: &[u8]) -> bool {
    assert!(buffer.len() >= 4);

    buffer[0..4]
        == [
            USBCOMMOND_MSB,
            command,
            USBCOMMOND_ACK_MSB,
            USBCOMMOND_ACK_LSB,
        ]
}

pub struct KagamiStudioIAP<D: Device> {
    device: D,
}

impl<D: Device> KagamiStudioIAP<D> {
    pub fn new(device: D) -> IAPResult<Self> {
        Ok(KagamiStudioIAP { device })
    }

    fn write(&self, report_id: u8, data: &[u8]) -> IAPResult<usize> {
        use heapless::Vec;
        let mut buffer: Vec<u8, 65> = Vec::new(); // 最大64字节
        buffer.push(report_id).map_err(|_| IAPError::BufferFull)?;
        buffer
            .extend_from_slice(data)
            .map_err(|_| IAPError::BufferFull)?;

        self.device.write(&buffer).map_err(IAPError::DeviceError)
    }

    fn read(&self, buffer: &mut [u8]) -> IAPResult<usize> {
        self.device.read(buffer).map_err(IAPError::DeviceError)
    }

    fn send_and_check_ack(
        &self,
        send_buffer: &[u8],
        recv_buffer: &mut [u8],
        command: u8,
    ) -> IAPResult<usize> {
        self.write(0, send_buffer)?;
        let len = self.read(recv_buffer)?;
        if !check_ack_header(command, recv_buffer) {
            return Err(IAPError::InvalidAckHeader {
                expected: command,
                actual: recv_buffer[1],
            });
        }
        Ok(len)
    }

    pub fn get_iap_address(&self) -> IAPResult<u32> {
        let mut buffer = [0u8; 8];
        let res = self.send_and_check_ack(
            &[USBCOMMOND_MSB, USBCOMMOND_GETAPPADDR],
            &mut buffer,
            USBCOMMOND_GETAPPADDR,
        )?;
        buffer[..res]
            .pread_with(4, scroll::BE)
            .map_err(IAPError::InvalidData)
    }

    pub fn enter_iap_mode(&self) -> IAPResult<()> {
        self.send_and_check_ack(
            &[USBCOMMOND_MSB, USBCOMMOND_IAPMODE],
            &mut [0u8; 4],
            USBCOMMOND_IAPMODE,
        )
        .map(|_| ())
    }

    pub fn begin_download_usb(&self) -> IAPResult<()> {
        self.send_and_check_ack(
            &[USBCOMMOND_MSB, USBCOMMOND_BEGINDOWN],
            &mut [0u8; 4],
            USBCOMMOND_BEGINDOWN,
        )
        .map(|_| ())
    }

    pub fn end_download_usb(&self) -> IAPResult<()> {
        self.send_and_check_ack(
            &[USBCOMMOND_MSB, USBCOMMOND_SENDEND],
            &mut [0u8; 4],
            USBCOMMOND_SENDEND,
        )
        .map(|_| ())
    }

    pub fn jump_to_app(&self) -> IAPResult<()> {
        self.send_and_check_ack(
            &[USBCOMMOND_MSB, USBCOMMOND_JUMPAPP],
            &mut [0u8; 4],
            USBCOMMOND_JUMPAPP,
        )
        .map(|_| ())
    }

    pub fn crc_verify_part<const PART_SIZE: usize>(
        &self,
        file_part: &FilePartInfo<PART_SIZE>,
    ) -> IAPResult<()> {
        let address = file_part.offset;

        let page_num = 2;
        let mut buffer = [0u8; 8];
        buffer[0] = USBCOMMOND_MSB;
        buffer[1] = USBCOMMOND_CRC;
        buffer.pwrite_with::<u32>(address, 2, scroll::BE)?;
        buffer.pwrite_with::<u16>(page_num, 6, scroll::BE)?;

        let mut recv_buffer = [0u8; 8];
        let res = self.send_and_check_ack(&buffer, &mut recv_buffer, USBCOMMOND_CRC)?;
        let device_crc: u32 = recv_buffer[..res].pread_with(4, scroll::BE)?;
        let file_crc = file_part.crc32();

        if device_crc != file_crc {
            return Err(IAPError::InvalidCRC {
                address,
                expected: file_crc,
                actual: device_crc,
            });
        }

        Ok(())
    }

    pub fn download_file_part<const PART_SIZE: usize>(
        &self,
        file_part: &FilePartInfo<PART_SIZE>,
    ) -> IAPResult<()> {
        const USB_PART_LENGTH: usize = 1024;
        for (chunk_index, chunk) in file_part.data.chunks(USB_PART_LENGTH).enumerate() {
            let addr: u32 = file_part.offset + chunk_index as u32 * USB_PART_LENGTH as u32;

            let mut addr_buffer = [0u8; 6];
            addr_buffer[0] = USBCOMMOND_MSB;
            addr_buffer[1] = USBCOMMOND_ADDRESS;
            addr_buffer.pwrite_with(addr, 2, scroll::BE)?;
            self.send_and_check_ack(&addr_buffer, &mut [0u8; 4], USBCOMMOND_ADDRESS)?;

            const DATA_PAYLOAD_LEN: usize = 64 - 4; // 2bytes command, 2bytes length

            for usb_chunk in chunk.chunks(DATA_PAYLOAD_LEN) {
                let mut buffer = [0u8; 64];
                buffer[0] = USBCOMMOND_MSB;
                buffer[1] = USBCOMMOND_SENDDATA;
                buffer.pwrite_with(usb_chunk.len() as u16, 2, scroll::BE)?;
                buffer[4..4 + usb_chunk.len()].copy_from_slice(usb_chunk);
                self.write(0, &buffer)?;
            }

            let mut recv_buffer = [0u8; 4];
            let len = self.read(&mut recv_buffer)?;
            if !check_ack_header(USBCOMMOND_SENDDATA, &recv_buffer[..len]) {
                return Err(IAPError::InvalidAckHeader {
                    expected: USBCOMMOND_SENDDATA,
                    actual: recv_buffer[1],
                });
            }
        }
        Ok(())
    }
}

use std::{mem::{self, ManuallyDrop}, slice::{self, Chunks}};
use hidapi::HidDevice;
use crate::{packet::{IAPCommand, IAPacket}, error::*};

#[derive(PartialEq)]
pub enum IAPState {
    Idle,
    Programming,
    Verifying,
}

pub struct IAP<'a> {
    device: HidDevice,
    firmware_data: Option<Vec<u8>>,
    tmp_data: Option<Chunks<'a, u8>>,
    flash_addr: u16,
    pub state: IAPState,
}

impl<'a> IAP<'a> {
    pub fn new(device: HidDevice) -> IAP<'a> {
        IAP {
            device,
            firmware_data: None,
            tmp_data: None,
            flash_addr: 0,
            state: IAPState::Idle,
        }
    }

    fn reset_tmp_data(&mut self) {
        let v = self.firmware_data.as_ref().unwrap();
        let s = unsafe { slice::from_raw_parts(v.as_ptr(), v.len()) };
        self.tmp_data = Some(s.chunks(60));
        self.flash_addr = 0;
    }

    pub fn start_program(&mut self, firmware_data: Vec<u8>) -> Result<usize> {
        let len = firmware_data.len();
        self.firmware_data = Some(firmware_data);
        self.erase()?;
        self.reset_tmp_data();
        self.state = IAPState::Programming;

        Ok(len)
    }

    pub fn program(&mut self) -> Result<u16> {
        if self.state != IAPState::Programming {
            return Err(Error::Other("IAP is not in programming state"));
        }

        let Some(chunks) = self.tmp_data.as_mut() else {
            return Err(Error::NoFirmwareData);
        };

        if let Some(chunk) = chunks.next() {
            let cmd_data_len = chunk.len() as u8;

            // 填充命令包
            let mut iap_cmd = IAPacket {
                cmd: IAPCommand::Program,
                len: cmd_data_len,
                addr: self.flash_addr.to_le_bytes(),
                buf: [0u8; 60],
            };

            iap_cmd.buf[0..cmd_data_len as usize]
                .copy_from_slice(chunk);

            // 发送IAP命令包
            self.device.write(&iap_cmd.packet())?;

            // 接收操作结果2字节
            let mut buf = [0u8; 2];
            self.device.read(&mut buf)?;

            if buf[0] != 0x00 || buf[1] != 0x00 {
                return Err(Error::CommandFailed(IAPCommand::Program));
            }
        
            self.flash_addr += cmd_data_len as u16;
        } else {
            self.reset_tmp_data();
            self.state = IAPState::Verifying;
        }

        Ok(self.flash_addr)
    }

    pub fn verify(&mut self) -> Result<u16> {
        if self.state != IAPState::Verifying {
            return Err(Error::Other("IAP is not in verifying state"));
        }

        let Some(chunks) = self.tmp_data.as_mut() else {
            return Err(Error::NoFirmwareData);
        };

        if let Some(chunk) = chunks.next() {
            let cmd_data_len = chunk.len() as u8;

            // 填充命令包
            let mut iap_cmd = IAPacket {
                cmd: IAPCommand::Verify,
                len: cmd_data_len,
                addr: self.flash_addr.to_le_bytes(),
                buf: [0u8; 60],
            };

            iap_cmd.buf[0..cmd_data_len as usize]
                .copy_from_slice(chunk);

            // 发送IAP命令包
            self.device.write(&iap_cmd.packet())?;

            // 接收操作结果2字节
            let mut buf = [0u8; 2];
            self.device.read(&mut buf)?;

            if buf[0] != 0x00 || buf[1] != 0x00 {
                return Err(Error::CommandFailed(IAPCommand::Verify));
            }
        
            self.flash_addr += cmd_data_len as u16;
        } else {
            self.state = IAPState::Idle;
            self.end()?;
        }

        Ok(self.flash_addr)
    }

    fn erase(&self) -> Result<()> {
        // 发送擦除命令
        let erase_command = IAPacket {
            cmd: IAPCommand::Erase,
            len: 0x0,
            addr: [0x0, 0x0],
            buf: [0x0; 60],
        };
        self.device.write(&erase_command.packet())?;

        let mut buf = [0u8; 2];
        self.device.read(&mut buf)?;
        if buf[0] != 0x00 || buf[1] != 0x00 {
            return Err(Error::CommandFailed(IAPCommand::Erase));
        }

        Ok(())
    }

    fn end(&self) -> Result<()> {
        let mut iap_cmd = IAPacket {
            cmd: IAPCommand::End,
            len: 0,
            addr: [0, 0],
            buf: [0; 60],
        };

        iap_cmd.len = std::mem::size_of::<u16>() as u8; // 后续数据长度

        self.device.write(&iap_cmd.packet())?;

        Ok(())
    }
}
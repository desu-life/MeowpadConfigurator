#[allow(dead_code)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct IAPacket {
    pub cmd: IAPCommand,
    pub len: u8,
    pub addr: [u8; 2],
    pub buf: [u8; 60],  // (IAP_LEN - 4)
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IAPCommand {
    Program = 0x80,
    Erase,
    Verify,
    End
}

impl From<IAPCommand> for u8 {
    fn from(value: IAPCommand) -> Self {
        value as u8
    }
}

impl AsRef<[u8]> for IAPacket {
    fn as_ref(&self) -> &[u8] {
        // 使用unsafe块将结构体转换为字节数组(slice)
        unsafe {
            let ptr = self as *const IAPacket as *const u8;
            std::slice::from_raw_parts(ptr, std::mem::size_of::<IAPacket>())
        }
    }
}

impl IAPacket {
    pub fn packet(&self) -> [u8; 65] {
        let mut buf = [0u8; 65];
        buf[0] = 0x00;
        buf[1..].copy_from_slice(self.as_ref());
        buf
    }
}

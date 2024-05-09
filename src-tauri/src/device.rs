pub struct HidDevice {
    pub device: hidapi::HidDevice,
}
unsafe impl Send for HidDevice {}

impl meowpad::Device for HidDevice {
    fn write(&self, data: &[u8]) -> meowpad::Result<usize> {
        self.device.write(data).map_err(|_| meowpad::error::Error::Disconnect)
    }

    fn read(&self, buf: &mut [u8]) -> meowpad::Result<usize> {
        self.device.read(buf).map_err(|_| meowpad::error::Error::Disconnect)
    }

    fn read_timeout(&self, buf: &mut [u8], timeout: i32) -> meowpad::Result<usize> {
        self.device.read_timeout(buf, timeout).map_err(|_| meowpad::error::Error::Disconnect)
    }
}
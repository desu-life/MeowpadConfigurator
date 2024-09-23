pub trait Device {
    fn get_serial(&self) -> crate::Result<String>;
    fn write(&self, data: &[u8]) -> crate::Result<usize>;
    fn read(&self, buf: &mut [u8]) -> crate::Result<usize>;
    fn read_timeout(&self, buf: &mut [u8], timeout: i32) -> crate::Result<usize>;
}

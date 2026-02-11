use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Device {
    async fn get_serial(&self) -> crate::Result<String>;
    async fn write(&self, data: &[u8]) -> crate::Result<usize>;
    async fn read(&self, buf: &mut [u8]) -> crate::Result<usize>;
    async fn read_timeout(&self, buf: &mut [u8], timeout: i32) -> crate::Result<usize>;
    async fn clear_buffer(&self) -> crate::Result<()>;
    
    /// Optional delay between operations (in milliseconds)
    /// Default implementation does nothing (for WASM compatibility)
    async fn delay(&self, _ms: u64) -> crate::Result<()> {
        Ok(())
    }
}

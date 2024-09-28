use std::ffi::CString;

pub struct HidDevice {
    pub device: hidapi::HidDevice,
}
unsafe impl Send for HidDevice {}

impl meowpad::Device for HidDevice {
    fn get_serial(&self) -> meowpad::Result<String> {
        self.device
            .get_serial_number_string()
            .map(|v| v.unwrap_or_default())
            .map_err(|_| meowpad::error::Error::Disconnect)
    }

    fn write(&self, data: &[u8]) -> meowpad::Result<usize> {
        self.device
            .write(data)
            .map_err(|_| meowpad::error::Error::Disconnect)
    }

    fn read(&self, buf: &mut [u8]) -> meowpad::Result<usize> {
        self.device
            .read(buf)
            .map_err(|_| meowpad::error::Error::Disconnect)
    }

    fn read_timeout(&self, buf: &mut [u8], timeout: i32) -> meowpad::Result<usize> {
        self.device
            .read_timeout(buf, timeout)
            .map_err(|_| meowpad::error::Error::Disconnect)
    }

    fn clear_buffer(&self) -> meowpad::Result<()> {
        self.device.set_blocking_mode(false).map_err(|_| meowpad::error::Error::Disconnect)?;
        let mut buf = [0u8; 64];
        while let Ok(s) = self.device.read(&mut buf) {
            if s == 0 {
                break;
            }
        }
        self.device.set_blocking_mode(true).map_err(|_| meowpad::error::Error::Disconnect)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DeviceInfoExtened<'a> {
    pub device_name: String,
    pub firmware_version: String,
    pub serial_number: Option<String>,
    pub inner: &'a hidapi::DeviceInfo,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DeviceInfoSerdi {
    pub path: CString,
    pub vendor_id: u16,
    pub product_id: u16,
    pub interface_number: i32,
    pub device_name: String,
    pub firmware_version: String,
    pub serial_number: Option<String>,
}

impl From<DeviceInfoExtened<'_>> for DeviceInfoSerdi {
    fn from(value: DeviceInfoExtened) -> Self {
        Self {
            path: value.inner.path().to_owned(),
            vendor_id: value.inner.vendor_id(),
            product_id: value.inner.product_id(),
            interface_number: value.inner.interface_number(),
            device_name: value.device_name,
            firmware_version: value.firmware_version,
            serial_number: value.serial_number,
        }
    }
}

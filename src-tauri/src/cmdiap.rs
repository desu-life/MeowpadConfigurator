use std::{sync::Mutex, thread, time::Duration};

use crate::{device::DeviceInfoExtened, error::{self, Error, Result}, MEOWPAD_DEVICE_NAME};
use hid_iap::iap::{IAPState, IAP};
use hidapi::HidApi;
use log::*;
use tauri::{Manager, State};


pub fn find_devices(api: &HidApi) -> Vec<DeviceInfoExtened> {
    // 期望的设备VID和PID
    const VID: u16 = 0x5D3E;
    const PID: u16 = 0xFE08;

    // 迭代设备列表，查找符合条件的设备
    let devices = api.device_list();
    devices.filter_map(|d| {
        // 过滤设备
        if !(d.vendor_id() == VID && d.product_id() == PID) {
            return None;
        }

        // 连接设备
        Some(DeviceInfoExtened {
            device_name: MEOWPAD_DEVICE_NAME.to_owned(),
            firmware_version: "IAP".to_owned(),
            serial_number: None,
            inner: d,
        })
    }).collect()
}



#[tauri::command]
pub fn connect_iap(iap_device: State<'_, Mutex<Option<IAP>>>) -> Result<()> {
    let api = HidApi::new().unwrap();

    match find_devices(&api)
        .first()
        .and_then(|d| match d.inner.open_device(&api) {
            Ok(d) => Some(IAP::new(d)),
            Err(_) => None,
        }) {
        Some(device) => {
            let mut _d = iap_device.lock().unwrap();
            info!("固件更新");
            _d.replace(device);
            Ok(())
        }
        None => {
            warn!("连接失败，无法找到设备");
            Err(error::Error::DeviceNotFound)
        }
    }
}

fn check_firmware(data: &[u8]) -> bool {
    if data.len() > 512 && data[52] == 0x73 && data[53] == 0x00 && data[54] == 0x10 && data[55] == 0x00 {
        return true;
    }
    false
}
    

#[tauri::command]
pub fn iap_start(iap_device: State<'_, Mutex<Option<IAP>>>, data: Vec<u8>) -> Result<usize> {
    if !check_firmware(&data) {
        return Ok(0);
    }
    let mut _iap = iap_device.lock().unwrap();
    let iap = _iap.as_mut().ok_or(Error::DeviceDisconnected)?;
    let len = iap.start_program(data)?;
    Ok(len)
}

#[tauri::command]
pub fn iap_flush(app: tauri::AppHandle, iap_device: State<'_, Mutex<Option<IAP>>>) -> Result<()> {
    let mut _iap = iap_device.lock().unwrap();
    let iap = _iap.as_mut().ok_or(Error::DeviceDisconnected)?;
    while iap.state == IAPState::Programming {
        let pos = iap.program()?;
        thread::sleep(Duration::from_millis(1));
        app.emit_all("iap_process", &[pos, IAPState::Programming as u16])
            .unwrap();
    }

    while iap.state == IAPState::Verifying {
        let pos = iap.verify()?;
        thread::sleep(Duration::from_millis(1));
        app.emit_all("iap_process", &[pos, IAPState::Verifying as u16])
            .unwrap();
    }
    Ok(())
}

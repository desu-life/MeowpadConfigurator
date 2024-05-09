use std::{sync::Mutex, thread, time::Duration};

use crate::error::{self, Error, Result};
use hid_iap::iap::{IAPState, IAP};
use hidapi::HidApi;
use log::*;
use tauri::{Manager, State};

#[tauri::command]
pub fn connect_iap(iap_device: State<'_, Mutex<Option<IAP>>>) -> Result<()> {
    const VID: u16 = 0x5D3E;
    const PID: u16 = 0xFE08;
    let api = HidApi::new().unwrap();

    match api.open(VID, PID) {
        Ok(device) => {
            info!("固件更新");
            let mut _d = iap_device.lock().unwrap();
            *_d = Some(IAP::new(device));
            Ok(())
        }
        Err(_) => {
            warn!("连接失败，无法找到设备");
            Err(error::Error::DeviceNotFound)
        }
    }
}

#[tauri::command]
pub fn iap_start(iap_device: State<'_, Mutex<Option<IAP>>>, data: Vec<u8>) -> Result<usize> {
    let mut _iap = iap_device.lock().unwrap();
    let iap = _iap.as_mut().ok_or(Error::DeviceDisconnected)?;
    // unsafe {
    //     FIRMWARE_DATA = Some(data);
    //     let data_ref = FIRMWARE_DATA.as_deref().unwrap();
    //     let len = iap.start_program(data_ref)?;
    //     Ok(len)
    // }
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


use std::sync::Mutex;
use hidapi::HidApi;
use meowpad::models::KeyRTStatus;
use meowpad3k::Meowpad;
use tauri::State;
use crate::{device::HidDevice, error::{self, Error, Result}};
use log::*;


#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Copy)]
struct Config {
    key: meowpad3k::config::Key,
    light: meowpad3k::config::Light,
}

#[tauri::command]
pub fn calibration_key_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.calibration_key()?;
    Ok(())
}

#[tauri::command]
pub fn get_debug_value_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<[KeyRTStatus; 3]> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    Ok(d.get_debug_value()?)
}

#[tauri::command]
pub fn erase_firmware_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.erase_firmware()?;
    Ok(())
}

#[tauri::command]
pub fn get_default_key_config_3k() -> meowpad4k::config::Key {
    meowpad4k::cbor::Keyboard::default().try_into().unwrap()
}

#[tauri::command]
pub fn get_default_light_config_3k() -> meowpad4k::config::Light {
    meowpad4k::cbor::Light::default().try_into().unwrap()
}

#[tauri::command]
pub fn get_key_config_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<meowpad3k::config::Key> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.load_key_config()?;
    Ok(d.key_config.unwrap().try_into()?)
}

#[tauri::command]
pub fn get_light_config_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<meowpad3k::config::Light> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.load_light_config()?;
    Ok(d.light_config.unwrap().try_into()?)
}

#[tauri::command]
pub fn set_key_config_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>, config: meowpad3k::config::Key) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.key_config = Some(config.into());
    d.set_key_config()?;
    Ok(())
}

#[tauri::command]
pub fn set_light_config_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>, config: meowpad3k::config::Light) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.light_config = Some(config.into());
    d.set_light_config()?;
    Ok(())
}

#[tauri::command]
pub fn save_key_config_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.save_key_config()?;
    Ok(())
}

#[tauri::command]
pub fn save_light_config_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.save_light_config()?;
    Ok(())
}

#[tauri::command]
pub fn get_raw_config_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<String> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.load_key_config()?;
    d.load_light_config()?;
    Ok(toml::to_string(&Config {
        key: d.key_config.unwrap().try_into()?,
        light: d.light_config.unwrap().try_into()?,
    })
    .unwrap())
}

#[tauri::command]
pub fn check_raw_config_3k(config: String) -> bool {
    toml::from_str::<Config>(&config).is_ok()
}

#[tauri::command]
pub fn save_raw_config_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>, config: String) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    let cfg = toml::from_str::<Config>(&config).expect("错误配置");
    d.key_config = Some(cfg.key.into());
    d.set_key_config()?;
    d.save_key_config()?;
    d.light_config = Some(cfg.light.into());
    d.set_light_config()?;
    d.save_light_config()?;
    Ok(())
}


#[tauri::command]
pub fn connect_3k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    _d.replace(_connect()?);
    Ok(())
}


fn _connect() -> Result<Meowpad<HidDevice>> {
    info!("开始连接!");
    let found_device = find_device();

    match found_device {
        Some(device) => {
            info!("连接到设备");
            Ok(device)
        }
        None => {
            warn!("连接失败，无法找到设备");
            Err(error::Error::DeviceNotFound)
        }
    }
}

fn find_device() -> Option<Meowpad<HidDevice>> {
    // 获取设备列表
    let api = HidApi::new().unwrap();

    // 期望的设备VID和PID
    const VID: u16 = 0x5D3E;
    const PID: u16 = 0xFE17;

    // 迭代设备列表，查找符合条件的设备
    let mut devices = api.device_list();
    devices.find_map(|d| {
        // 过滤设备
        if !(d.vendor_id() == VID && d.product_id() == PID) {
            return None;
        }

        // 连接设备
        let device_handle = match d.open_device(&api) {
            Ok(d) => Meowpad::new(HidDevice { device: d }),
            Err(_) => return None,
        };

        // 测试连接
        match device_handle.ping() {
            Ok(r) if !r => None,
            Err(_) => None,
            _ => {
                debug!("Name: {}", d.product_string().unwrap_or_default());
                debug!(
                    "Manufacturer: {}",
                    d.manufacturer_string().unwrap_or_default()
                );
                debug!("Addr: {}", d.path().to_string_lossy());
                debug!("{:?}", d);
                Some(device_handle)
            }
        }
    })
}

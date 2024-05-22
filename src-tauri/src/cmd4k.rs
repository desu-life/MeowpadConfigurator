
use std::sync::Mutex;
use hidapi::HidApi;
use meowpad::models::{DeviceStatus, KeyRTStatus};
use meowpad4k::Meowpad;
use tauri::State;
use crate::{device::HidDevice, error::{Error, Result}, FIRMWARE_VERSION_4K};
use log::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Copy)]
struct Config {
    key: meowpad4k::config::Key,
    light: meowpad4k::config::Light,
}

#[tauri::command]
pub fn get_firmware_4k_version(_app: tauri::AppHandle) -> &'static str {
    FIRMWARE_VERSION_4K
}

#[tauri::command]
pub fn get_device_info_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<serde_json::Value> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.get_device_name()?;
    d.get_firmware_version()?;
    let name = d.device_name.as_ref().expect("参数错误");
    let version = d.firmware_version.as_ref().expect("参数错误");
    info!("设备名称：{}", name);
    info!("固件版本：{}", version);
    Ok(serde_json::json!({
        "name": name,
        "version": version
    }))
}

#[tauri::command]
pub fn get_device_status_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<DeviceStatus> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    let status = d.get_status()?;
    info!(
        "按键配置状态: {}，灯光配置状态: {}，按键校准状态: {}，按键是否启用: {}",
        status.key, status.light, status.hall, status.enabled
    );
    Ok(status)
}

#[tauri::command]
pub fn calibration_key_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.calibration_key()?;
    Ok(())
}

#[tauri::command]
pub fn clear_config_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.clear_hall_config()?;
    d.clear_key_config()?;
    d.clear_light_config()?;
    Ok(())
}

#[tauri::command]
pub fn reset_device_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.reset_device()?;
    Ok(())
}

#[tauri::command]
pub async fn get_debug_value_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<[KeyRTStatus; 4]> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    Ok(d.get_debug_value()?)
}

#[tauri::command]
pub fn erase_firmware_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.erase_firmware()?;
    Ok(())
}

#[tauri::command]
pub fn get_default_key_config_4k() -> meowpad4k::config::Key {
    meowpad4k::cbor::Keyboard::default().try_into().unwrap()
}

#[tauri::command]
pub fn get_default_light_config_4k() -> meowpad4k::config::Light {
    meowpad4k::cbor::Light::default().try_into().unwrap()
}

#[tauri::command]
pub fn get_key_config_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<meowpad4k::config::Key> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.load_key_config()?;
    Ok(d.key_config.unwrap().try_into()?)
}

#[tauri::command]
pub fn get_light_config_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<meowpad4k::config::Light> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.load_light_config()?;
    Ok(d.light_config.unwrap().try_into()?)
}

#[tauri::command]
pub fn set_key_config_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>, config: meowpad4k::config::Key) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.key_config = Some(config.into());
    d.set_key_config()?;
    Ok(())
}

#[tauri::command]
pub fn set_light_config_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>, config: meowpad4k::config::Light) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.light_config = Some(config.into());
    d.set_light_config()?;
    Ok(())
}

#[tauri::command]
pub fn save_key_config_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.save_key_config()?;
    Ok(())
}

#[tauri::command]
pub fn save_light_config_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.save_light_config()?;
    Ok(())
}

#[tauri::command]
pub fn get_raw_config_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> Result<String> {
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
pub fn check_raw_config_4k(config: String) -> bool {
    toml::from_str::<Config>(&config).is_ok()
}

#[tauri::command]
pub fn save_raw_config_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>, config: String) -> Result<()> {
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
pub fn connect_4k(device_handle: State<'_, Mutex<Option<Meowpad<HidDevice>>>>) -> bool {
    let mut _d = device_handle.lock().unwrap();
    info!("开始连接!");
    let found_device = find_device();

    match found_device {
        Some(device) => {
            info!("连接到设备");
            _d.replace(device);
            true
        }
        None => {
            warn!("连接失败，无法找到设备");
            false
        }
    }
}

fn find_device() -> Option<Meowpad<HidDevice>> {
    // 获取设备列表
    let api = HidApi::new().unwrap();

    // 期望的设备VID和PID
    const VID: u16 = 0x5D3E;
    const PID: u16 = 0xFE07;

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

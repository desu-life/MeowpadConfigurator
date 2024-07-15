
use std::sync::Mutex;
use hidapi::HidApi;
use meowpad::models::{DeviceStatus, KeyHallConfig, KeyRTStatus, KeyState};
use meowboard::Meowboard;
use tauri::State;
use crate::{device::HidDevice, error::{Error, Result}};
use log::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Copy)]
struct Config {
    key: meowboard::config::Device,
}

#[tauri::command]
pub fn get_firmware_kb_version(_app: tauri::AppHandle) -> &'static str {
    "0"
}

#[tauri::command]
pub fn get_device_info_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<serde_json::Value> {
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
pub fn get_device_status_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<DeviceStatus> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    let status = d.get_status()?;
    info!(
        "按键配置状态: {}，按键校准状态: {}，按键是否启用: {}",
        status.key, status.hall, status.enabled
    );
    Ok(status)
}

#[tauri::command]
pub fn calibration_key_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>, key_indexs: Vec<u8>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.calibration_key(&key_indexs)?;
    Ok(())
}

#[tauri::command]
pub fn clear_config_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.clear_hall_config()?;
    d.clear_key_config()?;
    Ok(())
}

#[tauri::command]
pub fn reset_device_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.reset_device()?;
    Ok(())
}

#[tauri::command]
pub async fn get_debug_value_part_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>, index: u8) -> Result<Vec<KeyRTStatus>> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    Ok(d.get_debug_value_part(index)?.to_vec())
}

#[tauri::command]
pub async fn get_debug_value_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<Vec<KeyRTStatus>> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    Ok(d.get_debug_value()?.to_vec())
}

#[tauri::command]
pub async fn get_hall_config_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<Vec<KeyHallConfig>> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    Ok(d.get_hall_config()?.to_vec())
}

#[tauri::command]
pub async fn get_keystates_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<Vec<KeyState>> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    Ok(d.get_keystates()?.to_vec())
}

#[tauri::command]
pub async fn get_keyvalues_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<Vec<u16>> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    Ok(d.get_keyvalues()?.to_vec())
}

#[tauri::command]
pub async fn get_key_calibrate_status_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<Vec<bool>> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    Ok(d.get_key_calibrate_status()?.to_vec())
}

#[tauri::command]
pub fn erase_firmware_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.erase_firmware()?;
    Ok(())
}

#[tauri::command]
pub fn get_default_key_config_kb() -> meowboard::config::Device {
    meowboard::cbor::Device::default().try_into().unwrap()
}

#[tauri::command]
pub fn get_key_config_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<meowboard::config::Device> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.load_key_config()?;
    Ok(d.key_config.unwrap().try_into()?)
}

#[tauri::command]
pub fn set_key_config_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>, config: meowboard::config::Device) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.key_config = Some(config.into());
    d.set_key_config()?;
    Ok(())
}


#[tauri::command]
pub fn save_key_config_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.save_key_config()?;
    Ok(())
}


#[tauri::command]
pub fn get_raw_config_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> Result<String> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.load_key_config()?;
    Ok(toml::to_string(&Config {
        key: d.key_config.unwrap().try_into()?,
    })
    .unwrap())
}

#[tauri::command]
pub fn check_raw_config_kb(config: String) -> bool {
    toml::from_str::<Config>(&config).is_ok()
}

#[tauri::command]
pub fn save_raw_config_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>, config: String) -> Result<()> {
    let mut _d = device_handle.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    let cfg = toml::from_str::<Config>(&config).expect("错误配置");
    d.key_config = Some(cfg.key.into());
    d.set_key_config()?;
    d.save_key_config()?;
    Ok(())
}


#[tauri::command]
pub fn connect_kb(device_handle: State<'_, Mutex<Option<Meowboard<HidDevice>>>>) -> bool {
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

fn find_device() -> Option<Meowboard<HidDevice>> {
    // 获取设备列表
    let api = HidApi::new().unwrap();

    // 期望的设备VID和PID
    const VID: u16 = 0x2E3C;
    const PID: u16 = 0x5745;

    // 迭代设备列表，查找符合条件的设备
    let mut devices = api.device_list();
    devices.find_map(|d| {
        // 过滤设备
        if !(d.vendor_id() == VID && d.product_id() == PID) {
            return None;
        }

        // 连接设备
        let device_handle = match d.open_device(&api) {
            Ok(d) => Meowboard::new(HidDevice { device: d }),
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

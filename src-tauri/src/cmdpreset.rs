use std::sync::Mutex;

use hidapi::HidApi;
use tauri::{api::dialog::blocking::FileDialogBuilder, State};
use meowpad::Device;
use meowpad3k::Meowpad as Meowpad3k;
use meowpad4k::Meowpad as Meowpad4k;
use meowboard::Meowboard;
use crate::{device::HidDevice, device_preset::DevicePreset};

// #[tauri::command]
// fn load_preset(
//     device_handle_4k: State<'_, Mutex<Option<Meowpad4k<HidDevice>>>>,
//     device_handle_3k: State<'_, Mutex<Option<Meowpad3k<HidDevice>>>>,
//     device_handle_pure64: State<'_, Mutex<Option<Meowboard<HidDevice>>>>,
// ) {
//     let api = api_handle.lock().unwrap();
//     // 在执行扫描前先锁住设备，不让其他线程访问
//     let mut device_handle_4k = device_handle_4k.lock().unwrap();
//     let mut device_handle_3k = device_handle_3k.lock().unwrap();
//     let mut device_handle_pure64 = device_handle_pure64.lock().unwrap();

// }

#[tauri::command]
pub async fn load_preset_from_file() -> Option<DevicePreset> {
    let file_path = FileDialogBuilder::new().add_filter("desu.life Config File", &["pcf"]).pick_file();

    if let Some(file_path) = file_path {
        let content = std::fs::read_to_string(file_path).ok()?;
        Some(serde_json::from_str(&content).ok()?)
    } else {
        None
    }
}

#[tauri::command]
pub async fn save_preset_to_file(preset: DevicePreset) {
    let file_path = FileDialogBuilder::new().set_file_name(&preset.name).add_filter("desu.life Config File", &["pcf"]).save_file();

    if let Some(file_path) = file_path {
        if let Ok(content) = serde_json::to_string(&preset) {
            let _ = std::fs::write(file_path, content);
        }
    }
}

#[tauri::command]
pub fn load_preset_kb(mut config: meowboard::config::Device, preset: DevicePreset) -> meowboard::config::Device {
    if let Some(d) = preset.config.key_configs {
        if d.len() == 64 {
            for (i, &k) in d.iter().enumerate() {
                config.keys[i] = k.into();
            }
        }
    }

    if let Some(d) = preset.config.key_layers {
        for (i, l) in d.iter().enumerate() {
            if l.keys.len() == 64 {
                for (j, &k) in l.keys.iter().enumerate() {
                    if i == 0 {
                        config.normal_layer[j] = k;
                    } else if i == 1 {
                        config.fn_layer[j] = k;
                    }
                }
            }
        }
    }

    config
}

#[tauri::command]
pub fn gen_preset_kb(name: &str, config: meowboard::config::Device) -> DevicePreset {
    DevicePreset::new(name, "Pure64", config)
}

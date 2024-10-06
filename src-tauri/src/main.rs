#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Result as AnyResult;
use device::{DeviceInfoExtened, DeviceInfoSerdi};
use hid_iap::iap::IAP;
use hidapi::HidApi;
use log::*;
use meowboard::Meowboard;
use meowpad::Device;
use meowpad3k::Meowpad as Meowpad3k;
use meowpad4k::Meowpad as Meowpad4k;
use reqwest::Client;
use serde::Serialize;
use std::borrow::BorrowMut;
use std::env;
use std::io::Write;
use std::ops::Deref;
use std::panic;
use std::str::FromStr;
use std::sync::{mpsc, Mutex};
use std::time::Duration;
use tauri::api::dialog::MessageDialogBuilder;
use tauri::Manager;
use tauri::State;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::LogTarget;

mod cmd3k;
mod cmd4k;
mod cmdiap;
mod cmdkbd;
mod consts;
mod device;
mod error;
mod utils;
mod device_preset;
mod cmdpreset;
use cmd3k::*;
use cmd4k::*;
use cmdiap::*;
use cmdkbd::*;
use cmdpreset::*;
use consts::*;
use error::Result;

use crate::utils::compare_version;

/// blocking_dialog
macro_rules! message_dialog {
    ( $title:expr, $message:expr ) => {{
        use tauri::api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind};
        let (s, r) = mpsc::channel();
        MessageDialogBuilder::new($title, $message)
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Info)
            .show(move |_| s.send(()).unwrap());
        r.recv().unwrap();
    }};
}

/// non_blocking_dialog_with_fn
macro_rules! message_dialog_f {
    ( $title:expr, $message:expr, $f:expr ) => {{
        use tauri::api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind};
        MessageDialogBuilder::new($title, $message)
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Info)
            .show($f);
    }};
}

/// non_blocking_dialog_with_fn_yes_no
macro_rules! message_dialog_f_yn {
    ( $title:expr, $message:expr, $f:expr ) => {{
        use tauri::api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind};
        MessageDialogBuilder::new($title, $message)
            .buttons(MessageDialogButtons::YesNo)
            .kind(MessageDialogKind::Info)
            .show($f);
    }};
}

#[tauri::command]
async fn get_theme(_window: tauri::Window) -> &'static str {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            let Ok(settings) = ashpd::desktop::settings::Settings::new().await else {
                return "dark";
            };
            let Ok(color_scheme) = settings.color_scheme().await else {
                return "dark";
            };
            match color_scheme {
                ashpd::desktop::settings::ColorScheme::NoPreference => "dark",
                ashpd::desktop::settings::ColorScheme::PreferDark => "dark",
                ashpd::desktop::settings::ColorScheme::PreferLight => "light",
            }
        } else {
            let Ok(color_scheme) = _window.theme() else {
                return "dark";
            };

            match color_scheme {
                tauri::Theme::Light => "light",
                tauri::Theme::Dark => "dark",
                _ => "dark",
            }
        }
    }
}

#[tauri::command]
async fn get_latest_version(client: State<'_, Client>) -> Result<Version> {
    Ok(Version::get(client.deref()).await?)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Version {
    configurator_version: String,
    download_url: String,
    v2_starter_edition_latest_firmware_version: String,
    v2_starter_edition_firmware_download_url: String,
    v2_standard_edition_latest_firmware_version: String,
    v2_standard_edition_firmware_download_url: String,
}

impl Version {
    async fn get(client: &Client) -> reqwest::Result<Version> {
        client
            .get("https://desu.life/device/configurator_version/v2/")
            .send()
            .await?
            .json::<Version>()
            .await
    }
}

use tauri::api::shell;

use crate::device::HidDevice;

#[tauri::command]
async fn check_update(_window: tauri::Window, version: Version) -> bool {
    if compare_version(VERSION, &version.configurator_version) == std::cmp::Ordering::Less {
        warn!("最新版本信息：\n{:#?}", version);
        // window.hide().unwrap();
        // message_dialog_f!(
        //     "Meowpad Configurator",
        //     "检测到配置器未更新，请下载新版",
        //     move |_| {
        //         shell::open(&window.shell_scope(), version.download_url, None).unwrap();
        //         // window.close().unwrap();
        //     }
        // );
        return true;
    }
    false
}

#[tauri::command]
async fn open_update_url(window: tauri::Window, version: Version, str: String) {
    message_dialog_f_yn!("Meowpad Configurator", &str, move |r| {
        if r {
            shell::open(&window.shell_scope(), version.download_url, None).unwrap();
        }
        // window.close().unwrap();
    });
}

#[tauri::command]
fn device_list(
    api_handle: State<'_, Mutex<HidApi>>,
    device_handle_4k: State<'_, Mutex<Option<Meowpad4k<HidDevice>>>>,
    device_handle_3k: State<'_, Mutex<Option<Meowpad3k<HidDevice>>>>,
    device_handle_pure64: State<'_, Mutex<Option<Meowboard<HidDevice>>>>,
) -> Vec<DeviceInfoSerdi> {
    let api = api_handle.lock().unwrap();
    // 在执行扫描前先锁住设备，不让其他线程访问
    let mut device_handle_4k = device_handle_4k.lock().unwrap();
    let mut device_handle_3k = device_handle_3k.lock().unwrap();
    let mut device_handle_pure64 = device_handle_pure64.lock().unwrap();

    // 扫描设备
    let mut devices = vec![];

    devices.append(&mut cmd4k::find_devices(&api));
    devices.append(&mut cmd3k::find_devices(&api));
    devices.append(&mut cmdkbd::find_devices(&api));
    devices.append(&mut cmdiap::find_devices(&api));

    // 清空已连接设备的缓冲
    if let Some(d) = device_handle_4k.as_mut() {
        let _ = d.device.clear_buffer();
    }
    if let Some(d) = device_handle_3k.as_mut() {
        let _ = d.device.clear_buffer();
    }
    if let Some(d) = device_handle_pure64.as_mut() {
        let _ = d.device.clear_buffer();
    }

    devices.into_iter().map(|x| x.into()).collect()
}

#[tauri::command]
fn refresh_devices(api_handle: State<'_, Mutex<HidApi>>) -> bool {
    let mut api = api_handle.lock().unwrap();

    let devices_old: Vec<hidapi::DeviceInfo> = api.device_list().cloned().collect();

    let _ = api.refresh_devices();

    let device_list = api.device_list();

    let (len, _) = device_list.size_hint();

    if len != devices_old.len() {
        return true;
    }

    for d in device_list {
        if !d.path().is_empty() && devices_old.iter().any(|x| x.path() == d.path()) {
            continue;
        } else if let Some(sn) = d.serial_number() {
            if devices_old.iter().any(|x| x.serial_number() == Some(sn)) {
                continue;
            }
        } else if devices_old
            .iter()
            .any(|x| x.vendor_id() == d.vendor_id() && x.product_id() == d.product_id())
        {
            continue;
        }
        return true;
    }

    false
}

#[tauri::command]
fn connect_device(
    api_handle: State<'_, Mutex<HidApi>>,
    device_handle_iap: State<'_, Mutex<Option<IAP>>>,
    device_handle_4k: State<'_, Mutex<Option<Meowpad4k<HidDevice>>>>,
    device_handle_3k: State<'_, Mutex<Option<Meowpad3k<HidDevice>>>>,
    device_handle_pure64: State<'_, Mutex<Option<Meowboard<HidDevice>>>>,
    device_info: DeviceInfoSerdi,
) -> bool {
    let api = api_handle.lock().unwrap();

    let d = if !device_info.path.as_bytes().is_empty() {
        api.open_path(device_info.path.as_c_str()).ok()
    } else if let Some(sn) = device_info.serial_number {
        api.open_serial(device_info.vendor_id, device_info.product_id, &sn)
            .ok()
    } else {
        api.device_list()
            .find(|x| {
                x.vendor_id() == device_info.vendor_id
                    && x.product_id() == device_info.product_id
                    && x.interface_number() == device_info.interface_number
            })
            .and_then(|d| d.open_device(&api).ok())
    };

    if let Some(d) = d {
        info!("连接到设备");
        if device_info.device_name == MEOWPAD_DEVICE_NAME {
            if device_info.firmware_version == "IAP" {
                *device_handle_iap.lock().unwrap() = Some(IAP::new(d));
            } else {
                *device_handle_4k.lock().unwrap() =
                    Some(Meowpad4k::new(device::HidDevice { device: d }));
            }
        } else if device_info.device_name == MEOWPAD_SE_DEVICE_NAME {
            *device_handle_3k.lock().unwrap() =
                Some(Meowpad3k::new(device::HidDevice { device: d }));
        } else if device_info.device_name == PURE64_DEVICE_NAME {
            *device_handle_pure64.lock().unwrap() =
                Some(Meowboard::new(device::HidDevice { device: d }));
        } else {
            warn!("连接失败，无法找到设备");
            return false;
        }
    } else {
        warn!("连接失败，无法找到设备");
        return false;
    }

    true
}

fn main() -> AnyResult<()> {
    panic::set_hook(Box::new(|e| {
        use better_panic::Settings;
        use std::backtrace::Backtrace;
        let emessage = format!("Unexcepted Error：\n{}\n{}", e, Backtrace::force_capture());
        // eprintln!("{emessage}");
        let handler = Settings::debug()
            .most_recent_first(false)
            .create_panic_handler();
        handler(e);
        message_dialog!("Meowpad Configurator", &emessage);
        std::process::exit(1);
    }));

    // init_logger("INFO");
    let log_level = LevelFilter::from_str(&std::env::var("LOG_LEVEL").unwrap_or_default())
        .unwrap_or(LevelFilter::Info);

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout])
                .level(log_level)
                .level_for(
                    "tao::platform_impl::platform::event_loop::runner",
                    LevelFilter::Error,
                )
                .format(move |out, message, record| {
                    let colors = ColoredLevelConfig::new();

                    out.finish(format_args!(
                        "[{}] {}",
                        colors.color(record.level()),
                        message
                    ))
                })
                .build(),
        )
        .setup(|_app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = _app.get_window("main").unwrap();
                window.open_devtools();
                window.set_fullscreen(false).unwrap();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_theme,
            calibration_key_4k,
            get_debug_value_4k,
            erase_firmware_4k,
            get_default_key_config_4k,
            get_default_light_config_4k,
            get_key_config_4k,
            get_light_config_4k,
            set_key_config_4k,
            set_light_config_4k,
            save_key_config_4k,
            save_light_config_4k,
            clear_config_4k,
            get_raw_config_4k,
            check_raw_config_4k,
            save_raw_config_4k,
            connect_4k,
            reset_device_4k,
            calibration_key_3k,
            get_debug_value_3k,
            erase_firmware_3k,
            get_default_key_config_3k,
            get_default_light_config_3k,
            get_key_config_3k,
            get_light_config_3k,
            set_key_config_3k,
            set_light_config_3k,
            save_key_config_3k,
            save_light_config_3k,
            clear_config_3k,
            get_raw_config_3k,
            check_raw_config_3k,
            save_raw_config_3k,
            connect_3k,
            reset_device_3k,
            get_device_info_3k,
            get_device_status_3k,
            get_device_info_4k,
            get_device_status_4k,
            get_latest_version,
            get_firmware_4k_version,
            get_firmware_3k_version,
            check_update,
            connect_iap,
            iap_start,
            iap_flush,
            connect_kb,
            get_device_status_kb,
            calibration_key_kb,
            clear_config_kb,
            reset_device_kb,
            get_debug_value_kb,
            erase_firmware_kb,
            get_default_key_config_kb,
            get_key_config_kb,
            set_key_config_kb,
            save_key_config_kb,
            clear_config_kb,
            get_device_status_kb,
            get_keystates_kb,
            get_keyvalues_kb,
            get_device_info_kb,
            get_raw_config_kb,
            check_raw_config_kb,
            save_raw_config_kb,
            get_firmware_kb_version,
            get_key_calibrate_status_kb,
            get_debug_value_part_kb,
            get_hall_config_kb,
            open_update_url,
            device_list,
            refresh_devices,
            connect_device,
            load_preset_kb,
            gen_preset_kb,
            load_preset_from_file,
            save_preset_to_file
        ])
        .manage(
            Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        )
        .manage::<Mutex<Option<Meowpad3k<HidDevice>>>>(Mutex::new(None))
        .manage::<Mutex<Option<Meowpad4k<HidDevice>>>>(Mutex::new(None))
        .manage::<Mutex<Option<Meowboard<HidDevice>>>>(Mutex::new(None))
        .manage::<Mutex<Option<IAP>>>(Mutex::new(None))
        .manage::<Mutex<HidApi>>(Mutex::new(HidApi::new().unwrap()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

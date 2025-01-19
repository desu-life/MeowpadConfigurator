#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Result as AnyResult;
use device::DeviceInfoSerdi;
use hid_iap::iap::IAP;
use hidapi::HidApi;
use log::*;
use meowboard::Meowboard;
use meowpad::Device;
use meowpad3k::Meowpad as Meowpad3k;
use meowpad4k::Meowpad as Meowpad4k;
use reqwest::Client;
use std::env;
use std::ops::Deref;
use std::panic;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Duration;
use tauri::Manager;
use tauri::State;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::FilePath;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_opener::OpenerExt;

mod cmd3k;
mod cmd4k;
mod cmdiap;
mod cmdkbd;
mod cmdpreset;
mod consts;
mod device;
mod device_preset;
mod error;
mod utils;
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
    ( $app:ident, $title:literal, $message:expr ) => {{
        use tauri_plugin_dialog::{MessageDialogButtons, MessageDialogKind};
        $app.dialog()
            .message($message)
            .title($title)
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Info)
            .blocking_show();
    }};
}

/// non_blocking_dialog_with_fn
#[allow(unused_macros)]
macro_rules! message_dialog_f {
    ( $app:ident, $title:literal, $message:expr, $f:expr ) => {{
        use tauri_plugin_dialog::{MessageDialogButtons, MessageDialogKind};
        $app.dialog()
            .message($message)
            .title($title)
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Info)
            .show($f);
    }};
}

/// non_blocking_dialog_with_fn_yes_no
macro_rules! message_dialog_f_yn {
    ( $app:ident, $title:literal, $message:expr, $f:expr ) => {{
        use tauri_plugin_dialog::{MessageDialogButtons, MessageDialogKind};
        $app.dialog()
            .message($message)
            .title($title)
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
async fn get_latest_version(client: State<'_, Client>) -> Result<Vec<Version>> {
    Ok(Version::get(client.deref()).await?)
}

#[tauri::command]
fn update_firmware_call(handle: tauri::AppHandle) {
    use std::process::Command;

    tauri::async_runtime::spawn(async move {
        let file_path = handle
            .dialog()
            .file()
            .add_filter("Firmware File", &["hex"])
            .blocking_pick_file();

        if let Some(FilePath::Path(file_path)) = file_path {
            let resource_path = handle
                .path()
                .resolve(
                    "resources/FirmwareUpdater.exe",
                    tauri::path::BaseDirectory::Resource,
                )
                .expect("failed to resolve resource");

            warn!("resource_path: {:#?}", resource_path);

            Command::new(resource_path)
                .args([file_path])
                .spawn()
                .expect("failed to execute process")
                .wait()
                .expect("process failed");
        }
    });
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Version {
    version: String,
    infomation: VersionInfo,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct VersionInfo {
    notes: String,
    date: String,
    platforms: VersionPlatforms,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct VersionPlatforms {
    #[serde(rename = "macos-app")]
    macos: Option<VersionPlatform>,
    #[serde(rename = "linux-appimage")]
    linux: Option<VersionPlatform>,
    #[serde(rename = "windows-x86_64")]
    windows: Option<VersionPlatform>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct VersionPlatform {
    hash: String,
    url: String,
}

impl Version {
    async fn get(client: &Client) -> reqwest::Result<Vec<Version>> {
        client
            .get("https://assets.desu.life/device/app/")
            .query(&[("from", "a839dd451602fabdff70d25acc70cda3")])
            .send()
            .await?
            .json::<Vec<Version>>()
            .await
    }
}

use crate::device::HidDevice;

#[tauri::command]
async fn check_update(_window: tauri::Window, mut version: Vec<Version>) -> bool {
    version.sort_by(|a, b| compare_version(&b.version, &a.version));
    if let Some(version) = version.first() {
        if compare_version(VERSION, &version.version) == std::cmp::Ordering::Less {
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
    }
    false
}

#[tauri::command]
async fn open_update_url(app: tauri::AppHandle, version: Version, str: String) {
    message_dialog_f_yn!(app, "Meowpad Configurator", &str, move |r| {
        if r {
            let _ = app
                .opener()
                .open_url("https://desu.life/#device", None::<&str>);
        }
        // window.close().unwrap();
    });
}

#[tauri::command]
async fn device_list(
    api_handle: State<'_, Mutex<HidApi>>,
    device_handle_4k: State<'_, Mutex<Option<Meowpad4k<HidDevice>>>>,
    device_handle_3k: State<'_, Mutex<Option<Meowpad3k<HidDevice>>>>,
    device_handle_pure64: State<'_, Mutex<Option<Meowboard<HidDevice>>>>,
) -> Result<Vec<DeviceInfoSerdi>> {
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
    devices.append(&mut cmdiap::find_devices_pure(&api));

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

    Ok(devices.into_iter().map(|x| x.into()).collect())
}

#[tauri::command]
async fn refresh_devices(api_handle: State<'_, Mutex<HidApi>>) -> Result<bool> {
    let mut api = api_handle.lock().unwrap();

    let devices_old: Vec<hidapi::DeviceInfo> = api.device_list().cloned().collect();

    let _ = api.refresh_devices();

    let device_list = api.device_list();

    let (len, _) = device_list.size_hint();

    if len != devices_old.len() {
        return Ok(true);
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
        return Ok(true);
    }

    Ok(false)
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
    // init_logger("INFO");
    let log_level = LevelFilter::from_str(&std::env::var("LOG_LEVEL").unwrap_or_default())
        .unwrap_or(LevelFilter::Info);

    let mut builder = tauri::Builder::default();
    
    builder = builder.setup(|app| {
        let handle = app.handle().clone();
        panic::set_hook(Box::new(move |e| {
            use better_panic::Settings;
            use std::backtrace::Backtrace;
            let emessage = format!("Unexcepted Error：\n{}\n{}", e, Backtrace::force_capture());
            // eprintln!("{emessage}");
            let handler = Settings::debug()
                .most_recent_first(false)
                .create_panic_handler();
            handler(e);
            message_dialog!(handle, "Meowpad Configurator", &emessage);
            std::process::exit(1);
        }));

        #[cfg(debug_assertions)] // only include this code on debug builds
        {
            let window = app.get_webview_window("main").unwrap();
            window.open_devtools();
            window.set_fullscreen(false).unwrap();
        }
        Ok(())
    });

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app.get_webview_window("main")
                       .expect("no main window")
                       .set_focus();
        }));
    }

    builder
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    }),
                ])
                .max_file_size(50_000 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
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
            save_preset_to_file,
            update_firmware_call
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

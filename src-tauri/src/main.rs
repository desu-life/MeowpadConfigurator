// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

use anyhow::Result as AnyResult;
use hid_iap::iap::IAP;
use log::*;
use meowpad::models::DeviceStatus;
use meowpad4k::Meowpad as Meowpad4k;
use meowpad3k::Meowpad as Meowpad3k;
use reqwest::Client;
use std::env;
use std::io::Write;
use std::ops::Deref;
use std::panic;
use std::sync::{mpsc, Mutex};
use std::time::Duration;
use tauri::Manager;
use tauri::State;

mod consts;
mod device;
mod error;
mod utils;
mod cmdiap;
mod cmd4k;
mod cmd3k;
use cmd3k::*;
use cmd4k::*;
use cmdiap::*;
use consts::*;
use error::{Error, Result};

use crate::utils::compare_version;

fn init_logger(default_level: &str) {
    use env_logger::{Builder, Env};
    let log_level = Env::default().filter_or("LOG_LEVEL", default_level);
    let mut builder = Builder::from_env(log_level);
    builder
        .format(|buf, record| {
            let style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "[{style}{}{style:#}] {}",
                record.level(),
                record.args()
            )
        })
        .init();
}

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

#[tauri::command]
async fn get_device_info(app: tauri::AppHandle) -> Result<serde_json::Value> {
    let __d = app.state::<Mutex<Option<Meowpad4k<HidDevice>>>>();
    let mut _d = __d.lock().unwrap();
    if let Some(d) = _d.as_mut() {
        d.get_device_name()?;
        d.get_firmware_version()?;
        let name = d.device_name.as_ref().expect("参数错误");
        let version = d.firmware_version.as_ref().expect("参数错误");
        info!("设备名称：{}", name);
        info!("固件版本：{}", version);
        return Ok(serde_json::json!({
            "name": name,
            "version": version
        }));
    }
    let __d = app.state::<Mutex<Option<Meowpad3k<HidDevice>>>>();
    let mut _d = __d.lock().unwrap();
    if let Some(d) = _d.as_mut() {
        d.get_device_name()?;
        d.get_firmware_version()?;
        let name = d.device_name.as_ref().expect("参数错误");
        let version = d.firmware_version.as_ref().expect("参数错误");
        info!("设备名称：{}", name);
        info!("固件版本：{}", version);
        return Ok(serde_json::json!({
            "name": name,
            "version": version
        }));
    }
    Err(Error::DeviceDisconnected)
}

#[tauri::command]
async fn get_device_status(app: tauri::AppHandle) -> Result<DeviceStatus> {
    let __d = app.state::<Mutex<Option<Meowpad4k<HidDevice>>>>();
    let mut _d = __d.lock().unwrap();
    if let Some(d) = _d.as_mut() {
        let status = d.get_status()?;
        info!(
            "按键配置状态: {}，灯光配置状态: {}，按键校准状态: {}，按键是否启用: {}",
            status.key, status.light, status.hall, status.enabled
        );
        return Ok(status);
    }
    let __d = app.state::<Mutex<Option<Meowpad3k<HidDevice>>>>();
    let mut _d = __d.lock().unwrap();
    if let Some(d) = _d.as_mut() {
        let status = d.get_status()?;
        info!(
            "按键配置状态: {}，灯光配置状态: {}，按键校准状态: {}，按键是否启用: {}",
            status.key, status.light, status.hall, status.enabled
        );
        return Ok(status);
    }
    Err(Error::DeviceDisconnected)
}

#[tauri::command]
async fn get_latest_version(client: State<'_, Client>) -> Result<Version> {
    Ok(Version::get(client.deref()).await?)
}

#[tauri::command]
async fn get_firmware_4k_version(_app: tauri::AppHandle) -> &'static str {
    FIRMWARE_VERSION_4K
}

#[tauri::command]
async fn get_firmware_3k_version(_app: tauri::AppHandle) -> &'static str {
    FIRMWARE_VERSION_3K
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
async fn check_update(window: tauri::Window, version: Version) -> bool {
    if compare_version(VERSION, &version.configurator_version) == std::cmp::Ordering::Less {
        warn!("最新版本信息：\n{:#?}", version);
        window.hide().unwrap();
        message_dialog_f!(
            "Meowpad Configurator",
            "检测到配置器未更新，请下载新版",
            move |_| {
                shell::open(&window.shell_scope(), version.download_url, None).unwrap();
                window.close().unwrap();
            }
        );
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

    init_logger("INFO");

    tauri::Builder::default()
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
            get_raw_config_4k,
            check_raw_config_4k,
            save_raw_config_4k,
            connect_4k,
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
            get_raw_config_3k,
            check_raw_config_3k,
            save_raw_config_3k,
            connect_3k,
            get_device_info,
            get_device_status,
            get_latest_version,
            get_firmware_4k_version,
            get_firmware_3k_version,
            check_update,
            connect_iap,
            iap_start,
            iap_flush,
        ])
        .manage(
            Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        )
        .manage::<Mutex<Option<Meowpad3k<HidDevice>>>>(Mutex::new(None))
        .manage::<Mutex<Option<Meowpad4k<HidDevice>>>>(Mutex::new(None))
        .manage::<Mutex<Option<IAP>>>(Mutex::new(None))
        .manage::<Mutex<Option<Vec<u8>>>>(Mutex::new(None))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

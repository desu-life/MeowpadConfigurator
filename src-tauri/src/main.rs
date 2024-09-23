// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

use anyhow::Result as AnyResult;
use hid_iap::iap::IAP;
use log::*;
use meowboard::Meowboard;
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
mod cmdkbd;
use cmd3k::*;
use cmd4k::*;
use cmdkbd::*;
use cmdiap::*;
use consts::*;
use error::Result;

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
            get_hall_config_kb
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
        .manage::<Mutex<Option<Vec<u8>>>>(Mutex::new(None))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

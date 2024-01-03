// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

use anyhow::{anyhow, Result as AnyResult};
use hidapi_rusb::HidApi;
use log::*;
use meowpad::{Config, Meowpad, KeyRTStatus};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde_json::json;
use std::borrow::BorrowMut;
use std::env;
use std::ffi::CStr;
use std::fs;
use std::io::Write;
use std::panic;
use std::path::PathBuf;
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::api::path;
use tauri::Manager;
use tauri::Window;

mod consts;
mod error;
use consts::*;
use error::{Error, Result};

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap()
});
static mut HID_API: Lazy<HidApi> = Lazy::new(|| HidApi::new().unwrap());
static DEVICE: Mutex<Option<Meowpad>> = Mutex::new(None);

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

fn init_logger(default_level: &str) {
    use env_logger::{Builder, Env};
    let mut builder = Builder::from_env(Env::default().filter_or("LOG_LEVEL", default_level));
    builder
        .format(|buf, record| {
            let level = record.level();
            writeln!(
                buf,
                "[{}] {}",
                buf.default_level_style(level).value(level),
                record.args()
            )
        })
        .init();
}

#[tauri::command]
async fn calibration_key(_app: tauri::AppHandle) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDissconnected)?;
    d.calibration_key()?;
    Ok(())
}

// #[tauri::command]
// async fn get_calibration_key_result(_app: tauri::AppHandle, timeout: i32) -> Result<(), String> {
//     || -> AnyResult<_> {
//         let mut _d = DEVICE.lock().unwrap();
//         let d = _d.as_mut().ok_or_else(|| anyhow!("获取设备失败"))?;
//         d.get_calibration_key_result(timeout)?;
//         Ok(())
//     }()
//     .map_err(|e| format!("{}", e))
// }

// #[tauri::command]
// async fn get_auto_config(_app: tauri::AppHandle) -> Result<serde_json::Value, String> {
//     || -> AnyResult<_> {
//         let mut _d = DEVICE.lock().unwrap();
//         let d = _d.as_mut().ok_or_else(|| anyhow!("获取设备失败"))?;
//         let res = d.get_auto_config()?;
//         Ok(json!({
//             "KeyTriggerDegree": res.0,
//             "KeyReleaseDegree": res.1,
//             "DeadZone": res.2,
//         }))
//     }()
//     .map_err(|e| format!("{}", e))
// }

#[tauri::command]
async fn get_hall_value(_window: Window) -> Result<[KeyRTStatus; 4]> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDissconnected)?;
    Ok(d.debug()?)
}

#[tauri::command]
async fn erase_firmware(_app: tauri::AppHandle) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDissconnected)?;
    d.erase_firmware()?;
    Ok(())
}

#[tauri::command]
async fn get_default_config(_app: tauri::AppHandle) -> Config {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().unwrap();
    let config = d.default_config();
    config.try_into().expect("解析默认配置出错，真不应该..")
}

#[tauri::command]
async fn get_config(_app: tauri::AppHandle) -> Result<Config> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDissconnected)?;
    d.load_config()?;
    Ok(d.config().try_into()?)
}

#[tauri::command]
async fn get_raw_config(_app: tauri::AppHandle) -> Result<String> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDissconnected)?;
    d.load_config()?;
    Ok(toml::to_string::<Config>(&d.config().try_into()?).unwrap())
}

#[tauri::command]
async fn check_raw_config(_app: tauri::AppHandle, config: String) -> bool {
    toml::from_str::<meowpad::Config>(&config).is_ok()
}

#[tauri::command]
async fn save_raw_config(_app: tauri::AppHandle, config: String) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDissconnected)?;
    d.config = Some(toml::from_str::<Config>(&config).expect("错误配置").into());
    d.write_config()?;
    Ok(())
}

#[tauri::command]
async fn get_device_info(_app: tauri::AppHandle) -> Result<serde_json::Value> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDissconnected)?;
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
async fn save_config(_app: tauri::AppHandle, config: Config) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDissconnected)?;
    d.config = Some(config.into());
    d.write_config()?;
    Ok(())
}

#[tauri::command]
async fn get_version(_app: tauri::AppHandle) -> Result<Version> {
    Ok(Version::get().await?)
}

#[tauri::command]
async fn get_firmware_version(_app: tauri::AppHandle) -> &'static str {
    FIRMWARE_VERSION_4K
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
    async fn get() -> reqwest::Result<Version> {
        CLIENT
            .get("https://desu.life/device/configurator_version/v2/")
            .send()
            .await?
            .json::<Version>()
            .await
    }
}

use tauri::api::shell;

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

#[tauri::command]
async fn connect(_app: tauri::AppHandle) -> Result<()> {
    _connect()
}

fn _connect() -> Result<()> {
    info!("开始连接!");
    let found_device = find_device();

    match found_device {
        Some(device) => {
            let mut _d = DEVICE.lock().unwrap();
            *_d = Some(device);
            Ok(())
        }
        None => {
            warn!("连接失败，无法找到设备");
            Err(error::Error::DeviceNotFound)
        }
    }
}

fn find_device() -> Option<Meowpad> {
    // 获取设备列表
    let api = unsafe { HID_API.borrow_mut() };
    api.refresh_devices().unwrap();

    // 期望的设备VID和PID
    const VID: u16 = 0x5D3E;
    const PID: u16 = 0xFE07;

    // 缓存路径
    // let cache_path = path::cache_dir()
    //     .map(|mut p| {
    //         p.push(".meowkey");
    //         p
    //     })
    //     .unwrap_or_else(|| PathBuf::from(".meowkey"));

    // 迭代设备列表，查找符合条件的设备
    api.device_list().find_map(|d| {
        // 过滤设备
        if !(d.vendor_id() == VID && d.product_id() == PID) {
            return None;
        }

        // 连接设备
        let device = match d.open_device(api) {
            Ok(d) => Meowpad::new(d),
            Err(_) => return None,
        };

        // 测试连接
        match device.ping() {
            Ok(r) if !r => None,
            Err(_) => None,
            _ => {
                info!("连接到设备");
                debug!("Name: {}", d.product_string().unwrap_or_default());
                debug!(
                    "Manufacturer: {}",
                    d.manufacturer_string().unwrap_or_default()
                );
                debug!("Addr: {}", d.path().to_string_lossy());
                debug!("{:?}", d);
                Some(device)
            }
        }
    })
}

pub fn compare_version(version1: &str, version2: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;

    // 检查版本号的格式是否正确
    let re = regex::Regex::new(r"^\d+(\.\d+)*$").unwrap();
    if !re.is_match(version1) || !re.is_match(version2) {
        panic!("Invalid version format");
    }

    // 将版本号转换为数字向量
    let v1: Vec<u64> = version1.split('.').map(|s| s.parse().unwrap()).collect();
    let v2: Vec<u64> = version2.split('.').map(|s| s.parse().unwrap()).collect();

    // 比较数字向量
    for i in 0..std::cmp::max(v1.len(), v2.len()) {
        let n1 = v1.get(i).unwrap_or(&0);
        let n2 = v2.get(i).unwrap_or(&0);
        match n1.cmp(n2) {
            Greater => return Greater,
            Less => return Less,
            Equal => (),
        }
    }

    // 版本号相等
    Equal
}

fn main() -> AnyResult<()> {
    panic::set_hook(Box::new(|e| {
        use std::backtrace::Backtrace;
        use better_panic::Settings;
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

    let action = env::args().nth(1).unwrap_or_default();
    match action.as_str() {
        "--reset" => {
            _connect()?;
            let mut _d = DEVICE.lock().unwrap();
            let mut d = _d.take().unwrap();
            d.get_device_name()?;
            d.get_firmware_version()?;
            info!("设备名称：{:?}", d.device_name);
            info!("固件版本：{:?}", d.firmware_version);
            d.reset_config()?;
            warn!("重置配置成功")
        }
        "--erase" => {
            _connect()?;
            let mut _d = DEVICE.lock().unwrap();
            let mut d = _d.take().unwrap();
            d.get_device_name()?;
            d.get_firmware_version()?;
            info!("设备名称：{:?}", d.device_name);
            info!("固件版本：{:?}", d.firmware_version);
            d.erase_firmware()?;
            warn!("重置固件成功")
        }
        "--set-middle-point" => {
            _connect()?;
            let mut _d = DEVICE.lock().unwrap();
            let mut d = _d.take().unwrap();
            d.get_device_name()?;
            d.get_firmware_version()?;
            info!("设备名称：{:?}", d.device_name);
            info!("固件版本：{:?}", d.firmware_version);
            d.reset_middle_point()?;
            warn!("设置中点")
        }
        "--console" => {
            _connect()?;
            let mut _d = DEVICE.lock().unwrap();
            let mut d = _d.take().unwrap();
            d.get_device_name()?;
            d.get_firmware_version()?;
            info!("设备名称：{:?}", d.device_name);
            info!("固件版本：{:?}", d.firmware_version);
            d.load_config().unwrap();
            info!("当前设备配置：{:?}", d.config());
            let mut f = std::fs::File::create("meowpad.toml")?;
            f.write_all(toml::to_string_pretty(&d.config())?.as_bytes())?;

            let (tx, rx) = mpsc::channel();
            let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
            watcher
                .watch("meowpad.toml", RecursiveMode::NonRecursive)
                .unwrap();

            warn!("开始监听配置文件（meowpad.toml）如要修改配置，修改文件保存即可。");

            loop {
                match rx.recv() {
                    Ok(DebouncedEvent::Write(_)) => {
                        warn!(" * 检测到配置文件更新，自动写入中 ...");
                        d.config.iter_mut().for_each(|c| {
                            *c = toml::from_str(&fs::read_to_string("meowpad.toml").unwrap())
                                .unwrap();
                        });
                        d.write_config()?;
                    }
                    Err(e) => error!("watch error: {:?}", e),
                    _ => (),
                }
            }
        }
        _ => {
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
                    connect,
                    get_config,
                    save_config,
                    get_default_config,
                    get_device_info,
                    check_update,
                    get_version,
                    get_firmware_version,
                    calibration_key,
                    erase_firmware,
                    get_raw_config,
                    save_raw_config,
                    check_raw_config,
                    get_hall_value
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
    }
    Ok(())
}

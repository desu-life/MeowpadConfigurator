// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

use anyhow::Result as AnyResult;
use hid_iap::iap::IAPState;
use hid_iap::iap::IAP;
use hidapi::HidApi;
use log::*;
use meowpad::models::{DeviceStatus, KeyRTStatus};
use meowpad4k::Meowpad;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use reqwest::Client;
use std::env;
use std::fs;
use std::io::Write;
use std::panic;
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Manager;
use tauri::Window;
use tauri::State;
use std::ops::Deref;

mod consts;
mod device;
mod error;
mod utils;
use consts::*;
use error::{Error, Result};

use crate::utils::compare_version;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Copy)]
struct Config {
    key: meowpad4k::config::Key,
    light: meowpad4k::config::Light,
}

static DEVICE: Mutex<Option<Meowpad<HidDevice>>> = Mutex::new(None);
static IAP_DEVICE: Mutex<Option<IAP>> = Mutex::new(None);
static mut FIRMWARE_DATA: Option<Vec<u8>> = None;

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
async fn calibration_key(_app: tauri::AppHandle) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.calibration_key()?;
    Ok(())
}

#[tauri::command]
async fn get_debug_value(_window: Window) -> Result<[KeyRTStatus; 4]> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    Ok(d.get_debug_value()?)
}

#[tauri::command]
async fn erase_firmware(_app: tauri::AppHandle) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.erase_firmware()?;
    Ok(())
}

#[tauri::command]
async fn get_default_key_config(_app: tauri::AppHandle) -> meowpad4k::config::Key {
    meowpad4k::cbor::Keyboard::default().try_into().unwrap()
}

#[tauri::command]
async fn get_default_light_config(_app: tauri::AppHandle) -> meowpad4k::config::Light {
    meowpad4k::cbor::Light::default().try_into().unwrap()
}

#[tauri::command]
async fn get_key_config(_app: tauri::AppHandle) -> Result<meowpad4k::config::Key> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.load_key_config()?;
    Ok(d.key_config.unwrap().try_into()?)
}

#[tauri::command]
async fn get_light_config(_app: tauri::AppHandle) -> Result<meowpad4k::config::Light> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.load_light_config()?;
    Ok(d.light_config.unwrap().try_into()?)
}

#[tauri::command]
async fn set_key_config(_app: tauri::AppHandle, config: meowpad4k::config::Key) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.key_config = Some(config.into());
    d.set_key_config()?;
    Ok(())
}

#[tauri::command]
async fn set_light_config(_app: tauri::AppHandle, config: meowpad4k::config::Light) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.light_config = Some(config.into());
    d.set_light_config()?;
    Ok(())
}

#[tauri::command]
async fn save_key_config(_app: tauri::AppHandle) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.save_key_config()?;
    Ok(())
}

#[tauri::command]
async fn save_light_config(_app: tauri::AppHandle) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    d.save_light_config()?;
    Ok(())
}

#[tauri::command]
async fn get_raw_config(_app: tauri::AppHandle) -> Result<String> {
    let mut _d = DEVICE.lock().unwrap();
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
async fn check_raw_config(_app: tauri::AppHandle, config: String) -> bool {
    toml::from_str::<Config>(&config).is_ok()
}

#[tauri::command]
async fn save_raw_config(_app: tauri::AppHandle, config: String) -> Result<()> {
    let mut _d = DEVICE.lock().unwrap();
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
async fn get_device_info(_app: tauri::AppHandle) -> Result<serde_json::Value> {
    let mut _d = DEVICE.lock().unwrap();
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
async fn get_device_status(_app: tauri::AppHandle) -> Result<DeviceStatus> {
    let mut _d = DEVICE.lock().unwrap();
    let d = _d.as_mut().ok_or(Error::DeviceDisconnected)?;
    let status = d.get_status()?;
    info!(
        "按键配置状态: {}，灯光配置状态: {}，按键校准状态: {}，按键是否启用: {}",
        status.key, status.light, status.hall, status.enabled
    );
    Ok(status)
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

#[tauri::command]
async fn connect(_app: tauri::AppHandle) -> Result<()> {
    _connect()
}

#[tauri::command]
async fn connect_iap(_app: tauri::AppHandle) -> Result<()> {
    let vid = 0x5D3E;
    let pid = 0xFE08;
    let api = HidApi::new().unwrap();

    match api.open(vid, pid) {
        Ok(device) => {
            info!("固件更新");
            let mut _d = IAP_DEVICE.lock().unwrap();
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
async fn iap_start(_app: tauri::AppHandle, data: Vec<u8>) -> Result<usize> {
    let mut _iap = IAP_DEVICE.lock().unwrap();
    let iap = _iap.as_mut().ok_or(Error::DeviceDisconnected)?;
    unsafe {
        FIRMWARE_DATA = Some(data);
        let data_ref = FIRMWARE_DATA.as_deref().unwrap();
        let len = iap.start_program(data_ref)?;
        Ok(len)
    }
}

#[tauri::command]
async fn iap_flush(app: tauri::AppHandle) -> Result<()> {
    let mut _iap = IAP_DEVICE.lock().unwrap();
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

fn _connect() -> Result<()> {
    info!("开始连接!");
    let found_device = find_device();

    match found_device {
        Some(device) => {
            info!("连接到设备");
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

fn find_device() -> Option<Meowpad<HidDevice>> {
    // 获取设备列表
    let api = HidApi::new().unwrap();

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
    let mut devices = api.device_list();
    devices.find_map(|d| {
        // 过滤设备
        if !(d.vendor_id() == VID && d.product_id() == PID) {
            return None;
        }

        // 连接设备
        let device = match d.open_device(&api) {
            Ok(d) => Meowpad::new(HidDevice { device: d }),
            Err(_) => return None,
        };

        // 测试连接
        match device.ping() {
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
                Some(device)
            }
        }
    })
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

            d.key_config = Some(meowpad4k::cbor::Keyboard::default());
            d.set_key_config()?;
            d.save_key_config()?;

            d.light_config = Some(meowpad4k::cbor::Light::default());
            d.set_light_config()?;
            d.save_light_config()?;
            warn!("重置配置成功")
        }
        "--clear" => {
            _connect()?;
            let mut _d = DEVICE.lock().unwrap();
            let mut d = _d.take().unwrap();
            d.get_device_name()?;
            d.get_firmware_version()?;
            info!("设备名称：{:?}", d.device_name);
            info!("固件版本：{:?}", d.firmware_version);

            d.clear_key_config()?;
            d.clear_light_config()?;
            d.clear_hall_config()?;
            d.reset_device()?;
            warn!("设备配置已清空");
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
            let status = d.get_status()?;
            info!(
                "按键配置状态: {}，灯光配置状态: {}，按键校准状态: {}，按键是否启用: {}",
                status.key, status.light, status.hall, status.enabled
            );

            d.load_key_config()?;
            d.load_light_config()?;

            info!("当前按键配置：{:#?}", d.key_config.unwrap());
            info!("当前灯效配置：{:#?}", d.light_config.unwrap());

            let mut f = std::fs::File::create("meowpad.toml")?;
            f.write_all(
                toml::to_string(&Config {
                    key: d.key_config.unwrap().try_into()?,
                    light: d.light_config.unwrap().try_into()?,
                })?
                .as_bytes(),
            )?;

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
                        if let Ok(cfg) =
                            toml::from_str::<Config>(&fs::read_to_string("meowpad.toml")?)
                        {
                            d.key_config = Some(cfg.key.into());
                            d.set_key_config()?;
                            d.save_key_config()?;
                            d.light_config = Some(cfg.light.into());
                            d.set_light_config()?;
                            d.save_light_config()?;
                        } else {
                            warn!(" * 配置文件格式错误，请修改");
                            continue;
                        }
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
                    calibration_key,
                    get_debug_value,
                    erase_firmware,
                    get_default_key_config,
                    get_default_light_config,
                    get_key_config,
                    get_light_config,
                    set_key_config,
                    set_light_config,
                    save_key_config,
                    save_light_config,
                    get_raw_config,
                    check_raw_config,
                    save_raw_config,
                    get_device_info,
                    get_device_status,
                    get_latest_version,
                    get_firmware_4k_version,
                    get_firmware_3k_version,
                    check_update,
                    connect_iap,
                    iap_start,
                    iap_flush,
                    connect
                ])
                .manage(
                    Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap(),
                )
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
    }
    Ok(())
}

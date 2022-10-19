// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

use anyhow::{anyhow, Result as AnyResult};
use hidapi::HidApi;
use log::*;
use meowpad::*;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use std::env;
use std::fs;
use std::io::Write;
use std::sync::{mpsc, Mutex, RwLock};
use std::time::Duration;

static HID_API: RwLock<Lazy<HidApi>> = RwLock::new(Lazy::new(|| HidApi::new().unwrap()));
static DEVICE: Mutex<Option<Meowpad>> = Mutex::new(None);

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
fn get_default(_app: tauri::AppHandle) -> Result<Config, String> {
    match || -> AnyResult<Config> {
        let config = cbor::CONFIG::default();
        config.try_into()
    }() {
        Ok(cfg) => Ok(cfg),
        Err(e) => Err(format!("{}", e))
    }
}

#[tauri::command]
fn get_config(_app: tauri::AppHandle) -> Result<Config, String> {
    match || -> AnyResult<Config> {
        let _d = DEVICE.lock().unwrap();
        _d.as_ref().ok_or_else(|| anyhow!("获取设备失败"))?.config().try_into()
    }() {
        Ok(cfg) => Ok(cfg),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command]
fn save_config(_app: tauri::AppHandle, config: Config) -> Result<(), String> {
    match || -> AnyResult<()> {
        let mut _d = DEVICE.lock().unwrap();
        let d = _d.as_mut().ok_or_else(|| anyhow!("获取设备失败"))?;
        d.map_config(|c| *c = config.into());
        d.write_config()?;
        d.flush()?;
        Ok(())
    }() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command]
fn connect(_app: tauri::AppHandle) -> Result<(), String> {
    match _connect(false, false) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}

fn _connect(console: bool, reset: bool) -> AnyResult<()> {
    info!("开始连接!");
    let found_device = find_device();

    match found_device {
        Some(mut device) => {
            info!("设备名称：{}", device.get_device_name()?);
            info!("固件版本：{}", device.get_firmware_version()?);
            if reset {
                device.reset_config()?;
                warn!("重置配置成功")
            }
            // device.write_config()?;
            device.load_config()?;
            info!("当前设备配置：{:?}", device.config());

            if console {
                let mut f = std::fs::File::create("meowpad.toml")?;
                f.write_all(&toml::to_vec(&device.config())?)?;
            }

            let mut _d = DEVICE.lock().unwrap();
            *_d = Some(device);
            Ok(())
        }
        None => {
            warn!("连接失败，无法找到设备");
            Err(anyhow!("无法找到设备"))
        }
    }
}

fn find_device() -> Option<Meowpad> {
    // connect
    let mut api = HID_API.write().unwrap();
    api.refresh_devices().unwrap();
    let (vid, pid) = (0x5D3E, 0x7490);
    let meowpad = api
        .device_list()
        .filter(|d| d.vendor_id() == vid && d.product_id() == pid)
        .filter_map(|d| {
            let device = match d.open_device(&api) {
                Ok(d) => Meowpad::new(d),
                Err(_) => return None,
            };
            // 测试连接
            match device.ping() {
                Ok(r) if !r => return None,
                Err(_) => return None,
                _ => (),
            }
            info!("连接到设备");
            debug!("Name: {}", d.product_string().unwrap_or("None"));
            debug!(
                "Manufacturer: {}",
                d.manufacturer_string().unwrap_or("None")
            );
            debug!("Addr: {}", d.path().to_string_lossy());
            debug!("{:?}", d);
            Some(device)
        })
        .next();

    meowpad
}

static version: &str = "0.2.0";

#[derive(serde::Deserialize)]
struct Version {
    version: String,
    download_url: String,
}

fn main() -> AnyResult<()> {
    init_logger("INFO");

    // let resp = || -> AnyResult<Version> {
    //     Ok(reqwest::blocking::get("https://desu.life/device/configurator_version")?
    //         .json::<Version>()?)
    // }();

    // if let Ok(resp) = resp {
    //     if resp.version != version {
    //         warn!("配置器已更新，请使用以下链接下载更新");
    //         warn!("{}", resp.download_url);
    //         warn!("按回车键退出 ↲");
    //         console::Term::stdout().read_line()?;
    //         exit(0);
    //     }
    // }

    let action = env::args().nth(1).unwrap_or_default();
    match action.as_str() {
        "--reset" => _connect(false, true)?,
        "--console" => {
            _connect(true, false)?;
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
                        let mut _d = DEVICE.lock().unwrap();
                        let mut d = _d.take().unwrap();
                        d.map_config(|c| {
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
                        use tauri::Manager;
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
                    get_default
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
    }
    Ok(())
}

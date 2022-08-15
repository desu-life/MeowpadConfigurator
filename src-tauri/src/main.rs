// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

use anyhow::{anyhow, Result as AnyResult};
use hidapi::HidApi;
use log::*;
use meowpad::*;
use std::env;
use std::fs;
use std::io::Write;
use std::sync::{mpsc, Mutex, RwLock};
use std::time::Duration;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::{OnceCell, Lazy};

static HID_API: RwLock<Lazy<HidApi>> = RwLock::new(Lazy::new(|| (HidApi::new().unwrap())));
static DEVICE: Mutex<OnceCell<Meowpad>> = Mutex::new(OnceCell::new());

fn init_logger() {
    use env_logger::{Builder, Env};
    let mut builder = Builder::from_env(Env::default().filter_or("LOG_LEVEL", "INFO"));
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
    let config = Config::default();
    Ok(config)
}

#[tauri::command]
fn get_config(_app: tauri::AppHandle) -> Result<Config, String> {
    match || -> AnyResult<Config> {
        let _d = DEVICE.lock().unwrap();
        Ok(_d.get().ok_or(anyhow!("获取设备失败"))?.config())
    }() {
        Ok(cfg) => Ok(cfg),
        Err(e) => Err(format!("{}", e))
    }
}

#[tauri::command]
fn save_config(_app: tauri::AppHandle, config: Config) -> Result<(), String> {
    match || -> AnyResult<()> {
        let mut _d = DEVICE.lock().unwrap();
        let d = _d.get_mut().ok_or(anyhow!("获取设备失败"))?;
        d.map_config(|c| *c = config);
        d.write_config()?;
        Ok(())
    }() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e))
    }
}

#[tauri::command]
fn connect(_app: tauri::AppHandle) -> Result<(), String> {
    match _connect(false, false) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e))
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

            DEVICE.lock().unwrap().set(device).expect("设置设备连接失败");
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

fn main() -> AnyResult<()> {
    init_logger();

    let action = env::args().skip(1).next().unwrap_or_default();
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
                        let d = _d.get_mut().unwrap();
                        d.map_config(|c| {
                            *c = toml::from_str(&fs::read_to_string("meowpad.toml").unwrap()).unwrap();
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
            .invoke_handler(tauri::generate_handler![connect, get_config, save_config, get_default])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        }
    }
    Ok(())
}

/// Pure64 keyboard device commands for WASM
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use meowboard::Meowboard;
use meowpad::{
    models::{DeviceStatus, KeyHallConfig, KeyRTStatus, KeyState},
    Device,
};
use crate::webhid_device::WebHidDevice;
use crate::get_device_cache;
use std::sync::Arc;

// VID:PID for Pure64
pub const VID: u16 = 0x5D3E;
pub const PID: u16 = 0xFB01;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub version: String,
}

/// Probe device temporarily to get info (without saving to cache)
/// Used for device enumeration
#[wasm_bindgen]
pub async fn pure64_probe_device(hid_device_js: JsValue) -> Result<JsValue, JsValue> {
    log::info!("pure64_probe_device called");
    
    // Create WebHID device wrapper
    let webhid_device = match std::panic::catch_unwind(|| {
        WebHidDevice::new(hid_device_js.clone())
    }) {
        Ok(dev) => dev,
        Err(e) => {
            log::error!("Failed to create WebHidDevice: {:?}", e);
            return Err(JsValue::from_str("Failed to create device wrapper"));
        }
    };
    
    log::info!("WebHidDevice created");
    
    // Keep reference to WebHidDevice for async operations
    // Meowboard is not used for now since its methods are synchronous
    
    // Manually ping device with async read/write
    let mut ping_success = false;
    for i in 0..3 {
        log::debug!("Ping attempt {}", i + 1);
        
        // Send ping packet
        let ping_packet = meowpad::Packet::new(3u8, vec![]);
        
        // Build packet into bytes - need to handle 64-byte chunks
        for chunk in ping_packet.build_packets() {
            match webhid_device.write_async(&chunk).await {
                Ok(_) => {
                    log::debug!("Ping packet chunk sent");
                }
                Err(e) => {
                    log::debug!("Write error: {:?}", e);
                    continue;
                }
            }
        }
        
        // Wait for response with async read
        let mut buf = [0u8; 64];  // 64 bytes data (no report ID)
        match webhid_device.read_timeout_async(&mut buf, 1000).await {
            Ok(size) => {
                log::debug!("Received {} bytes", size);
                // buf[0] = packet ID
                // buf[1..] = packet data
                if size >= 1 {
                    let packet_id = buf[0];
                    log::debug!("Packet ID: {}", packet_id);
                    if packet_id == 3 {  // Ping response
                        log::info!("Ping successful");
                        ping_success = true;
                        break;
                    }
                }
            }
            Err(e) => {
                log::debug!("Read error: {:?}", e);
            }
        }
    }
    
    if !ping_success {
        log::error!("Device did not respond to ping");
        return Err(JsValue::from_str("Device did not respond to ping after retries"));
    }
    
    // Get device name (PacketID::GetDeviceName = 8)
    let mut device_name = "Pure64".to_string();
    log::info!("Getting device name...");
    
    let name_packet = meowpad::Packet::new(8u8, vec![]);
    for chunk in name_packet.build_packets() {
        if let Err(e) = webhid_device.write_async(&chunk).await {
            log::error!("Failed to write device name packet: {:?}", e);
        }
    }
    
    let mut buf = [0u8; 64];
    if let Ok(size) = webhid_device.read_timeout_async(&mut buf, 1000).await {
        if size >= 1 && buf[0] == 1 {  // PacketID::Ok = 1
            // Parse packet: buf[0] = packet ID, buf[1..3] = length (u16 BE), buf[3..] = data
            let data_len = u16::from_be_bytes([buf[1], buf[2]]) as usize;
            if data_len > 0 && size >= 3 + data_len {
                let name_bytes = &buf[3..3 + data_len];
                if let Ok(name) = String::from_utf8(name_bytes.to_vec()) {
                    device_name = name;
                    log::info!("Device name: {}", device_name);
                }
            }
        }
    }
    
    // Get firmware version (PacketID::GetFirmwareVersion = 7)
    let mut firmware_version = "Unknown".to_string();
    log::info!("Getting firmware version...");
    
    let version_packet = meowpad::Packet::new(7u8, vec![]);
    for chunk in version_packet.build_packets() {
        if let Err(e) = webhid_device.write_async(&chunk).await {
            log::error!("Failed to write firmware version packet: {:?}", e);
        }
    }
    
    let mut buf = [0u8; 64];
    if let Ok(size) = webhid_device.read_timeout_async(&mut buf, 1000).await {
        if size >= 1 && buf[0] == 1 {  // PacketID::Ok = 1
            // Parse packet: buf[0] = packet ID, buf[1..3] = length (u16 BE), buf[3..] = data
            let data_len = u16::from_be_bytes([buf[1], buf[2]]) as usize;
            if data_len > 0 && size >= 3 + data_len {
                let version_bytes = &buf[3..3 + data_len];
                if let Ok(version) = String::from_utf8(version_bytes.to_vec()) {
                    firmware_version = version;
                    log::info!("Firmware version: {}", firmware_version);
                }
            }
        }
    }
    
    let info = DeviceInfo {
        name: device_name,
        version: firmware_version,
    };
    
    log::info!("Device info: {:?}", info);
    
    serde_wasm_bindgen::to_value(&info)
        .map_err(|e| {
            log::error!("Serialization error: {}", e);
            JsValue::from_str(&format!("Serialization error: {}", e))
        })
}

/// Helper function to get WebHidDevice from cache
fn get_webhid_device(device_id: u32) -> Result<WebHidDevice, JsValue> {
    let cache_arc = get_device_cache();
    let cache = cache_arc.lock()
        .map_err(|_| JsValue::from_str("Cache lock failed"))?;
    
    cache.get(&device_id)
        .map(|dev| (**dev).clone())
        .ok_or_else(|| JsValue::from_str("Device not found"))
}

/// Get firmware version list
#[wasm_bindgen]
pub fn pure64_get_firmware_version() -> JsValue {
    // Compatible firmware versions for Pure64
    serde_wasm_bindgen::to_value(&["0.1.3"]).unwrap()
}

/// Get device information (name and firmware version)
#[wasm_bindgen]
pub async fn pure64_get_device_info(device_id: u32) -> Result<JsValue, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    meowboard.get_device_name().await
        .map_err(|e| JsValue::from_str(&format!("Failed to get device name: {:?}", e)))?;
    meowboard.get_firmware_version().await
        .map_err(|e| JsValue::from_str(&format!("Failed to get firmware version: {:?}", e)))?;
    
    let info = DeviceInfo {
        name: meowboard.device_name.unwrap_or_default(),
        version: meowboard.firmware_version.unwrap_or_default(),
    };
    
    serde_wasm_bindgen::to_value(&info)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get device status (key config, hall calibration, enabled state)
#[wasm_bindgen]
pub async fn pure64_get_device_status(device_id: u32) -> Result<JsValue, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    
    // Send GetStatus packet (PacketID = 16)
    let status_packet = meowpad::Packet::new(16u8, vec![]);
    for chunk in status_packet.build_packets() {
        if let Err(e) = webhid_device.write_async(&chunk).await {
            log::error!("Failed to write status packet: {:?}", e);
            return Err(JsValue::from_str(&format!("Failed to write: {:?}", e)));
        }
    }
    
    // Read response with timeout
    let mut buf = [0u8; 64];
    match webhid_device.read_timeout_async(&mut buf, 1000).await {
        Ok(size) => {
            log::info!("Read {} bytes for status", size);
            if size >= 4 && buf[0] == 1 {  // PacketID::Ok = 1
                // Parse status: buf[3] = key config, buf[4] = hall calib, buf[5] = enabled
                let status = meowpad::models::DeviceStatus {
                    key: buf[3] != 0,
                    hall: buf[4] != 0,
                    enabled: buf[5] != 0,
                    light: None,
                };
                
                serde_wasm_bindgen::to_value(&status)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            } else {
                log::error!("Unexpected response packet ID: {}", buf[0]);
                Err(JsValue::from_str("Unexpected response"))
            }
        }
        Err(e) => {
            log::error!("Status read error: {:?}", e);
            Err(JsValue::from_str(&format!("Read error: {:?}", e)))
        }
    }
}

/// Calibrate specific keys
#[wasm_bindgen]
pub async fn pure64_calibration_key(device_id: u32, key_indexes: Vec<u8>) -> Result<(), JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    meowboard.calibration_key(&key_indexes).await
        .map_err(|e| JsValue::from_str(&format!("Calibration failed: {:?}", e)))?;
    
    Ok(())
}

/// Clear all configuration
#[wasm_bindgen]
pub async fn pure64_clear_config(device_id: u32) -> Result<(), JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    meowboard.clear_hall_config().await
        .map_err(|e| JsValue::from_str(&format!("Failed to clear hall config: {:?}", e)))?;
    meowboard.clear_key_config().await
        .map_err(|e| JsValue::from_str(&format!("Failed to clear key config: {:?}", e)))?;
    
    Ok(())
}

/// Reset device
#[wasm_bindgen]
pub async fn pure64_reset_device(device_id: u32) -> Result<(), JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    meowboard.reset_device().await
        .map_err(|e| JsValue::from_str(&format!("Reset failed: {:?}", e)))?;
    
    Ok(())
}

/// Get debug values for a specific key index
#[wasm_bindgen]
pub async fn pure64_get_debug_value_part(device_id: u32, index: u8) -> Result<JsValue, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    let values = meowboard.get_debug_value_part(index).await
        .map_err(|e| JsValue::from_str(&format!("Failed to get debug values: {:?}", e)))?;
    
    serde_wasm_bindgen::to_value(&values.to_vec())
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get all debug values
#[wasm_bindgen]
pub async fn pure64_get_debug_value(device_id: u32) -> Result<JsValue, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    let values = meowboard.get_debug_value().await
        .map_err(|e| JsValue::from_str(&format!("Failed to get debug values: {:?}", e)))?;
    
    serde_wasm_bindgen::to_value(&values.to_vec())
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get hall effect configuration for all keys
#[wasm_bindgen]
pub async fn pure64_get_hall_config(device_id: u32) -> Result<JsValue, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    let config = meowboard.get_hall_config().await
        .map_err(|e| JsValue::from_str(&format!("Failed to get hall config: {:?}", e)))?;
    
    serde_wasm_bindgen::to_value(&config.to_vec())
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get key states
#[wasm_bindgen]
pub async fn pure64_get_keystates(device_id: u32) -> Result<JsValue, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    let states = meowboard.get_keystates().await
        .map_err(|e| JsValue::from_str(&format!("Failed to get keystates: {:?}", e)))?;
    
    serde_wasm_bindgen::to_value(&states.to_vec())
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get key values
#[wasm_bindgen]
pub async fn pure64_get_keyvalues(device_id: u32) -> Result<JsValue, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    let values = meowboard.get_keyvalues().await
        .map_err(|e| JsValue::from_str(&format!("Failed to get keyvalues: {:?}", e)))?;
    
    serde_wasm_bindgen::to_value(&values.to_vec())
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get key calibration status
#[wasm_bindgen]
pub async fn pure64_get_key_calibrate_status(device_id: u32) -> Result<JsValue, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    let status = meowboard.get_key_calibrate_status().await
        .map_err(|e| JsValue::from_str(&format!("Failed to get calibrate status: {:?}", e)))?;
    
    serde_wasm_bindgen::to_value(&status.to_vec())
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Erase firmware (caution!)
#[wasm_bindgen]
pub async fn pure64_erase_firmware(device_id: u32) -> Result<(), JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    meowboard.erase_firmware().await
        .map_err(|e| JsValue::from_str(&format!("Erase failed: {:?}", e)))?;
    
    Ok(())
}

/// Get default key configuration
#[wasm_bindgen]
pub fn pure64_get_default_key_config() -> Result<JsValue, JsValue> {
    let default_config: meowboard::config::Device = meowboard::cbor::Device::default()
        .try_into()
        .map_err(|e| JsValue::from_str(&format!("Failed to create default config: {:?}", e)))?;
    
    serde_wasm_bindgen::to_value(&default_config)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get current key configuration
#[wasm_bindgen]
pub async fn pure64_get_key_config(device_id: u32) -> Result<JsValue, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    let config = meowboard.load_key_config().await
        .map_err(|e| JsValue::from_str(&format!("Failed to load key config: {:?}", e)))?;
    
    serde_wasm_bindgen::to_value(&config)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Set key configuration
#[wasm_bindgen]
pub async fn pure64_set_key_config(device_id: u32, config: JsValue) -> Result<(), JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let config: meowboard::config::Device = serde_wasm_bindgen::from_value(config)
        .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;
    
    let mut meowboard = Meowboard::new(webhid_device);
    
    meowboard.set_key_config(config).await
        .map_err(|e| JsValue::from_str(&format!("Failed to set key config: {:?}", e)))?;
    
    Ok(())
}

/// Save current key configuration to device
#[wasm_bindgen]
pub async fn pure64_save_key_config(device_id: u32) -> Result<(), JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    meowboard.save_key_config().await
        .map_err(|e| JsValue::from_str(&format!("Failed to save key config: {:?}", e)))?;
    
    Ok(())
}

/// Get raw TOML configuration
#[wasm_bindgen]
pub async fn pure64_get_raw_config(device_id: u32) -> Result<String, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    let mut meowboard = Meowboard::new(webhid_device);
    
    let key_config = meowboard.load_key_config().await
        .map_err(|e| JsValue::from_str(&format!("Failed to load key config: {:?}", e)))?;
    
    #[derive(Serialize)]
    struct Config {
        key: meowboard::config::Device,
    }
    
    let config = Config { key: key_config };
    
    toml::to_string(&config)
        .map_err(|e| JsValue::from_str(&format!("TOML serialization error: {}", e)))
}

/// Check if raw TOML configuration is valid
#[wasm_bindgen]
pub fn pure64_check_raw_config(config: String) -> bool {
    #[derive(Deserialize)]
    struct Config {
        key: meowboard::config::Device,
    }
    
    toml::from_str::<Config>(&config).is_ok()
}

/// Save raw TOML configuration to device
#[wasm_bindgen]
pub async fn pure64_save_raw_config(device_id: u32, config: String) -> Result<(), JsValue> {
    let webhid_device = get_webhid_device(device_id)?;
    
    #[derive(Deserialize)]
    struct Config {
        key: meowboard::config::Device,
    }
    
    let cfg: Config = toml::from_str(&config)
        .map_err(|e| JsValue::from_str(&format!("Invalid TOML config: {}", e)))?;
    
    let mut meowboard = Meowboard::new(webhid_device);
    meowboard.set_key_config(cfg.key).await
        .map_err(|e| JsValue::from_str(&format!("Failed to set key config: {:?}", e)))?;
    meowboard.save_key_config().await
        .map_err(|e| JsValue::from_str(&format!("Failed to save key config: {:?}", e)))?;
    
    Ok(())
}

/// Ping device to test connection
#[wasm_bindgen]
pub async fn pure64_ping(device_id: u32) -> Result<bool, JsValue> {
    let webhid_device = get_webhid_device(device_id)?;

    let mut meowboard = Meowboard::new(webhid_device);
    let res = meowboard.ping().await
        .map_err(|e| JsValue::from_str(&format!("Failed to ping device: {:?}", e)))?;

    Ok(res)
}

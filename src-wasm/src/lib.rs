pub mod webhid_device;
pub mod utils;
pub mod pure64;

use wasm_bindgen::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use js_sys::Reflect;

use webhid_device::WebHidDevice;

// State management
static mut DEVICE_CACHE: Option<Arc<Mutex<std::collections::HashMap<u32, Arc<WebHidDevice>>>>> = None;
static mut DEVICE_ID_COUNTER: u32 = 0;

pub fn get_device_cache() -> Arc<Mutex<std::collections::HashMap<u32, Arc<WebHidDevice>>>> {
    unsafe {
        if DEVICE_CACHE.is_none() {
            DEVICE_CACHE = Some(Arc::new(Mutex::new(std::collections::HashMap::new())));
        }
        Arc::clone(DEVICE_CACHE.as_ref().unwrap())
    }
}

fn next_device_id() -> u32 {
    unsafe {
        DEVICE_ID_COUNTER += 1;
        DEVICE_ID_COUNTER
    }
}

/// Device info for JavaScript serialization
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub serial_number: String,
    pub product_name: String,
    pub manufacturer_name: String,
}

/// Initialize logging for debugging
#[wasm_bindgen]
pub fn init_logger() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
}

fn get_navigator() -> Result<JsValue, JsValue> {
    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("no window available"))?;
    let nav = Reflect::get(&window, &JsValue::from_str("navigator"))
        .map_err(|_| JsValue::from_str("no navigator available"))?;
    Ok(nav)
}

/// Test if WebHID is available
#[wasm_bindgen]
pub fn is_webhid_available() -> bool {
    if let Ok(navigator) = get_navigator() {
        Reflect::get(&navigator, &JsValue::from_str("hid")).is_ok()
    } else {
        false
    }
}

/// Get version string
#[wasm_bindgen]
pub fn version() -> String {
    "1.2.1".to_string()
}

/// Get available HID devices
#[wasm_bindgen]
pub async fn enumerate_devices() -> Result<JsValue, JsValue> {
    let navigator = get_navigator()?;
    let hid = Reflect::get(&navigator, &JsValue::from_str("hid"))?;
    
    let get_devices_fn = Reflect::get(&hid, &JsValue::from_str("getDevices"))?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| JsValue::from_str("getDevices is not a function"))?;
    
    let promise = Reflect::apply(&get_devices_fn, &hid, &js_sys::Array::new())?;
    let promise_obj = js_sys::Promise::from(promise);
    let future = wasm_bindgen_futures::JsFuture::from(promise_obj);
    
    future.await.map(|devices_value| devices_value)
}

/// Connect to an already cached HIDDevice (from enumeration)
#[wasm_bindgen]
pub async fn connect_cached_device(hid_device_js: JsValue) -> Result<u32, JsValue> {
    log::info!("Connecting to cached device");
    
    // Open the device if not already opened
    let opened = Reflect::get(&hid_device_js, &JsValue::from_str("opened"))?;
    if !opened.as_bool().unwrap_or(false) {
        log::info!("Opening device...");
        let open_fn = Reflect::get(&hid_device_js, &JsValue::from_str("open"))?
            .dyn_into::<js_sys::Function>()
            .map_err(|_| JsValue::from_str("open is not a function"))?;
        
        let open_promise = Reflect::apply(&open_fn, &hid_device_js, &js_sys::Array::new())?;
        let open_future = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(open_promise));
        open_future.await?;
        
        log::info!("Device opened successfully");
    } else {
        log::info!("Device already opened");
    }
    
    let webhid_device = WebHidDevice::new(hid_device_js);
    let device_id = next_device_id();
    
    if let Ok(mut cache) = get_device_cache().lock() {
        cache.insert(device_id, Arc::new(webhid_device));
    }
    
    log::info!("Device cached with ID: {}", device_id);
    Ok(device_id)
}

/// Connect to device by requesting user selection (fallback when device not in cache)
#[wasm_bindgen]
pub async fn request_device(vendor_id: u16, product_id: u16) -> Result<u32, JsValue> {
    let navigator = get_navigator()?;
    let hid = Reflect::get(&navigator, &JsValue::from_str("hid"))?;
    
    // Create filter object
    let filter = js_sys::Object::new();
    Reflect::set(&filter, &JsValue::from_str("vendorId"), &JsValue::from(vendor_id))?;
    Reflect::set(&filter, &JsValue::from_str("productId"), &JsValue::from(product_id))?;
    
    let filters = js_sys::Array::new();
    filters.push(&filter);
    
    let options = js_sys::Object::new();
    Reflect::set(&options, &JsValue::from_str("filters"), &filters)?;
    
    let request_device_fn = Reflect::get(&hid, &JsValue::from_str("requestDevice"))?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| JsValue::from_str("requestDevice is not a function"))?;
    
    let promise = Reflect::apply(&request_device_fn, &hid, &js_sys::Array::of1(&options))?;
    let promise_obj = js_sys::Promise::from(promise);
    let future = wasm_bindgen_futures::JsFuture::from(promise_obj);
    
    match future.await {
        Ok(device_obj) => {
            let device_array = js_sys::Array::from(&device_obj);
            if device_array.length() > 0 {
                let device = device_array.get(0);
                
                // Open the device before using it
                let open_fn = Reflect::get(&device, &JsValue::from_str("open"))?
                    .dyn_into::<js_sys::Function>()
                    .map_err(|_| JsValue::from_str("open is not a function"))?;
                
                let open_promise = Reflect::apply(&open_fn, &device, &js_sys::Array::new())?;
                let open_future = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(open_promise));
                open_future.await?;
                
                log::info!("Device opened successfully");
                
                let webhid_device = WebHidDevice::new(device);
                let device_id = next_device_id();
                
                if let Ok(mut cache) = get_device_cache().lock() {
                    cache.insert(device_id, Arc::new(webhid_device));
                }
                
                Ok(device_id)
            } else {
                Err(JsValue::from_str("No device selected"))
            }
        }
        Err(e) => Err(e),
    }
}

/// Test device connection
#[wasm_bindgen]
pub fn test_device(device_id: u32) -> Result<bool, JsValue> {
    match get_device_cache().lock() {
        Ok(cache) => {
            if cache.contains_key(&device_id) {
                Ok(true)
            } else {
                Err(JsValue::from_str("Device not found"))
            }
        }
        Err(_) => Err(JsValue::from_str("Cache lock failed")),
    }
}

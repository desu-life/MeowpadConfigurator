use meowpad::Device;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Reflect, Uint8Array};
use web_sys;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use async_trait::async_trait;
use log;


/// WebHID implementation of the Device trait
/// Uses js_sys to interact with navigator.hid API from JavaScript
#[derive(Clone)]
pub struct WebHidDevice {
    device: Arc<JsValue>,  // HIDDevice object from JS
    read_buffer: Arc<Mutex<VecDeque<u8>>>,
}

impl WebHidDevice {
    pub fn new(device: JsValue) -> Self {
        let device = Arc::new(device);
        let read_buffer = Arc::new(Mutex::new(VecDeque::new()));

        // Set up event listener for input reports
        let buffer_clone = Arc::clone(&read_buffer);
        
        let on_input_report = Closure::wrap(
            Box::new(move |event: JsValue| {
                // WebHID inputreport event structure:
                // { reportId: number, data: DataView }
                
                // Get reportId
                let report_id = if let Ok(id) = Reflect::get(&event, &JsValue::from_str("reportId")) {
                    id.as_f64().unwrap_or(0.0) as u8
                } else {
                    log::error!("Failed to get reportId");
                    return;
                };
                
                // Get data (DataView)
                if let Ok(data_view) = Reflect::get(&event, &JsValue::from_str("data")) {
                    // DataView needs to be converted to Uint8Array
                    if let Ok(buffer) = Reflect::get(&data_view, &JsValue::from_str("buffer")) {
                        if let Ok(byte_offset) = Reflect::get(&data_view, &JsValue::from_str("byteOffset")) {
                            if let Ok(byte_length) = Reflect::get(&data_view, &JsValue::from_str("byteLength")) {
                                // Create Uint8Array from buffer
                                let uint8_array = Uint8Array::new_with_byte_offset_and_length(
                                    &buffer,
                                    byte_offset.as_f64().unwrap_or(0.0) as u32,
                                    byte_length.as_f64().unwrap_or(0.0) as u32,
                                );
                                
                                let bytes: Vec<u8> = uint8_array.to_vec();
                                log::debug!("Received report ID {} with {} bytes", report_id, bytes.len());
                                
                                if let Ok(mut buf) = buffer_clone.lock() {
                                    // Only store data bytes, report ID is handled by WebHID
                                    for byte in bytes {
                                        buf.push_back(byte);
                                    }
                                }
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(JsValue)>,
        );

        let _ = Reflect::apply(
            &Reflect::get(&device, &JsValue::from_str("addEventListener"))
                .unwrap()
                .dyn_into::<js_sys::Function>()
                .unwrap(),
            &device,
            &js_sys::Array::of3(
                &JsValue::from_str("inputreport"),
                &on_input_report.as_ref().clone(),
                &JsValue::from_bool(false),
            ),
        );
        on_input_report.forget();

        WebHidDevice { device, read_buffer }
    }

    /// Try to read buffered data without blocking
    fn try_read_buffered(&self, buf: &mut [u8]) -> Option<usize> {
        let mut buffer = match self.read_buffer.lock() {
            Ok(b) => b,
            Err(_) => return None,
        };

        if buffer.is_empty() {
            return None;
        }

        let mut read_count = 0;
        for byte in buf.iter_mut() {
            if let Some(data) = buffer.pop_front() {
                *byte = data;
                read_count += 1;
            } else {
                break;
            }
        }

        if read_count > 0 {
            Some(read_count)
        } else {
            None
        }
    }

    /// Async write - waits for the write to complete
    pub async fn write_async(&self, data: &[u8]) -> meowpad::Result<usize> {
        // Check if device is opened
        let opened = Reflect::get(&self.device, &JsValue::from_str("opened"))
            .map(|v| v.as_bool().unwrap_or(false))
            .unwrap_or(false);
        
        if !opened {
            log::error!("Device not opened, cannot write");
            return Err(meowpad::error::Error::Disconnect);
        }
        
        // WebHID expects: sendReport(reportId, data)
        // Input data format: [reportId (1 byte), ...data (64 bytes)] = 65 bytes total
        // We need to split them: reportId from first byte, data from remaining 64 bytes
        if data.len() != 65 {
            log::error!("Invalid data length: expected 65 bytes, got {}", data.len());
            return Err(meowpad::error::Error::Disconnect);
        }
        
        let report_id = data[0];
        let data_bytes = &data[1..]; // Skip first byte (report ID)
        
        // Convert data (without report ID) to Uint8Array
        let array = Uint8Array::from(data_bytes);
        
        // log::debug!("Writing report ID {} with {} bytes of data", report_id, data_bytes.len());

        // Use sendReport for interrupt OUT transfers (standard for custom keyboards)
        let send_fn_name = "sendReport";
        let send_report_fn = match Reflect::get(&self.device, &JsValue::from_str(send_fn_name)) {
            Ok(func) => func,
            Err(e) => {
                log::error!("Failed to get {} function: {:?}", send_fn_name, e);
                return Err(meowpad::error::Error::Disconnect);
            }
        };
        
        let func = match send_report_fn.dyn_into::<js_sys::Function>() {
            Ok(f) => f,
            Err(e) => {
                log::error!("{} is not a function: {:?}", send_fn_name, e);
                return Err(meowpad::error::Error::Disconnect);
            }
        };
        
        // For devices without report IDs (report_id == 0), WebHID expects:
        // sendReport(0, full_data_without_leading_zero)
        // For devices with report IDs, WebHID expects:
        // sendReport(reportId, data_without_id)
        let promise = match Reflect::apply(
            &func,
            &self.device,
            &js_sys::Array::of2(&JsValue::from(report_id), &array),
        ) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Write error: {:?}", e);
                return Err(meowpad::error::Error::Disconnect);
            }
        };
        
        // Wait for the promise to resolve
        match wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(promise)).await {
            Ok(_) => {
                // log::debug!("Successfully sent {} bytes", data.len());
                Ok(data.len())
            }
            Err(e) => {
                log::error!("Write promise rejected: {:?}", e);
                Err(meowpad::error::Error::Disconnect)
            }
        }
    }
    
    pub fn get_device(&self) -> Arc<JsValue> {
        Arc::clone(&self.device)
    }
    
    /// Async read with timeout - waits for data to arrive
    pub async fn read_timeout_async(&self, buf: &mut [u8], timeout_ms: i32) -> meowpad::Result<usize> {
        // Clear buffer before reading to avoid stale data
        let _ = self.clear_buffer().await;
        
        let max_attempts = (timeout_ms / 10).max(10) as usize;
        
        for _ in 0..max_attempts {
            // Check if data is available
            if let Some(count) = self.try_read_buffered(buf) {
                return Ok(count);
            }
            
            // Wait 10ms before next attempt
            let promise = js_sys::Promise::new(&mut |resolve, _reject| {
                let _ = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        &resolve,
                        10,
                    );
            });
            
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
        }
        
        Err(meowpad::error::Error::Disconnect)
    }
}

#[async_trait(?Send)]
impl Device for WebHidDevice {
    async fn get_serial(&self) -> meowpad::Result<String> {
        match Reflect::get(&self.device, &JsValue::from_str("serialNumber")) {
            Ok(val) => {
                if val.is_string() {
                    Ok(val.as_string().unwrap_or_default())
                } else {
                    Err(meowpad::error::Error::Disconnect)
                }
            }
            Err(_) => Err(meowpad::error::Error::Disconnect),
        }
    }

    async fn write(&self, data: &[u8]) -> meowpad::Result<usize> {
        // Use the async version directly
        self.write_async(data).await
    }

    async fn read(&self, buf: &mut [u8]) -> meowpad::Result<usize> {
        // Blocking read with default 5 second timeout
        self.read_timeout_async(buf, 5000).await
    }

    async fn read_timeout(&self, buf: &mut [u8], timeout: i32) -> meowpad::Result<usize> {
        // Use the async version directly
        self.read_timeout_async(buf, timeout).await
    }

    async fn clear_buffer(&self) -> meowpad::Result<()> {
        match self.read_buffer.lock() {
            Ok(mut buffer) => {
                buffer.clear();
                Ok(())
            }
            Err(_) => Err(meowpad::error::Error::Disconnect),
        }
    }
}

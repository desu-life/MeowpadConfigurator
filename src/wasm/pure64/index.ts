/**
 * Unified Pure64 API with automatic Tauri/WebHID selection
 * This layer automatically chooses between Tauri invoke and WASM based on environment
 */

import { getRuntimeEnvironment } from '../environment';
import * as tauriApi from '../../apis/pure64/api';
import * as wasmApi from '../pure64-wasm';
import { initWasm, isWasmInitialized, getWasmDeviceId, setWasmDeviceId } from '../loader';
import type { IDeviceInfo, IDeviceStatus, IKeyHallConfig, IKeyRTStatus, KeyState } from '../../apis';
import type { IKeyboard } from '../../apis/pure64/config';

// Store current device ID for WASM mode
let currentWasmDeviceId: number | null = null;

/**
 * Initialize the API layer
 * Must be called before using any functions
 */
export async function initialize(): Promise<void> {
  const env = getRuntimeEnvironment();
  
  if (env === 'webhid') {
    await initWasm();
  } else if (env === 'tauri') {
    // Tauri doesn't need initialization
    return;
  } else {
    throw new Error('Unsupported environment. Requires Tauri or WebHID.');
  }
}

/**
 * Connect to device
 * In Tauri mode: uses existing Tauri connection
 * In WebHID mode: uses cached device from enumeration, or requests user selection as fallback
 */
export async function connect(): Promise<boolean> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.connect();
  } else if (env === 'webhid') {
    try {
      const { getCachedDevice } = await import('../device-cache');
      
      const VID = 0x5D3E;
      const PID = 0xFB01;
      
      // Try to use cached device first
      const cachedDevice = getCachedDevice(VID, PID);
      let deviceId: number;
      
      if (cachedDevice) {
        console.log('[Pure64] Using cached device');
        deviceId = await wasmApi.pure64ConnectCachedDevice(cachedDevice);
      } else {
        console.log('[Pure64] No cached device, requesting user selection');
        deviceId = await wasmApi.pure64RequestDevice();
      }
      
      currentWasmDeviceId = deviceId;
      setWasmDeviceId('pure64', deviceId);
      
      // Test connection with ping
      const pingResult = await wasmApi.pure64Ping(deviceId);
      return pingResult;
    } catch (error) {
      console.error('[Pure64] Connection failed:', error);
      return false;
    }
  }
  
  return false;
}

/**
 * Helper to get device ID for WASM calls
 */
function getDeviceId(): number {
  if (currentWasmDeviceId === null) {
    throw new Error('Device not connected. Call connect() first.');
  }
  return currentWasmDeviceId;
}

/**
 * Get firmware version list
 */
export async function get_firmware_version(): Promise<string[]> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_firmware_version();
  } else if (env === 'webhid') {
    // Check if WASM is initialized
    if (!isWasmInitialized()) {
      console.warn('[Pure64] WASM not initialized, returning empty firmware version list');
      return [];
    }
    return wasmApi.pure64GetFirmwareVersion();
  }
  
  return [];
}

/**
 * Get device information
 */
export async function get_device_info(): Promise<IDeviceInfo> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_device_info();
  } else {
    return wasmApi.pure64GetDeviceInfo(getDeviceId());
  }
}

/**
 * Get device status
 */
export async function get_device_status(): Promise<IDeviceStatus> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_device_status();
  } else {
    return wasmApi.pure64GetDeviceStatus(getDeviceId());
  }
}

/**
 * Calibrate keys
 */
export async function calibration_key(keyIndexs: number[]): Promise<void> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.calibration_key(keyIndexs);
  } else {
    return wasmApi.pure64CalibrationKey(getDeviceId(), keyIndexs);
  }
}

/**
 * Clear all configuration
 */
export async function clear_config(): Promise<void> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.clear_config();
  } else {
    return wasmApi.pure64ClearConfig(getDeviceId());
  }
}

/**
 * Reset device
 */
export async function reset_device(): Promise<boolean> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.reset_device();
  } else {
    await wasmApi.pure64ResetDevice(getDeviceId());
    return true;
  }
}

/**
 * Get debug values for specific key
 */
export async function get_debug_value_part(index: number): Promise<IKeyRTStatus[]> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_debug_value_part(index);
  } else {
    return wasmApi.pure64GetDebugValuePart(getDeviceId(), index);
  }
}

/**
 * Get all debug values
 */
export async function get_debug_value(): Promise<IKeyRTStatus[]> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_debug_value();
  } else {
    return wasmApi.pure64GetDebugValue(getDeviceId());
  }
}

/**
 * Get hall effect configuration
 */
export async function get_hall_config(): Promise<IKeyHallConfig[]> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_hall_config();
  } else {
    return wasmApi.pure64GetHallConfig(getDeviceId());
  }
}

/**
 * Get key states
 */
export async function get_keystates(): Promise<KeyState[]> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_keystates();
  } else {
    return wasmApi.pure64GetKeystates(getDeviceId());
  }
}

/**
 * Get key values
 */
export async function get_keyvalues(): Promise<number[]> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_keyvalues();
  } else {
    return wasmApi.pure64GetKeyvalues(getDeviceId());
  }
}

/**
 * Get key calibration status
 */
export async function get_key_calibrate_status(): Promise<boolean[]> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_key_calibrate_status();
  } else {
    return wasmApi.pure64GetKeyCalibrateStatus(getDeviceId());
  }
}

/**
 * Erase firmware
 */
export async function erase_firmware(): Promise<void> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.erase_firmware();
  } else {
    return wasmApi.pure64EraseFirmware(getDeviceId());
  }
}

/**
 * Get default key configuration
 */
export async function get_default_key_config(): Promise<IKeyboard> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_default_key_config();
  } else {
    return wasmApi.pure64GetDefaultKeyConfig();
  }
}

/**
 * Get current key configuration
 */
export async function get_key_config(): Promise<IKeyboard> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_key_config();
  } else {
    return wasmApi.pure64GetKeyConfig(getDeviceId());
  }
}

/**
 * Set key configuration
 */
export async function set_key_config(config: IKeyboard): Promise<void> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.set_key_config(config);
  } else {
    return wasmApi.pure64SetKeyConfig(getDeviceId(), config);
  }
}

/**
 * Save key configuration to device
 */
export async function save_key_config(): Promise<void> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.save_key_config();
  } else {
    return wasmApi.pure64SaveKeyConfig(getDeviceId());
  }
}

/**
 * Get raw TOML configuration
 */
export async function get_raw_config(): Promise<string> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.get_raw_config();
  } else {
    return wasmApi.pure64GetRawConfig(getDeviceId());
  }
}

/**
 * Check if raw configuration is valid
 */
export async function check_raw_config(config: string): Promise<boolean> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.check_raw_config(config);
  } else {
    return wasmApi.pure64CheckRawConfig(config);
  }
}

/**
 * Save raw TOML configuration
 */
export async function save_raw_config(config: string): Promise<void> {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    return tauriApi.save_raw_config(config);
  } else {
    return wasmApi.pure64SaveRawConfig(getDeviceId(), config);
  }
}

/**
 * Enumerate available devices (WebHID only)
 */
export async function enumerate_devices(): Promise<any[]> {
  const env = getRuntimeEnvironment();
  
  if (env === 'webhid') {
    return wasmApi.pure64EnumerateDevices();
  }
  
  return [];
}

/**
 * Get current runtime environment
 */
export function getEnvironment() {
  return getRuntimeEnvironment();
}

/**
 * Check if currently connected
 */
export function isConnected(): boolean {
  const env = getRuntimeEnvironment();
  
  if (env === 'tauri') {
    // In Tauri, we'd need to track connection state
    // For now, assume connected if in Tauri mode
    return true;
  } else {
    return currentWasmDeviceId !== null;
  }
}

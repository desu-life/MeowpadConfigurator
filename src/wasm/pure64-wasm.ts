/**
 * Pure64 keyboard WASM API wrapper
 * Provides TypeScript interface to WASM-compiled Rust functions
 */

import { getWasmModule, getWasmDeviceId } from './loader';
import type { IDeviceInfo, IDeviceStatus, IKeyHallConfig, IKeyRTStatus, KeyState } from '../apis';
import type { IKeyboard } from '../apis/pure64/config';

/**
 * Get firmware version list
 */
export async function pure64GetFirmwareVersion(): Promise<string[]> {
  const wasm = getWasmModule();
  return wasm.pure64_get_firmware_version();
}

/**
 * Get device information
 */
export async function pure64GetDeviceInfo(deviceId: number): Promise<IDeviceInfo> {
  const wasm = getWasmModule();
  const result = wasm.pure64_get_device_info(deviceId);
  return result;
}

/**
 * Get device status
 */
export async function pure64GetDeviceStatus(deviceId: number): Promise<IDeviceStatus> {
  const wasm = getWasmModule();
  const result = wasm.pure64_get_device_status(deviceId);
  return result;
}

/**
 * Calibrate specific keys
 */
export async function pure64CalibrationKey(deviceId: number, keyIndexes: number[]): Promise<void> {
  const wasm = getWasmModule();
  return wasm.pure64_calibration_key(deviceId, new Uint8Array(keyIndexes));
}

/**
 * Clear all configuration
 */
export async function pure64ClearConfig(deviceId: number): Promise<void> {
  const wasm = getWasmModule();
  return wasm.pure64_clear_config(deviceId);
}

/**
 * Reset device
 */
export async function pure64ResetDevice(deviceId: number): Promise<void> {
  const wasm = getWasmModule();
  return wasm.pure64_reset_device(deviceId);
}

/**
 * Get debug values for specific key
 */
export async function pure64GetDebugValuePart(deviceId: number, index: number): Promise<IKeyRTStatus[]> {
  const wasm = getWasmModule();
  const result = await wasm.pure64_get_debug_value_part(deviceId, index);
  return result;
}

/**
 * Get all debug values
 */
export async function pure64GetDebugValue(deviceId: number): Promise<IKeyRTStatus[]> {
  const wasm = getWasmModule();
  const result = await wasm.pure64_get_debug_value(deviceId);
  return result;
}

/**
 * Get hall effect configuration
 */
export async function pure64GetHallConfig(deviceId: number): Promise<IKeyHallConfig[]> {
  const wasm = getWasmModule();
  const result = await wasm.pure64_get_hall_config(deviceId);
  return result;
}

/**
 * Get key states
 */
export async function pure64GetKeystates(deviceId: number): Promise<KeyState[]> {
  const wasm = getWasmModule();
  const result = await wasm.pure64_get_keystates(deviceId);
  return result;
}

/**
 * Get key values
 */
export async function pure64GetKeyvalues(deviceId: number): Promise<number[]> {
  const wasm = getWasmModule();
  const result = await wasm.pure64_get_keyvalues(deviceId);
  return result;
}

/**
 * Get key calibration status
 */
export async function pure64GetKeyCalibrateStatus(deviceId: number): Promise<boolean[]> {
  const wasm = getWasmModule();
  const result = await wasm.pure64_get_key_calibrate_status(deviceId);
  return result;
}

/**
 * Erase firmware (dangerous!)
 */
export async function pure64EraseFirmware(deviceId: number): Promise<void> {
  const wasm = getWasmModule();
  return wasm.pure64_erase_firmware(deviceId);
}

/**
 * Get default key configuration
 */
export async function pure64GetDefaultKeyConfig(): Promise<IKeyboard> {
  const wasm = getWasmModule();
  const result = wasm.pure64_get_default_key_config();
  return result;
}

/**
 * Get current key configuration
 */
export async function pure64GetKeyConfig(deviceId: number): Promise<IKeyboard> {
  const wasm = getWasmModule();
  const result = await wasm.pure64_get_key_config(deviceId);
  return result;
}

/**
 * Set key configuration
 */
export async function pure64SetKeyConfig(deviceId: number, config: IKeyboard): Promise<void> {
  const wasm = getWasmModule();
  return wasm.pure64_set_key_config(deviceId, config);
}

/**
 * Save key configuration to device
 */
export async function pure64SaveKeyConfig(deviceId: number): Promise<void> {
  const wasm = getWasmModule();
  return wasm.pure64_save_key_config(deviceId);
}

/**
 * Get raw TOML configuration
 */
export async function pure64GetRawConfig(deviceId: number): Promise<string> {
  const wasm = getWasmModule();
  return wasm.pure64_get_raw_config(deviceId);
}

/**
 * Check if raw TOML configuration is valid
 */
export async function pure64CheckRawConfig(config: string): Promise<boolean> {
  const wasm = getWasmModule();
  return wasm.pure64_check_raw_config(config);
}

/**
 * Save raw TOML configuration
 */
export async function pure64SaveRawConfig(deviceId: number, config: string): Promise<void> {
  const wasm = getWasmModule();
  return wasm.pure64_save_raw_config(deviceId, config);
}

/**
 * Ping device to test connection
 */
export async function pure64Ping(deviceId: number): Promise<boolean> {
  const wasm = getWasmModule();
  return await wasm.pure64_ping(deviceId);
}

/**
 * Connect to a cached Pure64 device (from enumeration)
 */
export async function pure64ConnectCachedDevice(device: HIDDevice): Promise<number> {
  const wasm = getWasmModule();
  const deviceId = await wasm.connect_cached_device(device);
  return deviceId;
}

/**
 * Request user to select a Pure64 device via WebHID (fallback)
 */
export async function pure64RequestDevice(): Promise<number> {
  const wasm = getWasmModule();
  const VID = 0x5D3E;
  const PID = 0xFB01;
  
  const deviceId = await wasm.request_device(VID, PID);
  return deviceId;
}

/**
 * Enumerate available Pure64 devices
 */
export async function pure64EnumerateDevices(): Promise<any[]> {
  const wasm = getWasmModule();
  const devices = await wasm.enumerate_devices();
  
  // Filter for Pure64 devices
  const VID = 0x5D3E;
  const PID = 0xFB01;
  
  return devices.filter((d: any) => d.vendor_id === VID && d.product_id === PID);
}

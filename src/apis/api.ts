import { invoke } from "@tauri-apps/api/core";
import { IDevicePreset, IHidDeviceInfo, IVersion } from ".";
import { IKeyboard as PureConfig } from "./pure64/config";
import { isTauri } from "@/wasm/environment";

export async function connect_iap() {
  if (!isTauri()) throw new Error('Tauri only');
  return (await invoke("connect_iap")) as void;
}
export async function iap_start(data: number[]) {
  if (!isTauri()) throw new Error('Tauri only');
  return (await invoke("iap_start", { data })) as number;
  // Array.from(new Uint8Array(data))
}
export async function iap_flush() {
  if (!isTauri()) throw new Error('Tauri only');
  return (await invoke("iap_flush")) as void;
}
export async function check_update(version: IVersion[]) {
  if (!isTauri()) return false;
  return (await invoke("check_update", { version })) as boolean;
}
export async function open_update_url(version: IVersion, str: string) {
  if (!isTauri()) return false;
  return (await invoke("open_update_url", { version, str })) as boolean;
}
export async function get_latest_version() {
  if (!isTauri()) return [];
  return (await invoke("get_latest_version")) as IVersion[];
}
export async function get_theme() {
  if (!isTauri()) return 'auto';
  return (await invoke("get_theme")) as string;
}
export async function device_list() {
  if (!isTauri()) {
    // In browser mode, use WebHID
    try {
      if (!navigator.hid) {
        console.warn('[WebHID] Not available');
        return [];
      }
      
      const devices = await navigator.hid.getDevices();
      
      console.log('[WebHID] Total device interfaces:', devices.length);
      devices.forEach((dev, idx) => {
        console.log(`[WebHID] Interface ${idx}:`, {
          vendorId: dev.vendorId.toString(16),
          productId: dev.productId.toString(16),
          productName: dev.productName,
          collections: dev.collections.map(c => ({
            usagePage: '0x' + c.usagePage.toString(16),
            usage: '0x' + c.usage.toString(16),
          }))
        });
      });
      
      // Filter duplicates: One physical device has multiple HID interfaces
      // We need to select the vendor-specific/configuration interface, not the keyboard input interface
      // Configuration interface typically has usagePage 0xFF00 or higher (vendor-specific)
      const deviceMap = new Map<string, HIDDevice>();
      for (const device of devices) {
        const key = `${device.vendorId}-${device.productId}`;
        
        // Look for vendor-specific interface (usagePage >= 0xFF00)
        // or RAW HID interface (usage 0x01-0x02 with usagePage 0xFF00)
        const hasVendorInterface = device.collections.some(col => 
          col.usagePage >= 0xFF00 || 
          (col.usagePage === 0xFF00 && col.usage >= 0x01)
        );
        
        console.log(`[WebHID] Device ${key}, has vendor interface: ${hasVendorInterface}`);
        
        // Prefer vendor-specific interface, or take first if none found yet
        if (!deviceMap.has(key)) {
          deviceMap.set(key, device);
        } else if (hasVendorInterface && !deviceMap.get(key)!.collections.some(col => col.usagePage >= 0xFF00)) {
          // Replace with vendor interface if current one isn't
          console.log(`[WebHID] Replacing with vendor interface for ${key}`);
          deviceMap.set(key, device);
        }
      }
      
      // Map device info by product ID
      const getDeviceNameByPid = (pid: number): string => {
        switch (pid) {
          case 0xFB01: return 'Pure64';
          case 0xFB02: return 'MeowpadV3';
          case 0xFB03: return 'Meowpad';
          case 0xFB04: return 'Meowpad SE v2';
          case 0xFB05: return 'Meowpad SE v2.1';
          default: return 'Unknown Device';
        }
      };
      
      // Try to get detailed device info (firmware version)
      // This requires temporarily connecting to each device
      const { getWasmModule, initWasm, isWasmInitialized } = await import('@/wasm/loader');
      const { cacheDevice } = await import('@/wasm/device-cache');
      
      // Ensure WASM is initialized
      if (!isWasmInitialized()) {
        console.log('[WebHID] Initializing WASM for device probing...');
        await initWasm();
      }
      
      const result: IHidDeviceInfo[] = [];
      
      for (const device of deviceMap.values()) {
        const deviceName = getDeviceNameByPid(device.productId);
        let firmwareVersion = '';
        
        // Try to fetch firmware version for Pure64 devices
        if (device.productId === 0xFB01) {
          try {
            // Ensure device is opened
            if (!device.opened) {
              await device.open();
            }
            
            // Use WASM probe function to get device info (now async)
            const wasm = getWasmModule();
            const probeResult = await wasm.pure64_probe_device(device);
            
            if (probeResult) {
              firmwareVersion = probeResult.version || '';
            }
            
            // Close device after probing
            if (device.opened) {
              await device.close();
            }
          } catch (err: any) {
            console.warn(`[WebHID] Could not probe device ${deviceName}:`, err?.message || err);
            // Continue without firmware version
            if (device.opened) {
              try { await device.close(); } catch {}
            }
          }
        }
        
        // Cache the device for later connection
        cacheDevice(device.vendorId, device.productId, device);
        
        result.push({
          path: [device.vendorId, device.productId],
          vendor_id: device.vendorId,
          product_id: device.productId,
          serial_number: device.serialNumber || undefined,
          interface_number: 0,
          device_name: deviceName as any,
          firmware_version: firmwareVersion || '', // Will be fetched on connection
        });
      }
      
      console.log(`[WebHID] Found ${result.length} unique devices (${devices.length} total interfaces)`);
      return result;
    } catch (err) {
      console.warn('[WebHID] Failed to get devices:', err);
      return [];
    }
  }
  return (await invoke("device_list")) as IHidDeviceInfo[];
}
export async function refresh_devices() {
  if (!isTauri()) return false; // WebHID doesn't need refresh
  return (await invoke("refresh_devices")) as boolean;
}
export async function connect_device(deviceInfo: IHidDeviceInfo) {
  if (!isTauri()) {
    // WebHID: Call device-specific connect based on device name
    const deviceName = deviceInfo.device_name;
    console.log(`[WebHID] Connecting to device: ${deviceName}`);
    
    try {
      switch (deviceName) {
        case 'Pure64': {
          const pure64 = await import('@/wasm/pure64');
          return await pure64.connect();
        }
        case 'MeowpadV3': {
          const v3 = await import('@/apis/meowpadv3/api');
          return await v3.connect();
        }
        case 'Meowpad': {
          const v2 = await import('@/apis/meowpadv2/api');
          return await v2.connect();
        }
        case 'Meowpad SE v2': {
          const v2se = await import('@/apis/meowpadv2se/api');
          return await v2se.connect();
        }
        case 'Meowpad SE v2.1': {
          const v21se = await import('@/apis/meowpadv21se/api');
          return await v21se.connect();
        }
        default:
          console.warn(`[WebHID] Unknown device: ${deviceName}`);
          return false;
      }
    } catch (err) {
      console.error(`[WebHID] Failed to connect to ${deviceName}:`, err);
      return false;
    }
  }
  return (await invoke("connect_device", { deviceInfo })) as boolean;
}
export async function load_preset_kb(config: PureConfig, preset: IDevicePreset) {
  if (!isTauri()) throw new Error('Tauri only');
  return (await invoke("load_preset_kb", { config, preset })) as PureConfig;
}
export async function gen_preset_kb(name: string, config: PureConfig) {
  if (!isTauri()) throw new Error('Tauri only');
  return (await invoke("gen_preset_kb", { name, config })) as IDevicePreset;
}
export async function load_preset_from_file() {
  if (!isTauri()) return null;
  return (await invoke("load_preset_from_file")) as IDevicePreset | null;
}
export async function save_preset_to_file(preset: IDevicePreset) {
  if (!isTauri()) throw new Error('Tauri only');
  return (await invoke("save_preset_to_file", { preset }));
}
export async function update_firmware_call() {
  if (!isTauri()) throw new Error('Tauri only');
  return (await invoke("update_firmware_call"));
}

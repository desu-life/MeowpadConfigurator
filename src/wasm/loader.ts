/**
 * WASM module loader and initializer
 */

import { isWebHIDAvailable } from './environment';

// WASM module instance
let wasmModule: any = null;
let wasmInitialized = false;
let wasmInitPromise: Promise<void> | null = null;

/**
 * Initialize WASM module
 * This should be called once at app startup for web environment
 */
export async function initWasm(): Promise<void> {
  // Return existing promise if initialization is in progress
  if (wasmInitPromise) {
    return wasmInitPromise;
  }

  // Return immediately if already initialized
  if (wasmInitialized) {
    return Promise.resolve();
  }

  // Check if WebHID is available
  if (!isWebHIDAvailable()) {
    throw new Error('WebHID is not available in this browser');
  }

  // Start initialization
  wasmInitPromise = (async () => {
    try {
      // Dynamic import of WASM module
      const loadedModule = await import('../wasm-bindings/meowpad_wasm.js');
      
      // Initialize WASM
      await loadedModule.default();
      
      // Initialize logger
      if (loadedModule.init_logger) {
        loadedModule.init_logger();
      }
      
      console.log('[WASM] Module loaded successfully');
      
      // Store module reference
      wasmModule = loadedModule;
      wasmInitialized = true;
      
    } catch (error) {
      wasmInitPromise = null;
      console.error('[WASM] Failed to load module:', error);
      throw new Error(`Failed to initialize WASM module: ${error}`);
    }
  })();

  return wasmInitPromise;
}

/**
 * Get the initialized WASM module
 */
export function getWasmModule(): any {
  if (!wasmInitialized || !wasmModule) {
    throw new Error('WASM module not initialized. Call initWasm() first.');
  }
  
  return wasmModule;
}

/**
 * Check if WASM is initialized
 */
export function isWasmInitialized(): boolean {
  return wasmInitialized;
}

/**
 * Check if WebHID is available in WASM
 */
export async function checkWebHIDInWasm(): Promise<boolean> {
  if (!wasmInitialized) {
    await initWasm();
  }
  
  try {
    // TODO: Call actual WASM function
    // return wasmModule.is_webhid_available();
    return isWebHIDAvailable();
  } catch {
    return false;
  }
}

/**
 * Get WASM version
 */
export function getWasmVersion(): string {
  if (!wasmInitialized || !wasmModule) {
    return 'not-initialized';
  }
  
  try {
    // TODO: Call actual WASM function
    // return wasmModule.version();
    return '1.2.1';
  } catch {
    return 'unknown';
  }
}

/**
 * Device ID management for WASM
 * Maps device identifiers to WASM device IDs
 */
const deviceIdMap = new Map<string, number>();

export function setWasmDeviceId(identifier: string, wasmId: number): void {
  deviceIdMap.set(identifier, wasmId);
}

export function getWasmDeviceId(identifier: string): number | undefined {
  return deviceIdMap.get(identifier);
}

export function clearWasmDeviceId(identifier: string): void {
  deviceIdMap.delete(identifier);
}

export function clearAllWasmDeviceIds(): void {
  deviceIdMap.clear();
}

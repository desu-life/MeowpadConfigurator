/**
 * WASM integration main export
 * Provides unified API that works in both Tauri and Web environments
 */

export * from './environment';
export * from './loader';

// Device-specific unified APIs
export * as Pure64 from './pure64';

// Re-export types
export type { IDeviceInfo, IDeviceStatus, IKeyHallConfig, IKeyRTStatus, KeyState } from '../apis';

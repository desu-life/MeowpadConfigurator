/**
 * Environment detection for Tauri vs Web
 */

declare global {
  interface Window {
    __TAURI__?: any;
  }
}

/**
 * Check if running in Tauri environment
 */
export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}

/**
 * Check if WebHID is available
 */
export function isWebHIDAvailable(): boolean {
  return typeof navigator !== 'undefined' && 'hid' in navigator;
}

/**
 * Get current runtime environment
 */
export type RuntimeEnvironment = 'tauri' | 'webhid' | 'unknown';

export function getRuntimeEnvironment(): RuntimeEnvironment {
  if (isTauri()) {
    return 'tauri';
  }
  if (isWebHIDAvailable()) {
    return 'webhid';
  }
  return 'unknown';
}

/**
 * Check if app can run in current environment
 */
export function isEnvironmentSupported(): boolean {
  const env = getRuntimeEnvironment();
  return env === 'tauri' || env === 'webhid';
}

/**
 * Get environment info for debugging
 */
export function getEnvironmentInfo() {
  return {
    environment: getRuntimeEnvironment(),
    isTauri: isTauri(),
    isWebHID: isWebHIDAvailable(),
    isSupported: isEnvironmentSupported(),
    userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : 'unknown',
  };
}

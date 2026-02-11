/**
 * WebHID device cache
 * Stores HIDDevice objects obtained during enumeration
 */

// Cache: key = "vendorId-productId", value = HIDDevice
const deviceCache = new Map<string, HIDDevice>();

/**
 * Store a device in cache
 */
export function cacheDevice(vendorId: number, productId: number, device: HIDDevice): void {
  const key = `${vendorId}-${productId}`;
  deviceCache.set(key, device);
  console.log(`[DeviceCache] Cached device ${key}`);
}

/**
 * Get a device from cache
 */
export function getCachedDevice(vendorId: number, productId: number): HIDDevice | undefined {
  const key = `${vendorId}-${productId}`;
  const device = deviceCache.get(key);
  console.log(`[DeviceCache] Get device ${key}:`, device ? 'found' : 'not found');
  return device;
}

/**
 * Clear all cached devices
 */
export function clearDeviceCache(): void {
  deviceCache.clear();
  console.log('[DeviceCache] Cleared all devices');
}

/**
 * Check if a device is cached
 */
export function hasDevice(vendorId: number, productId: number): boolean {
  const key = `${vendorId}-${productId}`;
  return deviceCache.has(key);
}

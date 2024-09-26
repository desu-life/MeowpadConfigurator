import { invoke } from "@tauri-apps/api/tauri";
import { IHidDeviceInfo, IVersion } from ".";

export async function connect_iap() {
  return (await invoke("connect_iap")) as void;
}
export async function iap_start(data: number[]) {
  return (await invoke("iap_start", { data })) as number;
  // Array.from(new Uint8Array(data))
}
export async function iap_flush() {
  return (await invoke("iap_flush")) as void;
}
export async function check_update(version: IVersion) {
  return (await invoke("check_update", { version })) as boolean;
}
export async function open_update_url(version: IVersion, str: string) {
  return (await invoke("open_update_url", { version, str })) as boolean;
}
export async function get_latest_version() {
  return (await invoke("get_latest_version")) as IVersion;
}
export async function get_theme() {
  return (await invoke("get_theme")) as string;
}
export async function device_list() {
  return (await invoke("device_list")) as IHidDeviceInfo[];
}
export async function refresh_devices() {
  return (await invoke("refresh_devices")) as boolean;
}
export async function connect_device(deviceInfo: IHidDeviceInfo) {
  return (await invoke("connect_device", { deviceInfo })) as boolean;
}
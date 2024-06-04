import { invoke } from "@tauri-apps/api/tauri";
import { IDeviceInfo, IDeviceStatus, IVersion } from ".";

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
export async function get_latest_version() {
  return (await invoke("get_latest_version")) as IVersion;
}
export async function get_theme() {
  return (await invoke("get_theme")) as string;
}
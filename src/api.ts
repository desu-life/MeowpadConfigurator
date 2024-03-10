import { invoke } from "@tauri-apps/api/tauri";
import {
  IDeviceInfo,
  IDeviceStatus,
  IKeyRTStatus,
  IKeyboard,
  ILighting,
  IVersion,
} from "./interface";

export async function calibration_key() {
  return (await invoke("calibration_key")) as void;
}
export async function get_debug_value() {
  return (await invoke("get_debug_value")) as IKeyRTStatus[];
}
export async function erase_firmware() {
  return (await invoke("erase_firmware")) as void;
}
export async function get_default_key_config() {
  return (await invoke("get_default_key_config")) as IKeyboard;
}
export async function get_default_light_config() {
  return (await invoke("get_default_light_config")) as ILighting;
}
export async function get_key_config() {
  return (await invoke("get_key_config")) as IKeyboard;
}
export async function get_light_config() {
  return (await invoke("get_light_config")) as ILighting;
}
export async function set_key_config(config: IKeyboard) {
  return (await invoke("set_key_config", { config })) as void;
}
export async function set_light_config(config: ILighting) {
  return (await invoke("set_light_config", { config })) as void;
}
export async function save_key_config() {
  return (await invoke("save_key_config")) as void;
}
export async function save_light_config() {
  return (await invoke("save_light_config")) as void;
}
export async function get_raw_config() {
  return (await invoke("get_raw_config")) as string;
}
export async function check_raw_config(config: string) {
  return (await invoke("check_raw_config", { config })) as boolean;
}
export async function save_raw_config(config: string) {
  return (await invoke("save_raw_config", { config })) as void;
}
export async function get_device_info() {
  return (await invoke("get_device_info")) as IDeviceInfo;
}
export async function get_device_status() {
  return (await invoke("get_device_status")) as IDeviceStatus;
}
export async function get_latest_version() {
  return (await invoke("get_latest_version")) as IVersion;
}
export async function get_firmware_4k_version() {
  return (await invoke("get_firmware_4k_version")) as string;
}
export async function check_update(version: IVersion) {
  return (await invoke("check_update", { version })) as boolean;
}
export async function connect() {
  return (await invoke("connect")) as void;
}
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

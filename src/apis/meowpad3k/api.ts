import { invoke } from "@tauri-apps/api/tauri";
import { IDebugValue, IKeyboard, ILighting } from "./config";
import { IDeviceInfo, IDeviceStatus } from "..";

export async function calibration_key() {
  return (await invoke("calibration_key_3k")) as void;
}
export async function get_debug_value() {
  return (await invoke("get_debug_value_3k")) as IDebugValue;
}
export async function erase_firmware() {
  return (await invoke("erase_firmware_3k")) as void;
}
export async function get_default_key_config() {
  return (await invoke("get_default_key_config_3k")) as IKeyboard;
}
export async function get_default_light_config() {
  return (await invoke("get_default_light_config_3k")) as ILighting;
}
export async function get_key_config() {
  return (await invoke("get_key_config_3k")) as IKeyboard;
}
export async function get_light_config() {
  return (await invoke("get_light_config_3k")) as ILighting;
}
export async function set_key_config(config: IKeyboard) {
  return (await invoke("set_key_config_3k", { config })) as void;
}
export async function set_light_config(config: ILighting) {
  return (await invoke("set_light_config_3k", { config })) as void;
}
export async function save_key_config() {
  return (await invoke("save_key_config_3k")) as void;
}
export async function save_light_config() {
  return (await invoke("save_light_config_3k")) as void;
}
export async function get_raw_config() {
  return (await invoke("get_raw_config_3k")) as string;
}
export async function check_raw_config(config: string) {
  return (await invoke("check_raw_config_3k", { config })) as boolean;
}
export async function save_raw_config(config: string) {
  return (await invoke("save_raw_config_3k", { config })) as void;
}
export async function connect() {
  return (await invoke("connect_3k")) as boolean;
}
export async function get_device_info() {
  return (await invoke("get_device_info_3k")) as IDeviceInfo;
}
export async function get_device_status() {
  return (await invoke("get_device_status_3k")) as IDeviceStatus;
}
export async function get_firmware_version() {
  return (await invoke("get_firmware_3k_version")) as string;
}

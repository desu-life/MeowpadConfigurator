import { invoke } from "@tauri-apps/api/tauri";
import { IDeviceInfo, IDeviceStatus, IKeyRTStatus } from "..";
import { IKeyboard, ILighting } from "./config";

export async function calibration_key() {
  return (await invoke("calibration_key_4k")) as void;
}
export async function get_debug_value() {
  return (await invoke("get_debug_value_4k")) as IKeyRTStatus[];
}
export async function erase_firmware() {
  return (await invoke("erase_firmware_4k")) as void;
}
export async function get_default_key_config() {
  return (await invoke("get_default_key_config_4k")) as IKeyboard;
}
export async function get_default_light_config() {
  return (await invoke("get_default_light_config_4k")) as ILighting;
}
export async function get_key_config() {
  return (await invoke("get_key_config_4k")) as IKeyboard;
}
export async function get_light_config() {
  return (await invoke("get_light_config_4k")) as ILighting;
}
export async function set_key_config(config: IKeyboard) {
  return (await invoke("set_key_config_4k", { config })) as void;
}
export async function set_light_config(config: ILighting) {
  return (await invoke("set_light_config_4k", { config })) as void;
}
export async function save_key_config() {
  return (await invoke("save_key_config_4k")) as void;
}
export async function save_light_config() {
  return (await invoke("save_light_config_4k")) as void;
}
export async function clear_config() {
  return (await invoke("clear_config_4k")) as void;
}
export async function get_raw_config() {
  return (await invoke("get_raw_config_4k")) as string;
}
export async function check_raw_config(config: string) {
  return (await invoke("check_raw_config_4k", { config })) as boolean;
}
export async function save_raw_config(config: string) {
  return (await invoke("save_raw_config_4k", { config })) as void;
}
export async function connect() {
  return (await invoke("connect_4k")) as boolean;
}
export async function reset_device() {
  return (await invoke("reset_device_4k")) as boolean;
}
export async function get_device_info() {
  return (await invoke("get_device_info_4k")) as IDeviceInfo;
}
export async function get_device_status() {
  return (await invoke("get_device_status_4k")) as IDeviceStatus;
}
export async function get_firmware_version() {
  return (await invoke("get_firmware_4k_version")) as string;
}

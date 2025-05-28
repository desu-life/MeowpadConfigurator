import { invoke } from "@tauri-apps/api/core";
import { IDebugValue, IKeyboard, ILighting } from "./config";
import { IDeviceInfo, IDeviceStatus } from "..";

export async function calibration_key() {
  return (await invoke("calibration_key_21se")) as void;
}
export async function get_debug_value() {
  return (await invoke("get_debug_value_21se")) as IDebugValue;
}
export async function erase_firmware() {
  return (await invoke("erase_firmware_21se")) as void;
}
export async function get_default_key_config() {
  return (await invoke("get_default_key_config_21se")) as IKeyboard;
}
export async function get_default_light_config() {
  return (await invoke("get_default_light_config_21se")) as ILighting;
}
export async function get_key_config() {
  return (await invoke("get_key_config_21se")) as IKeyboard;
}
export async function get_light_config() {
  return (await invoke("get_light_config_21se")) as ILighting;
}
export async function set_key_config(config: IKeyboard) {
  return (await invoke("set_key_config_21se", { config })) as void;
}
export async function set_light_config(config: ILighting) {
  return (await invoke("set_light_config_21se", { config })) as void;
}
export async function save_key_config() {
  return (await invoke("save_key_config_21se")) as void;
}
export async function save_light_config() {
  return (await invoke("save_light_config_21se")) as void;
}
export async function clear_config() {
  return (await invoke("clear_config_21se")) as void;
}
export async function get_raw_config() {
  return (await invoke("get_raw_config_21se")) as string;
}
export async function check_raw_config(config: string) {
  return (await invoke("check_raw_config_21se", { config })) as boolean;
}
export async function save_raw_config(config: string) {
  return (await invoke("save_raw_config_21se", { config })) as void;
}
export async function connect() {
  return (await invoke("connect_21se")) as boolean;
}
export async function reset_device() {
  return (await invoke("reset_device_21se")) as boolean;
}
export async function get_device_info() {
  return (await invoke("get_device_info_21se")) as IDeviceInfo;
}
export async function get_device_status() {
  return (await invoke("get_device_status_21se")) as IDeviceStatus;
}
export async function get_firmware_version() {
  return (await invoke("get_firmware_21se_version")) as string[];
}
export async function reset_middle_point() {
  return (await invoke("reset_middle_point_21se")) as void;
}
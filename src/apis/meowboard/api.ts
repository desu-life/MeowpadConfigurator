import { invoke } from "@tauri-apps/api/tauri";
import { IDeviceInfo, IDeviceStatus, IKeyHallConfig, IKeyRTStatus, KeyState } from "..";
import { IKeyboard } from "./config";

export async function calibration_key(keyIndexs: number[]) {
  return (await invoke("calibration_key_kb", { keyIndexs })) as void;
}
export async function get_debug_value() {
  return (await invoke("get_debug_value_kb")) as IKeyRTStatus[];
}
export async function get_debug_value_part(index: number) {
  return (await invoke("get_debug_value_part_kb", { index })) as IKeyRTStatus[];
}
export async function get_keystates() {
  return (await invoke("get_keystates_kb")) as KeyState[];
}
export async function get_keyvalues() {
  return (await invoke("get_keyvalues_kb")) as number[];
}
export async function get_key_calibrate_status() {
  return (await invoke("get_key_calibrate_status_kb")) as boolean[];
}
export async function get_default_key_config() {
  return (await invoke("get_default_key_config_kb")) as IKeyboard;
}
export async function get_key_config() {
  return (await invoke("get_key_config_kb")) as IKeyboard;
}
export async function set_key_config(config: IKeyboard) {
  return (await invoke("set_key_config_kb", { config })) as void;
}
export async function save_key_config() {
  return (await invoke("save_key_config_kb")) as void;
}
export async function save_light_config() {
  return (await invoke("save_light_config_kb")) as void;
}
export async function clear_config() {
  return (await invoke("clear_config_kb")) as void;
}
export async function get_raw_config() {
  return (await invoke("get_raw_config_kb")) as string;
}
export async function check_raw_config(config: string) {
  return (await invoke("check_raw_config_kb", { config })) as boolean;
}
export async function save_raw_config(config: string) {
  return (await invoke("save_raw_config_kb", { config })) as void;
}
export async function connect() {
  return (await invoke("connect_kb")) as boolean;
}
export async function reset_device() {
  return (await invoke("reset_device_kb")) as boolean;
}
export async function get_device_info() {
  return (await invoke("get_device_info_kb")) as IDeviceInfo;
}
export async function get_device_status() {
  return (await invoke("get_device_status_kb")) as IDeviceStatus;
}
export async function get_hall_config() {
  return (await invoke("get_hall_config_kb")) as IKeyHallConfig[];
}
export async function get_firmware_version() {
  return (await invoke("get_firmware_kb_version")) as string;
}
export async function erase_firmware() {
  return (await invoke("erase_firmware_kb")) as void;
}
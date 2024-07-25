import { IDeviceInfo, IDeviceStatus } from "../apis";
import { defineStore, acceptHMRUpdate } from "pinia";
import { IKeyboard as IKBB } from "../apis/meowboard/config";
import { Toggle } from "../interface";
import * as apib from '@/apis/meowboard/api'
import { Hex2Rgb, Rgb2Hex } from "@/utils";
import { KeyCode } from "@/keycode";

export const useDeviceStore = defineStore("device", () => {
  const connected = ref(false);
  const device_info = ref<IDeviceInfo | undefined>(undefined);
  const device_status = ref<IDeviceStatus | undefined>(undefined);

  const raw_config = ref<string | undefined>(undefined);

  // configs
  const device_config = ref<IKBB | undefined>(undefined);
  const max_brightness = ref<number>(0);
  const jitters_elimination_time = ref<number>(0);
  const continuous_report = ref<Toggle>(Toggle.Off);
  const hall_filter = ref<Toggle>(Toggle.Off);
  const fangwuchu = ref<Toggle>(Toggle.Off)


  async function try_connect() {
    if (await apib.connect()) {
      device_info.value = await apib.get_device_info()
      return true
    }
    return false;
  }
  
  async function get_status() {
      device_status.value = await apib.get_device_status()
  }

  async function get_config_raw() {
    raw_config.value = await apib.get_raw_config()
  }

  async function save_config_raw() {
    await apib.save_raw_config(raw_config.value!)
  }

  async function check_config_raw() {
    return await apib.check_raw_config(raw_config.value!)
    // 无设备连接时不检查，永远通过
    return true
  }

  function store_key_config() {
    let config = device_config as Ref<IKBB>;
    config.value!.jitters_elimination_time = Math.round(jitters_elimination_time.value * 8)
    config.value!.continuous_report = continuous_report.value == Toggle.On ? true : false
    config.value!.hall_filter = hall_filter.value == Toggle.On ? 1 : 0
    config.value!.max_brightness = Math.floor(max_brightness.value / 2)

    for (let i = 0; i < 64; i++) {
      config.value!.keys[i].release_dead_zone = fangwuchu.value == Toggle.On ? 5 : 0
    }
  }


  function extract_key_config() {
    let config = device_config as Ref<IKBB>;
    jitters_elimination_time.value = config.value!.jitters_elimination_time / 8
    continuous_report.value = config.value!.continuous_report == true ? Toggle.On : Toggle.Off
    hall_filter.value = config.value!.hall_filter == 1 ? Toggle.On : Toggle.Off
    max_brightness.value = Math.floor(config.value!.max_brightness * 2)

    for (let i = 0; i < 64; i++) {
      if (config.value!.keys[i].release_dead_zone == 5) {
        fangwuchu.value = Toggle.On
        break
      }
    }
  }
  
  async function get_config() {
    device_config.value = await apib.get_key_config()
    extract_key_config()
  }

  async function get_default_config() {
    device_config.value = await apib.get_default_key_config()
    extract_key_config()
  }

  async function sync_config() {
    store_key_config()
    await apib.set_key_config(device_config.value! as IKBB)
  }

  async function save_config() {
    await apib.save_key_config()
  }

  return {
    device_config,
    raw_config,
    connected,
    device_info,
    device_status,
    jitters_elimination_time,
    continuous_report,
    hall_filter,
    max_brightness,
    fangwuchu,
    try_connect,
    get_status,
    get_config_raw,
    check_config_raw,
    save_config_raw,
    get_config,
    get_default_config,
    sync_config,
    save_config,
  };
});


if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDeviceStore, import.meta.hot));
}

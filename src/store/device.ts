import { IDeviceInfo, IDeviceStatus } from "../apis";
import { defineStore, acceptHMRUpdate } from "pinia";
import { IKeyboard as IKB4K, ILighting as ILT4K } from "../apis/meowpad4k/config";
import { IKeyboard as IKB3K, ILighting as ILT3K, LightingMode as LM3K } from "../apis/meowpad3k/config";
import { Toggle } from "../interface";
import * as api4k from '@/apis/meowpad4k/api'
import * as api3k from '@/apis/meowpad3k/api'
import { Hex2Rgb, Rgb2Hex } from "@/utils";
import { KeyCode } from "@/keycode";

export const useDeviceStore = defineStore("device", () => {
  const connected = ref(false);
  const device_info = ref<IDeviceInfo | undefined>(undefined);
  const device_status = ref<IDeviceStatus | undefined>(undefined);

  const raw_config = ref<string | undefined>(undefined);

  // configs
  const key_config = ref<IKB4K | IKB3K | undefined>(undefined);
  const light_config = ref<ILT4K | ILT3K | undefined>(undefined);
  const led_colors = ref<string[] | null>(null);
  const high_speed_color = ref<string | null>(null);
  const low_speed_color = ref<string | null>(null);
  const max_brightness = ref<number>(0);
  const change_color_when_pressed = ref<Toggle>(Toggle.Off);
  const random_color_mode = ref<Toggle>(Toggle.Off);
  const is_flow_delay = ref<Toggle>(Toggle.Off);
  const enable_hs = ref<Toggle>(Toggle.Off);
  const jitters_elimination_time = ref<number>(0);
  const continuous_report = ref<Toggle>(Toggle.Off);
  const kalman_filter = ref<Toggle>(Toggle.Off);
  const enable_light = ref<Toggle>(Toggle.Off);

  function is_4k() {
    return device_info.value?.name == 'Meowpad'
  }

  function is_3k() {
    return device_info.value?.name == 'Meowpad SE v2'
  }

  async function try_connect() {
    if (await api4k.connect()) {
      device_info.value = await api4k.get_device_info()
      return true
    }
    if (await api3k.connect()) {
      device_info.value = await api3k.get_device_info()
      return true
    }
    return false;
  }
  
  async function get_status() {
    if (is_4k()) {
      device_status.value = await api4k.get_device_status()
    }
    if (is_3k()) {
      device_status.value = await api3k.get_device_status()
    }
  }

  async function get_config_raw() {
    if (is_4k()) {
      raw_config.value = await api4k.get_raw_config()
    }
    if (is_3k()) {
      raw_config.value = await api3k.get_raw_config()
    }
  }

  async function save_config_raw() {
    if (is_4k()) {
      await api4k.save_raw_config(raw_config.value!)
    }
    if (is_3k()) {
      await api3k.save_raw_config(raw_config.value!)
    }
  }

  async function check_config_raw() {
    if (is_4k()) {
      return await api4k.check_raw_config(raw_config.value!)
    }
    if (is_3k()) {
      return await api3k.check_raw_config(raw_config.value!)
    }
    // 无设备连接时不检查，永远通过
    return true
  }

  function store_key_config_4k() {
    let config = key_config as Ref<IKB4K>;
    for (let i = 0; i < config.value!.keys.length; i++) {
      while (config.value!.keys[i].key_data.length < 6) {
        config.value!.keys[i].key_data.push(KeyCode.None)
      }
  
      while (config.value!.keys[i].key_data.length > 6) {
        config.value!.keys[i].key_data.pop()
      }
    }
    config.value!.jitters_elimination_time = Math.round(jitters_elimination_time.value * 8)
    config.value!.continuous_report = continuous_report.value == Toggle.On ? true : false
    config.value!.kalman_filter = kalman_filter.value == Toggle.On ? true : false
    config.value!.enable_hs = enable_hs.value == Toggle.On ? true : false
  }

  function store_light_config_4k() {
    let config = light_config as Ref<ILT4K>;
    config.value!.led_colors = []
    for (let i = 0; i < led_colors.value!.length; i++) {
      config.value!.led_colors.push(Hex2Rgb(led_colors.value![i]))
    }
    config.value!.low_speed_color = Hex2Rgb(low_speed_color.value!)
    config.value!.high_speed_color = Hex2Rgb(high_speed_color.value!)
  
    config.value!.change_color_when_pressed = change_color_when_pressed.value == Toggle.On ? true : false
    config.value!.random_color_mode = random_color_mode.value == Toggle.On ? true : false
    config.value!.is_flow_delay = is_flow_delay.value == Toggle.On ? true : false
  
    config.value!.max_brightness = Math.round(max_brightness.value / 2)
  }

  function extract_key_config_4k() {
    let config = key_config as Ref<IKB4K>;
    jitters_elimination_time.value = config.value!.jitters_elimination_time / 8
    continuous_report.value = config.value!.continuous_report == true ? Toggle.On : Toggle.Off
    kalman_filter.value = config.value!.kalman_filter == true ? Toggle.On : Toggle.Off
    enable_hs.value = config.value!.enable_hs == true ? Toggle.On : Toggle.Off
    for (let i = 0; i < config.value.keys.length; i++) {
      config.value.keys[i].key_data = config.value.keys[i].key_data.filter(k => k != KeyCode.None)
    }
  }
  
  function extract_light_config_4k() {
    let config = light_config as Ref<ILT4K>;
    led_colors.value = []
    for (let i = 0; i < config.value!.led_colors.length; i++) {
      led_colors.value.push(Rgb2Hex(config.value!.led_colors[i]))
    }

    low_speed_color.value = Rgb2Hex(config.value!.low_speed_color)
    high_speed_color.value = Rgb2Hex(config.value!.high_speed_color)
    change_color_when_pressed.value = config.value!.change_color_when_pressed == true ? Toggle.On : Toggle.Off
    random_color_mode.value = config.value!.random_color_mode == true ? Toggle.On : Toggle.Off
    is_flow_delay.value = config.value!.is_flow_delay == true ? Toggle.On : Toggle.Off
    max_brightness.value = Math.floor(config.value!.max_brightness * 2)
  }

  
  function store_key_config_3k() {
    let config = key_config as Ref<IKB3K>;
    for (let i = 0; i < config.value!.keys.length; i++) {
      while (config.value!.keys[i].key_data.length < 6) {
        config.value!.keys[i].key_data.push(KeyCode.None)
      }
  
      while (config.value!.keys[i].key_data.length > 6) {
        config.value!.keys[i].key_data.pop()
      }
    }

    while (config.value!.side_btn.length < 6) {
      config.value!.side_btn.push(KeyCode.None)
    }

    while (config.value!.side_btn.length > 6) {
      config.value!.side_btn.pop()
    }

    config.value!.jitters_elimination_time = Math.round(jitters_elimination_time.value)
    config.value!.continuous_report = continuous_report.value == Toggle.On ? true : false
    config.value!.kalman_filter = kalman_filter.value == Toggle.On ? true : false
  }

  function store_light_config_3k() {
    let config = light_config as Ref<ILT3K>;
    config.value!.led_colors = []
    for (let i = 0; i < led_colors.value!.length; i++) {
      config.value!.led_colors.push(Hex2Rgb(led_colors.value![i]))
    }

    config.value!.max_brightness = Math.round(max_brightness.value / 2)
    config.value!.lighting_mode = enable_light.value == Toggle.On ? LM3K.Solid : LM3K.Off
  }

  function extract_key_config_3k() {
    let config = key_config as Ref<IKB3K>;
    jitters_elimination_time.value = config.value!.jitters_elimination_time
    continuous_report.value = config.value!.continuous_report == true ? Toggle.On : Toggle.Off
    kalman_filter.value = config.value!.kalman_filter == true ? Toggle.On : Toggle.Off
    for (let i = 0; i < config.value.keys.length; i++) {
      config.value.keys[i].key_data = config.value.keys[i].key_data.filter(k => k != KeyCode.None)
    }
    config.value.side_btn = config.value.side_btn.filter(k => k != KeyCode.None)
  }

  function extract_light_config_3k() {
    let config = light_config as Ref<ILT3K>;
    led_colors.value = []
    for (let i = 0; i < config.value!.led_colors.length; i++) {
      led_colors.value.push(Rgb2Hex(config.value!.led_colors[i]))
    }
    max_brightness.value = Math.floor(config.value!.max_brightness * 2)
    enable_light.value = config.value!.lighting_mode == LM3K.Solid ? Toggle.On : Toggle.Off
  }

  async function get_config() {
    if (is_4k()) {
      key_config.value = await api4k.get_key_config()
      extract_key_config_4k()
      light_config.value = await api4k.get_light_config()
      extract_light_config_4k()
    }
    if (is_3k()) {
      key_config.value = await api3k.get_key_config()
      extract_key_config_3k()
      light_config.value = await api3k.get_light_config()
      extract_light_config_3k()
    }
  }

  async function get_default_config() {
    if (is_4k()) {
      key_config.value = await api4k.get_default_key_config()
      extract_key_config_4k()
      light_config.value = await api4k.get_default_light_config()
      extract_light_config_4k()
    }
    if (is_3k()) {
      key_config.value = await api3k.get_default_key_config()
      extract_key_config_3k()
      light_config.value = await api3k.get_default_light_config()
      extract_light_config_3k()
    }
  }

  async function sync_config() {
    if (is_4k()) {
      store_key_config_4k()
      await api4k.set_key_config(key_config.value! as IKB4K)
      store_light_config_4k()
      await api4k.set_light_config(light_config.value! as ILT4K)
    }
    if (is_3k()) {
      store_key_config_3k()
      await api3k.set_key_config(key_config.value! as IKB3K)
      store_light_config_3k()
      await api3k.set_light_config(light_config.value! as ILT3K)
    }
  }

  async function save_config() {
    if (is_4k()) {
      await api4k.save_key_config()
      await api4k.save_light_config()
    }
    if (is_3k()) {
      await api3k.save_key_config()
      await api3k.save_light_config()
    }
  }

  async function calibration_key() {
    if (is_4k()) {
      await api4k.calibration_key()
    }
    if (is_3k()) {
      await api3k.calibration_key()
    }
  }

  return {
    key_config,
    light_config,
    raw_config,
    connected,
    device_info,
    device_status,
    led_colors,
    low_speed_color,
    high_speed_color,
    jitters_elimination_time,
    continuous_report,
    kalman_filter,
    enable_hs,
    max_brightness,
    change_color_when_pressed,
    random_color_mode,
    is_flow_delay,
    enable_light,
    is_4k,
    is_3k,
    try_connect,
    get_status,
    get_config_raw,
    check_config_raw,
    save_config_raw,
    get_config,
    get_default_config,
    sync_config,
    save_config,
    calibration_key,
  };
});


if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDeviceStore, import.meta.hot));
}

import { IDeviceInfo, IDeviceStatus, IHidDeviceInfo, ISOCDKeyPairs } from "@/apis";
import { defineStore, acceptHMRUpdate } from "pinia";
import { IKeyboard as IKBV2, ILighting as ILTV2 } from "@/apis/meowpadv2/config";
import { IKeyboard as IKBV2SE, ILighting as ILTV2SE, LightingMode as LMV2SE } from "@/apis/meowpadv2se/config";
import { IKeyboard as IKBV21SE, ILighting as ILTV21SE, LightingMode as LMV21SE } from "@/apis/meowpadv21se/config";
import { IKeyboard as IKBP64 } from "@/apis/pure64/config";
import { IKeyboard as IKBV3 } from "@/apis/meowpadv3/config";
import { Toggle } from "../interface";
import * as apiv2 from '@/apis/meowpadv2/api'
import * as apiv2se from '@/apis/meowpadv2se/api'
import * as apiv21se from '@/apis/meowpadv21se/api'
import * as apiv3 from '@/apis/meowpadv3/api'
import * as apib from '@/apis/pure64/api'
import { Hex2Rgb, Rgb2Hex } from "@/utils";
import { KeyCode } from "@/keycode";

export const useDeviceStore = defineStore("device", () => {
  const connected = ref(false);
  const device_info = ref<IDeviceInfo | undefined>(undefined);
  const device_status = ref<IDeviceStatus | undefined>(undefined);
  const device_hid_info = ref<IHidDeviceInfo | undefined>(undefined);

  const raw_config = ref<string | undefined>(undefined);

  // configs
  const device_config = ref<any | undefined>(undefined);
  const key_config = ref<any | undefined>(undefined);
  const light_config = ref<any | undefined>(undefined);
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
  const key_proof = ref<Toggle>(Toggle.Off);
  const auto_calibration = ref<Toggle>(Toggle.Off);
  const hall_filter = ref<number>(0);
  const scod_pairs = ref<ISOCDKeyPairs[]>([]);

  function is_v2() {
    return device_hid_info.value?.device_name == 'Meowpad'
  }

  function is_v2se() {
    return device_hid_info.value?.device_name == 'Meowpad SE v2'
  }

  function is_pure() {
    return device_hid_info.value?.product_id == 0xFB01
  }

  function is_v21se() {
    return device_hid_info.value?.device_name == 'Meowpad SE v2.1'
  }

  function is_v3() {
    return device_hid_info.value?.product_id == 0xFB02
  }

  async function try_connect() {
    if (await apiv2.connect()) {
      device_info.value = await apiv2.get_device_info()
      return true
    }
    if (await apiv2se.connect()) {
      device_info.value = await apiv2se.get_device_info()
      return true
    }
    if (await apib.connect()) {
      device_info.value = await apib.get_device_info()
      return true
    }
    if (await apiv21se.connect()) {
      device_info.value = await apiv21se.get_device_info()
      return true
    }
    if (await apiv3.connect()) {
      device_info.value = await apiv3.get_device_info()
      return true
    }
    return false;
  }
  
  async function get_info() {
    if (is_v2()) {
      device_info.value = await apiv2.get_device_info()
    }
    if (is_v2se()) {
      device_info.value = await apiv2se.get_device_info()
    }
    if (is_pure()) {
      device_info.value = await apib.get_device_info()
    }
    if (is_v21se()) {
      device_info.value = await apiv21se.get_device_info()
    }
    if (is_v3()) {
      device_info.value = await apiv3.get_device_info()
    }
  }

  
  async function get_status() {
    if (is_v2()) {
      device_status.value = await apiv2.get_device_status()
    }
    if (is_v2se()) {
      device_status.value = await apiv2se.get_device_status()
    }
    if (is_pure()) {
      device_status.value = await apib.get_device_status()
    }
    if (is_v21se()) {
      device_status.value = await apiv21se.get_device_status()
    }
    if (is_v3()) {
      device_status.value = await apiv3.get_device_status()
    }
  }

  async function get_config_raw() {
    if (is_v2()) {
      raw_config.value = await apiv2.get_raw_config()
    }
    if (is_v2se()) {
      raw_config.value = await apiv2se.get_raw_config()
    }
    if (is_pure()) {
      raw_config.value = await apib.get_raw_config()
    }
    if (is_v21se()) {
      raw_config.value = await apiv21se.get_raw_config()
    }
    if (is_v3()) {
      raw_config.value = await apiv3.get_raw_config()
    }
  }

  async function save_config_raw() {
    if (is_v2()) {
      await apiv2.save_raw_config(raw_config.value!)
    }
    if (is_v2se()) {
      await apiv2se.save_raw_config(raw_config.value!)
    }
    if (is_pure()) {
      await apib.save_raw_config(raw_config.value!)
    }
    if (is_v21se()) {
      await apiv21se.save_raw_config(raw_config.value!)
    }
    if (is_v3()) {
      await apiv3.save_raw_config(raw_config.value!)
    }
  }

  async function check_config_raw() {
    if (is_v2()) {
      return await apiv2.check_raw_config(raw_config.value!)
    }
    if (is_v2se()) {
      return await apiv2se.check_raw_config(raw_config.value!)
    }
    if (is_pure()) {
      return await apib.check_raw_config(raw_config.value!)
    }
    if (is_v21se()) {
      return await apiv21se.check_raw_config(raw_config.value!)
    }
    if (is_v3()) {
      return await apiv3.check_raw_config(raw_config.value!)
    }
    // 无设备连接时不检查，永远通过
    return true
  }

  function store_key_config_pure64() {
    let config = device_config as Ref<IKBP64>;
    config.value!.jitters_elimination_time = Math.round(jitters_elimination_time.value * 8)
    config.value!.high_reportrate = enable_hs.value == Toggle.On ? true : false
    config.value!.key_proof = key_proof.value == Toggle.On ? true : false
    config.value!.auto_calibration = auto_calibration.value == Toggle.On ? true : false
    config.value!.hall_filter = hall_filter.value
    config.value!.max_brightness = Math.floor(max_brightness.value / 2)

    config.value!.led_color = Hex2Rgb(led_colors.value![0])

    for (let i = 0; i < 64; i++) {
      config.value!.keys[i].release_dead_zone = Math.floor(config.value!.keys[i].release_dead_zone * 2)
      config.value!.keys[i].press_percentage = Math.floor(config.value!.keys[i].press_percentage * 2)
      config.value!.keys[i].release_percentage = Math.floor(config.value!.keys[i].release_percentage * 2)
      config.value!.keys[i].dead_zone = Math.floor(config.value!.keys[i].dead_zone * 2)
    }

    if (config.value!.socd_key_pairs) {
      config.value!.socd_key_pairs = []
      const len = Math.min(5, scod_pairs.value!.length);
      for (let i = 0; i < len; i++) {
        if (scod_pairs.value![i].key1 < 0 || scod_pairs.value![i].key1 > 63 ||
            scod_pairs.value![i].key2 < 0 || scod_pairs.value![i].key2 > 63) {
          // 如果有不合法的按键对，清空所有配对
          config.value!.socd_key_pairs = []
          break;
        }
  
        config.value!.socd_key_pairs.push({
          key1: scod_pairs.value![i].key1,
          key2: scod_pairs.value![i].key2,
        })
      }
    }

  }


  function extract_key_config_pure64() {
    let config = device_config as Ref<IKBP64>;

    jitters_elimination_time.value = config.value!.jitters_elimination_time / 8
    enable_hs.value = config.value!.high_reportrate == true ? Toggle.On : Toggle.Off
    key_proof.value = config.value!.key_proof == true ? Toggle.On : Toggle.Off
    auto_calibration.value = config.value!.auto_calibration == true ? Toggle.On : Toggle.Off
    hall_filter.value = config.value!.hall_filter
    max_brightness.value = Math.floor(config.value!.max_brightness * 2)

    led_colors.value = []
    led_colors.value.push(Rgb2Hex(config.value!.led_color))

    for (let i = 0; i < 64; i++) {
      config.value!.keys[i].press_percentage = config.value!.keys[i].press_percentage / 2
      config.value!.keys[i].release_percentage = config.value!.keys[i].release_percentage / 2
      config.value!.keys[i].dead_zone = config.value!.keys[i].dead_zone / 2
      config.value!.keys[i].release_dead_zone = config.value!.keys[i].release_dead_zone / 2
    }

    scod_pairs.value = []
    if (config.value!.socd_key_pairs) {
      for (let i = 0; i < config.value!.socd_key_pairs.length; i++) {
        scod_pairs.value.push({
          key1: config.value!.socd_key_pairs[i].key1,
          key2: config.value!.socd_key_pairs[i].key2,
        })
      }
    }
  }
  

  function store_key_config_v2() {
    let config = key_config as Ref<IKBV2>;
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

  function store_light_config_v2() {
    let config = light_config as Ref<ILTV2>;
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

  function extract_key_config_v2() {
    let config = key_config as Ref<IKBV2>;
    jitters_elimination_time.value = config.value!.jitters_elimination_time / 8
    continuous_report.value = config.value!.continuous_report == true ? Toggle.On : Toggle.Off
    kalman_filter.value = config.value!.kalman_filter == true ? Toggle.On : Toggle.Off
    enable_hs.value = config.value!.enable_hs == true ? Toggle.On : Toggle.Off
    for (let i = 0; i < config.value.keys.length; i++) {
      config.value.keys[i].key_data = config.value.keys[i].key_data.filter(k => k != KeyCode.None)
    }
  }
  
  function extract_light_config_v2() {
    let config = light_config as Ref<ILTV2>;
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

  
  function store_key_config_v2se() {
    let config = key_config as Ref<IKBV2SE>;
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

  function store_light_config_v2se() {
    let config = light_config as Ref<ILTV2SE>;
    config.value!.led_colors = []
    for (let i = 0; i < led_colors.value!.length; i++) {
      config.value!.led_colors.push(Hex2Rgb(led_colors.value![i]))
    }

    config.value!.max_brightness = Math.round(max_brightness.value / 2)
    config.value!.lighting_mode = enable_light.value == Toggle.On ? LMV2SE.Solid : LMV2SE.Off
  }

  function extract_key_config_v2se() {
    let config = key_config as Ref<IKBV2SE>;
    jitters_elimination_time.value = config.value!.jitters_elimination_time
    continuous_report.value = config.value!.continuous_report == true ? Toggle.On : Toggle.Off
    kalman_filter.value = config.value!.kalman_filter == true ? Toggle.On : Toggle.Off
    for (let i = 0; i < config.value.keys.length; i++) {
      config.value.keys[i].key_data = config.value.keys[i].key_data.filter(k => k != KeyCode.None)
    }
    config.value.side_btn = config.value.side_btn.filter(k => k != KeyCode.None)
  }

  function extract_light_config_v2se() {
    let config = light_config as Ref<ILTV2SE>;
    led_colors.value = []
    for (let i = 0; i < config.value!.led_colors.length; i++) {
      led_colors.value.push(Rgb2Hex(config.value!.led_colors[i]))
    }
    max_brightness.value = Math.floor(config.value!.max_brightness * 2)
    enable_light.value = config.value!.lighting_mode == LMV2SE.Solid ? Toggle.On : Toggle.Off
  }
  
  function store_key_config_v21se() {
    let config = key_config as Ref<IKBV21SE>;
    for (let i = 0; i < config.value!.hall_keys.length; i++) {
      while (config.value!.hall_keys[i].key_data.length < 6) {
        config.value!.hall_keys[i].key_data.push(KeyCode.None)
      }
  
      while (config.value!.hall_keys[i].key_data.length > 6) {
        config.value!.hall_keys[i].key_data.pop()
      }
    }

    for (let i = 0; i < config.value!.normal_keys.length; i++) {
      while (config.value!.normal_keys[i].key_data.length < 6) {
        config.value!.normal_keys[i].key_data.push(KeyCode.None)
      }
  
      while (config.value!.normal_keys[i].key_data.length > 6) {
        config.value!.normal_keys[i].key_data.pop()
      }
    }

    config.value!.jitters_elimination_time = Math.round(jitters_elimination_time.value * 8)
    config.value!.continuous_report = continuous_report.value == Toggle.On ? true : false
    config.value!.kalman_filter = kalman_filter.value == Toggle.On ? true : false
  }

  function store_light_config_v21se() {
    let config = light_config as Ref<ILTV21SE>;
    config.value!.led_colors = []
    for (let i = 0; i < led_colors.value!.length; i++) {
      config.value!.led_colors.push(Hex2Rgb(led_colors.value![i]))
    }

    config.value!.max_brightness = Math.round(max_brightness.value / 2)
  }

  function extract_key_config_v21se() {
    let config = key_config as Ref<IKBV21SE>;
    jitters_elimination_time.value = config.value!.jitters_elimination_time / 8
    continuous_report.value = config.value!.continuous_report == true ? Toggle.On : Toggle.Off
    kalman_filter.value = config.value!.kalman_filter == true ? Toggle.On : Toggle.Off
    for (let i = 0; i < config.value.hall_keys.length; i++) {
      config.value.hall_keys[i].key_data = config.value.hall_keys[i].key_data.filter(k => k != KeyCode.None)
    }
    for (let i = 0; i < config.value.normal_keys.length; i++) {
      config.value.normal_keys[i].key_data = config.value.normal_keys[i].key_data.filter(k => k != KeyCode.None)
    }
  }

  function extract_light_config_v21se() {
    let config = light_config as Ref<ILTV21SE>;
    led_colors.value = []
    for (let i = 0; i < config.value!.led_colors.length; i++) {
      led_colors.value.push(Rgb2Hex(config.value!.led_colors[i]))
    }
    max_brightness.value = Math.floor(config.value!.max_brightness * 2)
  }

  function store_key_config_v3() {
    let config = device_config as Ref<IKBV3>;
    config.value!.high_reportrate = enable_hs.value == Toggle.On ? true : false
    config.value!.key_proof = key_proof.value == Toggle.On ? true : false
    config.value!.auto_calibration = auto_calibration.value == Toggle.On ? true : false
    config.value!.hall_filter = hall_filter.value
    config.value!.max_brightness = Math.round(max_brightness.value / 2)

    config.value!.led_color = Hex2Rgb(led_colors.value![0])

    for (let i = 0; i < config.value!.layer.length; i++) {
      while (config.value!.layer[i].length < 3) {
        config.value!.layer[i].push({ t: "None" })
      }
    }

    if (config.value!.socd_key_pairs) {
      config.value!.socd_key_pairs = []
      const len = Math.min(5, scod_pairs.value!.length);
      for (let i = 0; i < len; i++) {
        if (scod_pairs.value![i].key1 < 0 || scod_pairs.value![i].key1 > 63 ||
            scod_pairs.value![i].key2 < 0 || scod_pairs.value![i].key2 > 63) {
          // 如果有不合法的按键对，清空所有配对
          config.value!.socd_key_pairs = []
          break;
        }
  
        config.value!.socd_key_pairs.push({
          key1: scod_pairs.value![i].key1,
          key2: scod_pairs.value![i].key2,
        })
      }
    }

  }


  function extract_key_config_v3() {
    let config = device_config as Ref<IKBV3>;

    enable_hs.value = config.value!.high_reportrate == true ? Toggle.On : Toggle.Off
    key_proof.value = config.value!.key_proof == true ? Toggle.On : Toggle.Off
    auto_calibration.value = config.value!.auto_calibration == true ? Toggle.On : Toggle.Off
    hall_filter.value = config.value!.hall_filter
    max_brightness.value = Math.round(config.value!.max_brightness * 2)

    led_colors.value = []
    led_colors.value.push(Rgb2Hex(config.value!.led_color))

    scod_pairs.value = []
    if (config.value!.socd_key_pairs) {
      for (let i = 0; i < config.value!.socd_key_pairs.length; i++) {
        scod_pairs.value.push({
          key1: config.value!.socd_key_pairs[i].key1,
          key2: config.value!.socd_key_pairs[i].key2,
        })
      }
    }
  }


  return {
    device_hid_info,
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
    hall_filter,
    key_proof,
    auto_calibration,
    device_config,
    scod_pairs,
    is_v2,
    is_v2se,
    is_v21se,
    is_v3,
    is_pure,
    try_connect,
    get_info,
    get_status,
    get_config_raw,
    check_config_raw,
    save_config_raw,
    extract_key_config_v21se,
    store_key_config_v21se,
    extract_light_config_v21se,
    store_light_config_v21se,
    extract_key_config_v2se,
    store_key_config_v2se,
    extract_light_config_v2se,
    store_light_config_v2se,
    extract_key_config_v2,
    store_key_config_v2,
    extract_light_config_v2,
    store_light_config_v2,
    store_key_config_pure64,
    extract_key_config_pure64,
    store_key_config_v3,
    extract_key_config_v3,

  };
});


if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDeviceStore, import.meta.hot));
}

import { IDeviceInfo, IDeviceStatus } from "../apis";
import { defineStore, acceptHMRUpdate } from "pinia";
import { IKeyboard, ILighting } from "../apis/meowpad4k/config";
import { Toggle } from "../interface";

export const useDeviceStore = defineStore("device", () => {
  const connected = ref(false);
  const device_info = ref<IDeviceInfo | undefined>(undefined);
  const device_status = ref<IDeviceStatus | undefined>(undefined);

  const raw_config = ref<string | undefined>(undefined);

  // 4k configs
  const key_config = ref<IKeyboard | undefined>(undefined);
  const light_config = ref<ILighting | undefined>(undefined);
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

  function is_4k() {
    return device_info.value?.name == 'Meowpad'
  }
  function is_3k() {
    return device_info.value?.name == 'Meowpad3K'
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
    is_4k,
    is_3k
  };
});


if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDeviceStore, import.meta.hot));
}

import { IDeviceInfo, IDeviceStatus, IKeyboard, ILighting, Toggle } from './interface';
import { defineStore, acceptHMRUpdate } from "pinia";
import { IVersion, LightingMode } from "@/interface";
import { invoke } from "@tauri-apps/api/tauri";
import { Type } from 'naive-ui/es/button/src/interface';

export const useStore = defineStore("main", () => {
  const count = ref(0);
  function increment() {
    count.value++;
  }

  const loading = ref(false);
  const connected = ref(false);
  const iap_connected = ref(false);
  const key_config = ref<IKeyboard | undefined>(undefined);
  const light_config = ref<ILighting | undefined>(undefined);
  const raw_config = ref<string | undefined>(undefined);
  const led_colors = ref<string[] | null>(null);
  const low_speed_color = ref<string | null>(null);
  const high_speed_color = ref<string | null>(null);
  const version_info = ref<IVersion | undefined>(undefined);
  const device_info = ref<IDeviceInfo | undefined>(undefined);
  const device_status = ref<IDeviceStatus | undefined>(undefined);
  const need_update_firmware = ref<boolean>(false);
  const developer_mode = ref<boolean>(false);
  const debug_mode = ref<boolean>(false);
  const can_sync = ref<boolean>(false);
  const jitters_elimination_time = ref<number>(0);
  const continuous_report = ref<Toggle>(Toggle.Off);
  const kalman_filter = ref<Toggle>(Toggle.Off);
  const change_color_when_pressed = ref<Toggle>(Toggle.Off);
  const random_color_mode = ref<Toggle>(Toggle.Off);
  const is_flow_delay = ref<Toggle>(Toggle.Off);
  const enable_hs = ref<Toggle>(Toggle.Off);
  const max_brightness = ref<number>(0);
  const status = ref<Type | undefined>(undefined)
  const status_str = ref("")
  const latest_firmware_download_url = ref<string | null>(null);

  return {
    loading,
    key_config,
    light_config,
    raw_config,
    connected,
    iap_connected,
    version_info,
    device_info,
    device_status,
    need_update_firmware,
    developer_mode,
    debug_mode,
    can_sync,
    led_colors,
    low_speed_color,
    high_speed_color,
    jitters_elimination_time,
    continuous_report,
    kalman_filter,
    enable_hs,
    max_brightness,
    status,
    status_str,
    change_color_when_pressed,
    random_color_mode,
    is_flow_delay,
    latest_firmware_download_url
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}

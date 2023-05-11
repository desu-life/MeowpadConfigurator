import { IDevice, IHsData } from './interface';
import { defineStore, acceptHMRUpdate } from "pinia";
import { IConfig, IVersion, LightingMode } from "@/interface";
import { invoke } from "@tauri-apps/api/tauri";

export const useStore = defineStore("main", () => {
  const count = ref(0);
  function increment() {
    count.value++;
  }

  const loading = ref(false);
  const connected = ref(false);
  const config = ref<IConfig | undefined>(undefined);
  const raw_config = ref<string | undefined>(undefined);
  const led_color_l = ref<string | null>(null);
  const led_color_r = ref<string | null>(null);
  const led_color_btm_l = ref<string | undefined>(undefined);
  const led_color_btm_r = ref<string | undefined>(undefined);
  const speed_press_high_color = ref<string | undefined>(undefined);
  const speed_press_low_color = ref<string | undefined>(undefined);
  const breath_speed = ref<number | undefined>(undefined);
  const rainbow_light_switching_speed = ref<number | undefined>(undefined);
  const firmware_version = ref<string>();
  invoke("get_firmware_version").then((v: string) => firmware_version.value = v)
  const version_info = ref<IVersion | undefined>(undefined);
  const device_info = ref<IDevice | undefined>(undefined);
  const need_update_firmware = ref<boolean>(false);
  const is_hs = ref<boolean>(false);
  const debug_mode = ref<boolean>(false);
  const in_debug = ref<boolean>(false);
  const can_sync = ref<boolean>(false);
  const debug_str = ref<string>("");
  const adc_data = ref<IHsData[]>([
    {
      dyn: 0,
      max: 0,
      min: 10000,
      fixed: 0,
    },
    {
      dyn: 0,
      max: 0,
      min: 10000,
      fixed: 0,
    }
  ]);

  return {
    loading,
    config,
    raw_config,
    connected,
    led_color_btm_l,
    led_color_btm_r,
    speed_press_high_color,
    speed_press_low_color,
    led_color_l,
    led_color_r,
    breath_speed,
    rainbow_light_switching_speed,
    firmware_version,
    version_info,
    device_info,
    need_update_firmware,
    is_hs,
    debug_mode,
    debug_str,
    in_debug,
    can_sync,
    adc_data
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}

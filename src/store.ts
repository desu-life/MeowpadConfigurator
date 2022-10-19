import { defineStore, acceptHMRUpdate } from "pinia";
import { IConfig, LightingMode } from "@/interface";

export const useStore = defineStore("main", () => {
  const count = ref(0);
  function increment() {
    count.value++;
  }

  const loading = ref(false);
  const connected = ref(false);
  const config = ref<IConfig | undefined>(undefined);
  const led_color_l = ref<string | null>(null);
  const led_color_r = ref<string | null>(null);
  const led_color_btm_l = ref<string | undefined>(undefined);
  const led_color_btm_r = ref<string | undefined>(undefined);
  const speed_press_high_color = ref<string | undefined>(undefined);
  const speed_press_low_color = ref<string | undefined>(undefined);

  return {
    loading,
    config,
    connected,
    led_color_btm_l,
    led_color_btm_r,
    speed_press_high_color,
    speed_press_low_color,
    led_color_l,
    led_color_r,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}

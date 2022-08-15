import { defineStore, acceptHMRUpdate } from "pinia";
import { IConfig } from "@/interface";

export const useStore = defineStore("main", () => {
  const count = ref(0);
  function increment() {
    count.value++;
  }
  
  const loading = ref(false)
  const connected = ref(false)
//   const config = ref<IConfig | undefined>({
//     breath_interval: 8,
//     breath_maximum_light_duration: 1200,
//     breath_minimum_light_duration: 40,
//     fade_light_switching_speed: 1200,
//     k1: 29,
//     k2: 27,
//     k3: 41,
//     k4: 59,
//     k5: 53,
//     led1: {b: 229, g: 255, r: 204},
//     led2: {b: 204, g: 153, r: 255},
//     lighting_mode: "RainbowBreath",
//     maximum_brightness: 255,
//     minimum_brightness: 5,
//     press_light_duration: 5,
//     press_light_maximum_brightness: 255,
//     press_light_minimum_brightness: 0
//   });
  const config = ref<IConfig | undefined>(undefined);

  return { loading, config, connected };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}

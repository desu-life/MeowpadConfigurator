<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { Type } from "naive-ui/es/button/src/interface"
import { useStore } from '@/store'
import { IConfig, IDevice } from "@/interface";
import { Rgb2Hex, Hex2Rgb } from '@/utils';
// defineProps<{ msg: string }>()



const store = useStore()
const status = ref<Type | undefined>(undefined)
const status_str = ref("设备未连接")
const reset = ref(false)

async function connect() {
  store.loading = true
  status.value = "warning"
  status_str.value = "连接中"
  try {
    const res = await invoke("connect")
    let info = await check_device_info()
    console.table(info)
    store.device_info = info
    
    
    // if (store.device_info!.version != store.firmware_version) {
    //   store.need_update_firmware = true // 需要更新固件
    //   store.loading = false
    //   status.value = "error"
    //   status_str.value = "设备版本 " + info!.version + " 与本程序不匹配，请升级固件至 " + store.firmware_version
    //   return
    // }

    // 不管怎么样总之是连上了
    store.connected = true
    
    status.value = "success"
    if (info === undefined) {
      status_str.value = "设备已连接"
    } else {
      status_str.value = "设备已连接，固件版本：" + info!.version
    }
  } catch (e) {
    store.connected = false
    status.value = "error"
    status_str.value = "连接失败，错误原因：" + e
    console.error(e)
  }
  await get_config()
  store.loading = false
}

async function check_device_info(): Promise<IDevice | undefined> {
  try {
    const res: IDevice = await invoke("get_device_info")
    return res
  } catch (e) {
    store.connected = false
    status.value = "error"
    status_str.value = "获取设备信息失败，错误原因：" + e
    console.error(e)
  }
}

async function calibration_key() {
  store.loading = true
  status.value = "info"
  status_str.value = "请同时按下两个按键并保持2秒后松开，即可完成校准过程"
  try {
    await invoke("calibration_key")
  } catch (e) {
    store.connected = false
    status.value = "error"
    status_str.value = "连接出错，错误原因：" + e
    console.error(e)
  }
  store.loading = false
  setTimeout(async () => {
    // 清空显示
    if (status_str.value = "请同时按下两个按键并保持2秒后松开，即可完成校准过程") {
      status.value = "success"
      if (store.device_info === undefined) {
        status_str.value = "设备已连接"
      } else {
        status_str.value = "设备已连接，固件版本：" + store.device_info!.version
      }
    }
  }, 5000)
}

async function get_default_config() {
  store.loading = true
  try {
    const res: IConfig = await invoke("get_default_config")
    console.dir(res)
    store.led_color_l = Rgb2Hex(res.led_color_l)
    store.led_color_r = Rgb2Hex(res.led_color_r)
    store.led_color_btm_l = Rgb2Hex(res.led_color_btm_l)
    store.led_color_btm_r = Rgb2Hex(res.led_color_btm_r)
    store.speed_press_high_color = Rgb2Hex(res.speed_press_high_color)
    store.speed_press_low_color = Rgb2Hex(res.speed_press_low_color)
    store.breath_speed = 20 - res.breath_interval
    store.rainbow_light_switching_speed = 30 - res.rainbow_light_switching_interval
    store.config = res
    status.value = "success"
    status_str.value = "已重置配置，同步以提交更改"
  } catch (e) {
    status.value = "error"
    status_str.value = "获取默认值失败，错误原因：" + e
    console.error(e)
  }
  store.loading = false
}

async function get_config() {
  store.loading = true
  try {
    const res: IConfig = await invoke("get_config")
    console.dir(res)
    store.config = res
    store.led_color_l = Rgb2Hex(res.led_color_l)
    store.led_color_r = Rgb2Hex(res.led_color_r)
    store.led_color_btm_l = Rgb2Hex(res.led_color_btm_l)
    store.led_color_btm_r = Rgb2Hex(res.led_color_btm_r)
    store.speed_press_high_color = Rgb2Hex(res.speed_press_high_color)
    store.speed_press_low_color = Rgb2Hex(res.speed_press_low_color)
    store.breath_speed = 20 - res.breath_interval
    store.rainbow_light_switching_speed = 30 - res.rainbow_light_switching_interval
  } catch (e) {
    const es = e as string
    status.value = "error"
    status_str.value = es
    console.error(es)
    if (es.includes("Semantic") || es.includes("Syntax") || es.includes("Unexpected")) {
      status_str.value = "检测到设备配置数据错误，将在五秒后自动重置"
      setTimeout(async () => {
        await get_default_config()
        await sync_config()
      }, 5000)
    }
  }
  store.loading = false
}

async function sync_config() {
  store.loading = true
  status.value = "warning"
  status_str.value = "正在同步配置"
  try {
    store.config!.led_color_l = Hex2Rgb(store.led_color_l!)
    store.config!.led_color_r = Hex2Rgb(store.led_color_r!)
    store.config!.led_color_btm_l = Hex2Rgb(store.led_color_btm_l!)
    store.config!.led_color_btm_r = Hex2Rgb(store.led_color_btm_r!)
    store.config!.speed_press_high_color = Hex2Rgb(store.speed_press_high_color!)
    store.config!.speed_press_low_color = Hex2Rgb(store.speed_press_low_color!)
    store.config!.breath_interval = 20 - store.breath_speed!
    store.config!.rainbow_light_switching_interval = 30 - store.rainbow_light_switching_speed!
    await invoke('save_config', { config: store.config })
    status.value = "success"
    status_str.value = "同步配置成功"
  } catch (e) {
    store.config = undefined
    store.connected = false
    status.value = "error"
    status_str.value = "同步配置失败，错误原因：" + e
    console.error(e)
  }
  store.loading = false
}

</script>

<template>
  <div class="justify-self-start h-full flex items-center">
    <n-button class="ml-4 pointer-events-none" :loading="store.loading" :type="status">{{ status_str }}</n-button>
  </div>
  <div class="justify-self-end h-full flex items-center">
    <div v-if="store.config == undefined">
      <n-button class="mr-4" :disabled="store.loading" @click="connect" >连接设备</n-button>
    </div>
    <div v-else>
      <n-button class="mr-4" :disabled="store.loading"  @click="calibration_key" >校准设备</n-button>
      <n-button class="mr-4" :disabled="store.loading" @click="get_default_config" >默认值</n-button>
      <n-button class="mr-4" :disabled="store.loading" @click="sync_config" >同步配置</n-button>
    </div>
  </div>
</template>

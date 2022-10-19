<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { Type } from "naive-ui/es/button/src/interface"
import { useStore } from '@/store'
import { IConfig } from "@/interface";
import { Rgb2Hex, Hex2Rgb } from '@/utils';
// defineProps<{ msg: string }>()

const store = useStore()
const status = ref<Type | undefined>(undefined)
const status_str = ref("设备未连接")

async function connect() {
  store.loading = true
  status.value = "warning"
  status_str.value = "连接中"
  try {
    const res = await invoke("connect")
    store.connected = true
    await get_config()
    status.value = "success"
    status_str.value = "连接成功"
  } catch (e) {
    store.connected = false
    status.value = "error"
    status_str.value = "连接失败，错误原因：" + e
    console.log(e)
  }
  store.loading = false
}

async function get_config() {
  try {
    const res: IConfig = await invoke("get_config")
    console.log(res)
    store.led_color_l = Rgb2Hex(res.led_color_l)
    store.led_color_r = Rgb2Hex(res.led_color_r)
    store.led_color_btm_l = Rgb2Hex(res.led_color_btm_l)
    store.led_color_btm_r = Rgb2Hex(res.led_color_btm_r)
    store.speed_press_high_color = Rgb2Hex(res.speed_press_high_color)
    store.speed_press_low_color = Rgb2Hex(res.speed_press_low_color)
    store.config = res
  } catch (e) {
    store.connected = false
    status.value = "error"
    status_str.value = "获取配置失败，错误原因：" + e
    console.log(e)
  }
}

async function sync_config() {
  store.loading = true
  status.value = "warning"
  status_str.value = "正在上传配置"
  try {
    store.config!.led_color_l = Hex2Rgb(store.led_color_l!)
    store.config!.led_color_r = Hex2Rgb(store.led_color_r!)
    store.config!.led_color_btm_l = Hex2Rgb(store.led_color_btm_l!)
    store.config!.led_color_btm_r = Hex2Rgb(store.led_color_btm_r!)
    store.config!.speed_press_high_color = Hex2Rgb(store.speed_press_high_color!)
    store.config!.speed_press_low_color = Hex2Rgb(store.speed_press_low_color!)
    await invoke('save_config', { config: store.config })
    status.value = "success"
    status_str.value = "上传配置成功"
  } catch (e) {
    store.config = undefined
    store.connected = false
    status.value = "error"
    status_str.value = "上传配置失败，错误原因：" + e
    console.log(e)
  }
  store.loading = false
}

</script>

<template>
  <div class="justify-self-start h-full flex items-center">
    <n-button class="ml-4 pointer-events-none" :loading="store.loading" :type="status">{{ status_str }}</n-button>
  </div>
  <div class="justify-self-end h-full flex items-center">
    <n-button class="mr-4" :disabled="store.loading" @click="connect" v-if="store.config == undefined">连接设备</n-button>
    <n-button class="mr-4" :disabled="store.loading" @click="sync_config" v-else>上传配置</n-button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { Type } from "naive-ui/es/button/src/interface"
import { useStore } from '@/store'
import { IConfig } from "@/interface";
// defineProps<{ msg: string }>()

const store = useStore()
const status = ref<Type | undefined>(undefined)
const status_str = ref("设备未连接")

async function connect() {
  store.loading = true
  status.value = undefined
  status_str.value = "连接中"
  try {
    const res = await invoke("connect")
    store.connected = true
    status.value = "success"
    status_str.value = "连接成功"
    await get_config()
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
  try {
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

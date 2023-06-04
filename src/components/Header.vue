<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import { Type } from "naive-ui/es/button/src/interface"
import { useStore } from '@/store'
import { useI18n } from "vue-i18n";
import { setI18nLanguage, i18n } from '@/locales/index'
import { IConfig, IDevice } from "@/interface";
import { Rgb2Hex, Hex2Rgb } from '@/utils';
import { useDialog } from 'naive-ui'


const { t } = useI18n();
const dialog = useDialog()
const store = useStore()
const status = ref<Type | undefined>(undefined)
const status_str = ref(t("device_disconnected"))
const show_calibrate_msg = ref(false)

const state = ref({
  options: [
    {
      value: 'zh',
      label: '简体中文',
    },
    {
      value: 'en',
      label: 'English',
    },
  ],
  currentLang: i18n.global.locale.value
})

function handleChange(e: string) {
  setI18nLanguage(i18n, e)
  status_str.value = t("device_disconnected")
}

async function connect() {
  store.loading = true
  status.value = "warning"
  status_str.value = t('connecting')
  try {
    const res = await invoke("connect")
    let info = await check_device_info()
    console.table(info)
    store.device_info = info


    if (store.device_info!.version != store.firmware_version) {
      store.need_update_firmware = true // 需要更新固件
      store.loading = false
      status.value = "error"
      status_str.value = t('bad_firmware_version', { version: info!.version })
      return
    }

    // 不管怎么样总之是连上了
    store.connected = true

    status.value = "success"
    if (info === undefined) {
      status_str.value = t('connected')
    } else {
      status_str.value = t('connected_device', { version: info!.version })
    }
  } catch (e) {
    store.connected = false
    status.value = "error"
    status_str.value = t('connection_broke', { e: e })
    console.error(e)
  }
  if (store.debug_mode)
    await get_config_raw()
  else
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
    status_str.value = t('unknown_device_e', { e: e })
    console.error(e)
  }
}

async function calibration_key() {
  store.loading = true
  try {
    await invoke("calibration_key")
  } catch (e) {
    store.connected = false
    status.value = "error"
    status_str.value = t('connection_broke', { e: e })
    console.error(e)
    store.loading = false
    return
  }
  store.loading = false
  show_calibrate_msg.value = true
  try {
    await invoke("get_calibration_key_result", { "timeout": 5000 })
  } catch (e) {
    status.value = "error"
    status_str.value = t('cali_failed', { e: e })
    console.error(e)
    show_calibrate_msg.value = false
    return
  }
  status.value = "success"
  status_str.value = t('cali_finished')
  show_calibrate_msg.value = false
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
    status_str.value = t('reset_success')
  } catch (e) {
    status.value = "error"
    status_str.value = t('reset_failed', { e: e })
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
    store.is_hs = await invoke("is_hs")
    if (store.version_info !== undefined) {
      if (store.is_hs)
        store.version_info.latest_firmware_download_url = store.version_info.v1_hs_latest_firmware_download_url
      else
        store.version_info.latest_firmware_download_url = store.version_info.v1_latest_firmware_download_url
    }
  } catch (e) {
    const es = e as string
    status.value = "error"
    status_str.value = es
    console.error(es)
    if (es.includes("Semantic") || es.includes("Syntax") || es.includes("Unexpected")) {
      status_str.value = t('device_config_error')
      setTimeout(async () => {
        await get_default_config()
        await sync_config()
      }, 5000)
    }
  }
  store.loading = false
}

async function get_config_raw() {
  store.loading = true
  try {
    const res: string = await invoke("get_raw_config")
    store.raw_config = res
    store.is_hs = await invoke("is_hs")
    if (store.version_info !== undefined) {
      if (store.is_hs)
        store.version_info.latest_firmware_download_url = store.version_info.v1_hs_latest_firmware_download_url
      else
        store.version_info.latest_firmware_download_url = store.version_info.v1_latest_firmware_download_url
    }
  } catch (e) {
    const es = e as string
    status.value = "error"
    status_str.value = es
    console.error(es)
    if (es.includes("Semantic") || es.includes("Syntax") || es.includes("Unexpected")) {
      status_str.value = t('device_config_error')
      setTimeout(async () => {
        await get_default_config()
        await sync_config()
        store.config = undefined
      }, 5000)
    }
  }
  store.loading = false
}

async function sync_config() {
  store.loading = true
  status.value = "warning"
  status_str.value = t('syncing_config')
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
    status_str.value = t('sync_success')
  } catch (e) {
    store.config = undefined
    store.connected = false
    status.value = "error"
    status_str.value = t('sync_error', { e: e })
    console.error(e)
  }
  store.loading = false
}

async function sync_config_raw() {
  store.loading = true
  status.value = "warning"
  status_str.value = t('syncing_config')
  try {
    await invoke('save_raw_config', { config: store.raw_config })
    status.value = "success"
    status_str.value = t('sync_success')
  } catch (e) {
    store.raw_config = undefined
    store.connected = false
    status.value = "error"
    status_str.value = t('sync_error', { e: e })
    console.error(e)
  }
  store.loading = false
}


function debug() {
  dialog.warning({
    title: t('warning'),
    content: t('developer_warning'),
    positiveText: t('confirm'),
    negativeText: t('unconfirm'),
    maskClosable: false,
    onPositiveClick: () => {
      store.debug_mode = true
    },
  })
}

const regex = /(\d)\/d:(\d*),f:(\d*)\*/gm;

async function debug_mode() {
  try {
    await invoke("debug_mode")
    store.in_debug = true
    status.value = "warning"
    status_str.value = t('enter_debug_mode')
    const unlisten_debug = await listen('debug', (event) => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
      store.debug_str = event.payload as string;
      let m;

      while ((m = regex.exec(store.debug_str)) !== null) {
        // This is necessary to avoid infinite loops with zero-width matches
        if (m.index === regex.lastIndex) {
          regex.lastIndex++;
        }

        // The result can be accessed through the `m`-variable.
        store.adc_data[m[1] - 1].dyn = parseInt(m[2])
        store.adc_data[m[1] - 1].fixed = parseInt(m[3])
        // store.adc_data[m[1] - 1].min = parseInt(m[3])

        if (store.adc_data[m[1] - 1].dyn > store.adc_data[m[1] - 1].max)
          store.adc_data[m[1] - 1].max = store.adc_data[m[1] - 1].dyn

        if (store.adc_data[m[1] - 1].dyn < store.adc_data[m[1] - 1].min)
          store.adc_data[m[1] - 1].min = store.adc_data[m[1] - 1].dyn
      }

    })
    const unlisten_exit_debug = await listen('exit-debug', (event) => {
      unlisten_debug()
      unlisten_exit_debug()
      store.connected = false
      store.in_debug = false
      store.raw_config = undefined
      status.value = undefined
      status_str.value = t("device_disconnected")
    })
  } catch (e) {
    store.connected = false
    store.in_debug = false
    status.value = "error"
    status_str.value = t('connection_broke', { e: e })
    console.error(e)
  }
}

async function erase_firmware() {
  try {
    await invoke("erase_firmware")
    store.connected = false
    status.value = undefined
    status_str.value = t("device_disconnected")
  } catch (e) {
    store.connected = false
    status.value = "error"
    status_str.value = t('connection_broke', { e: e })
    console.error(e)
  }
}

</script>

<template>
  <n-modal v-model:show="show_calibrate_msg" transform-origin="center">
    <n-card style="width: fit-content;border-radius: 8px;align-items: center;" :bordered="false"
      :title="$t('cali_msg')" role="dialog" aria-modal="true">
    </n-card>
  </n-modal>
  <div class="justify-self-start h-full flex items-center">
    <n-select v-if="!store.connected" class="ml-4" @update:value="handleChange" v-model:value="state.currentLang" placeholder="Language" :options="state.options"></n-select>
      <n-button class="ml-4 pointer-events-none" :loading="store.loading" :type="status">{{ status_str }}</n-button>
  </div>
  <div class="justify-self-end h-full flex items-center">
    <div v-if="store.debug_mode">
      <div v-if="!store.connected">
        <n-button class="mr-4" :disabled="store.loading" @click="connect">{{ t("connect") }}</n-button>
      </div>
      <div v-else>
        <n-button class="mr-4" :disabled="store.loading || store.in_debug" @click="erase_firmware">{{ $t('erase_firmware') }}</n-button>
        <n-button class="mr-4" :disabled="store.loading || store.in_debug" @click="debug_mode">{{ $t('debug_mode') }}</n-button>
        <n-button class="mr-4" :disabled="store.loading || store.in_debug || !store.can_sync"
          @click="sync_config_raw">{{ $t('sync_config') }}</n-button>
      </div>
    </div>
    <div v-else>
      <div v-if="!store.connected">
        <n-button class="mr-4" :disabled="store.loading" @click="debug">{{ t("developer_mode") }}</n-button>
        <n-button class="mr-4" :disabled="store.loading" @click="connect">{{ t("connect") }}</n-button>
      </div>
      <div v-else>
        <n-button class="mr-4" :disabled="store.loading" v-if="store.is_hs" @click="calibration_key">{{ $t('cali_device') }}</n-button>
        <n-button class="mr-4" :disabled="store.loading" @click="get_default_config">{{ $t('default_config') }}</n-button>
        <n-button class="mr-4" :disabled="store.loading" @click="sync_config">{{ $t('sync_config') }}</n-button>
      </div>
    </div>
  </div>
</template>

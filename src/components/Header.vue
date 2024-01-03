<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import { Type } from "naive-ui/es/button/src/interface"
import { useStore } from '@/store'
import { useI18n } from "vue-i18n";
import { setI18nLanguage, i18n } from '@/locales/index'
import { IConfig, IDevice, IError, IKeyRTStatus, KeyCode, Toggle } from "@/interface";
import { Rgb2Hex, Hex2Rgb, getErrorMsg } from '@/utils';
import { useDialog } from 'naive-ui'


const { t } = useI18n();
const dialog = useDialog()
const store = useStore()
store.status_str = t("device_disconnected")
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
  store.status_str = t("device_disconnected")
}

async function connect() {
  store.loading = true
  store.status = "warning"
  store.status_str = t('connecting')
  try {
    const res = await invoke("connect")
    let info = await check_device_info()
    console.table(info)
    store.device_info = info

    let firmware_version = await invoke("get_firmware_version")
    if (store.device_info!.version != firmware_version) {
      store.need_update_firmware = true // 需要更新固件
      store.loading = false
      store.status = "error"
      store.status_str = t('bad_firmware_version', { version: info!.version })
      return
    }

    if (store.version_info) {
      store.latest_firmware_download_url = store.version_info.v2_standard_edition_firmware_download_url
    }


    // 不管怎么样总之是连上了
    store.connected = true

    store.status = "success"
    if (info === undefined) {
      store.status_str = t('connected')
    } else {
      store.status_str = t('connected_device', { version: info!.version })
    }
  } catch (e) {
    store.connected = false
    store.status = "error"
    store.status_str = t('connection_broke', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
  if (store.connected) {
    if (store.developer_mode)
      await get_config_raw()
    else
      await get_config()
  }
  store.loading = false
}

async function check_device_info(): Promise<IDevice | undefined> {
  try {
    const res: IDevice = await invoke("get_device_info")
    return res
  } catch (e) {
    store.connected = false
    store.status = "error"
    store.status_str = t('unknown_device_e', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
}

async function calibration_key() {
  store.loading = true
  try {
    await invoke("calibration_key")
  } catch (e) {
    store.connected = false
    store.status = "error"
    store.status_str = t('connection_broke', { e: getErrorMsg(t, e as IError) })
    console.error(e)
    store.loading = false
    return
  }
  store.loading = false
  show_calibrate_msg.value = true
  setTimeout(() => {
    show_calibrate_msg.value = false
  }, 3000)
}

function store_config(res: IConfig) {
  store.led_colors = []
  for (let i = 0; i < res.led_colors.length; i++) {
    store.led_colors.push(Rgb2Hex(res.led_colors[i]))
  }
  store.low_speed_color = Rgb2Hex(res.low_speed_color)
  store.high_speed_color = Rgb2Hex(res.high_speed_color)
  store.jitters_elimination_time = res.jitters_elimination_time / 8
  store.continuous_report = res.continuous_report == true ? Toggle.On : Toggle.Off
  store.kalman_filter = res.kalman_filter == true ? Toggle.On : Toggle.Off
  store.change_color_when_pressed = res.change_color_when_pressed == true ? Toggle.On : Toggle.Off
  store.random_color_mode = res.random_color_mode == true ? Toggle.On : Toggle.Off
  store.is_flow_delay = res.is_flow_delay == true ? Toggle.On : Toggle.Off
  store.max_brightness = Math.floor(res.max_brightness * 2)
  store.config = res
  for (let i = 0; i < store.config.keys.length; i++) {
    store.config.keys[i].key_data = store.config.keys[i].key_data.filter(k => k != KeyCode.None)
  }
}

async function get_default_config() {
  store.loading = true
  try {
    const res: IConfig = await invoke("get_default_config")
    console.dir(res)
    store_config(res)
    store.status = "success"
    store.status_str = t('reset_success')
  } catch (e) {
    store.status = "error"
    store.status_str = t('reset_failed', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
  store.loading = false
}

async function get_config() {
  store.loading = true
  try {
    const res: IConfig = await invoke("get_config")
    console.dir(res)
    store_config(res)
  } catch (e) {
    const es = e as IError
    store.status = "error"
    store.status_str = getErrorMsg(t, e as IError)
    console.error(es)
    if (es.type === "Meowpad" && es.data.toString().includes("config_")) {
      store.status_str = t('device_config_error')
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
  } catch (e) {
    const es = e as IError
    store.status = "error"
    store.status_str = getErrorMsg(t, e as IError)
    console.error(es)
    if (es.type === "Meowpad" && es.data.toString().includes("config_")) {
      store.status_str = t('device_config_error')
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
  store.status = "warning"
  store.status_str = t('syncing_config')
  try {
    store.config!.led_colors = []
    for (let i = 0; i < store.led_colors!.length; i++) {
      store.config!.led_colors.push(Hex2Rgb(store.led_colors![i]))
    }
    store.config!.low_speed_color = Hex2Rgb(store.low_speed_color!)
    store.config!.high_speed_color = Hex2Rgb(store.high_speed_color!)

    for (let i = 0; i < store.config!.keys.length; i++) {
      while (store.config!.keys[i].key_data.length < 6) {
        store.config!.keys[i].key_data.push(KeyCode.None)
      }

      while (store.config!.keys[i].key_data.length > 6) {
        store.config!.keys[i].key_data.pop()
      }
    }
    store.config!.jitters_elimination_time = Math.round(store.jitters_elimination_time * 8)
    store.config!.continuous_report = store.continuous_report == Toggle.On ? true : false
    store.config!.kalman_filter = store.kalman_filter == Toggle.On ? true : false
    store.config!.change_color_when_pressed = store.change_color_when_pressed == Toggle.On ? true : false
    store.config!.random_color_mode = store.random_color_mode == Toggle.On ? true : false
    store.config!.is_flow_delay = store.is_flow_delay == Toggle.On ? true : false
    
    store.config!.max_brightness = Math.round(store.max_brightness / 2)

    await invoke('save_config', { config: store.config })
    store.status = "success"
    store.status_str = t('sync_success')

    for (let i = 0; i < store.config!.keys.length; i++) {
      store.config!.keys[i].key_data = store.config!.keys[i].key_data.filter(k => k != KeyCode.None)
    }
  } catch (e) {
    store.config = undefined
    store.connected = false
    store.status = "error"
    store.status_str = t('sync_error', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
  store.loading = false
}

async function sync_config_raw() {
  store.loading = true
  store.status = "warning"
  store.status_str = t('syncing_config')
  try {
    await invoke('save_raw_config', { config: store.raw_config })
    store.status = "success"
    store.status_str = t('sync_success')
  } catch (e) {
    store.raw_config = undefined
    store.connected = false
    store.status = "error"
    store.status_str = t('sync_error', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
  store.loading = false
}

function exit_developer_mode() {
  store.developer_mode = false
  store.connected = false
  store.status = undefined
  store.status_str = t("device_disconnected")
}

function developer_mode() {
  dialog.warning({
    title: t('warning'),
    content: t('developer_warning'),
    positiveText: t('confirm'),
    negativeText: t('unconfirm'),
    maskClosable: false,
    onPositiveClick: () => {
      store.developer_mode = true
    },
  })
}

const regex = /(\d)\/d:(\d*),f:(\d*)\*/gm;



async function debug() {
  store.debug_mode = !store.debug_mode

}

async function erase_firmware() {
  try {
    await invoke("erase_firmware")
    store.connected = false
    store.status = undefined
    store.status_str = t("device_disconnected")
  } catch (e) {
    store.connected = false
    store.status = "error"
    store.status_str = t('connection_broke', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
}

</script>

<template>
  <n-modal v-model:show="show_calibrate_msg" transform-origin="center">
    <n-card style="width: fit-content;border-radius: 8px;align-items: center;" :bordered="false" :title="$t('cali_msg')"
      role="dialog" aria-modal="true">
    </n-card>
  </n-modal>
  <div class="left">
    <n-select v-if="!store.connected" class="ml" @update:value="handleChange" v-model:value="state.currentLang"
      placeholder="Language" :options="state.options"></n-select>
    <n-button class="ml" id="msgbox" :loading="store.loading" :type="store.status">{{ store.status_str }}</n-button>
  </div>
  <div class="right">
    <div v-if="store.developer_mode">
      <div v-if="!store.connected">
        <n-button class="mr" :disabled="store.loading" @click="connect">{{ t("connect") }}</n-button>
        <n-button class="mr" :disabled="store.loading" @click="exit_developer_mode">{{ $t('exit') }}</n-button>
      </div>
      <div v-else>
        <n-button class="mr" :disabled="store.loading" @click="debug">
        {{ store.debug_mode ? $t('exit') : t('debug_mode') }}
        </n-button>
        <n-button class="mr" v-if="!store.debug_mode" :disabled="store.loading" @click="erase_firmware">{{ $t('erase_firmware') }}</n-button>
        <n-button class="mr" v-if="!store.debug_mode" :disabled="store.loading || !store.can_sync" @click="sync_config_raw">{{
          $t('sync_config') }}</n-button>
          <n-button class="mr" v-if="!store.debug_mode" :disabled="store.loading" @click="exit_developer_mode">{{ $t('exit') }}</n-button>
      </div>
    </div>
    <div v-else>
      <div v-if="!store.connected">
        <n-button class="mr" :disabled="store.loading" @click="developer_mode">{{ t("developer_mode") }}</n-button>
        <n-button class="mr" :disabled="store.loading" @click="connect">{{ t("connect") }}</n-button>
      </div>
      <div v-else>
        <n-button class="mr" :disabled="store.loading" @click="calibration_key">{{ $t('cali_device')
        }}</n-button>
        <n-button class="mr" :disabled="store.loading" @click="get_default_config">{{ $t('default_config') }}</n-button>
        <n-button class="mr" :disabled="store.loading" @click="sync_config">{{ $t('sync_config') }}</n-button>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.left {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  overflow: hidden;
}

.right {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  flex-direction: row;
}

.mr {
  margin-right: 1rem;
}
.ml {
  margin-left: 1rem;
}

#msgbox {
  pointer-events: none;
}
</style>


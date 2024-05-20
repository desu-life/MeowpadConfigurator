<script setup lang="ts">
import { ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { Type } from "naive-ui/es/button/src/interface"
import { useDeviceStore, useStore } from '@/store'
import { useI18n } from "vue-i18n";
import { setI18nLanguage, i18n } from '@/locales/index'
import { Rgb2Hex, Hex2Rgb, getErrorMsg } from '@/utils';
import { useDialog } from 'naive-ui'
import * as api from '@/apis/api'
import * as api4k from '@/apis/meowpad4k/api'
import { IError } from '@/apis';
import { Toggle } from '@/interface';
import { IKeyboard, ILighting } from '@/apis/meowpad4k/config';
import { KeyCode } from '@/keycode';


const { t } = useI18n();
const dialog = useDialog()
const store = useStore()
const device = useDeviceStore()
const message = useMessage()
store.status_str = t("device_disconnected")
const show_calibrate_msg = ref(false)
const sync_btn_type = ref<Type | undefined>("default")

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
    await api4k.connect()
    let info = await api.get_device_info()
    console.table(info)
    device.device_info = info

    let firmware_version = await api.get_firmware_4k_version()
    if (device.device_info!.version != firmware_version) {
      store.need_update_firmware = true // 需要更新固件
      store.loading = false
      store.status = "error"
      store.status_str = t('bad_firmware_version', { version: info!.version })
      return
    }

    if (store.version_info) {
      store.latest_firmware_download_url = store.version_info.v2_standard_edition_firmware_download_url
    }

    let status = await api.get_device_status()
    device.device_status = status

    if (device.device_status.key == false) {
      await get_default_key_config()
      await sync_key_config()
      await api4k.save_key_config()
    }

    if (device.device_status.key == false) {
      await get_default_light_config()
      await sync_light_config()
      await api4k.save_light_config()
    }

    // 不管怎么样总之是连上了
    device.connected = true
    store.status = "success"

    if (device.device_status.hall == false) {
      dialog.warning({
        title: t('warning'),
        content: t('device_cali_warn'),
        positiveText: t('yes'),
        negativeText: t('no'),
        maskClosable: false,
        onPositiveClick: () => {
          calibration_key()
        },
      })
    }

    if (info === undefined) {
      store.status_str = t('connected')
    } else {
      store.status_str = t('connected_device', { version: info!.version })
    }
  } catch (e) {
    device.connected = false
    store.status = "error"
    store.status_str = t('connection_broke', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
  if (device.connected) {
    if (store.developer_mode)
      await get_config_raw()
    else
      await get_config()
  }
  store.loading = false
}

async function calibration_key() {
  store.loading = true
  try {
    await api4k.calibration_key()
  } catch (e) {
    device.connected = false
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


function store_key_config(res: IKeyboard) {
  device.jitters_elimination_time = res.jitters_elimination_time / 8
  device.continuous_report = res.continuous_report == true ? Toggle.On : Toggle.Off
  device.kalman_filter = res.kalman_filter == true ? Toggle.On : Toggle.Off
  device.enable_hs = res.enable_hs == true ? Toggle.On : Toggle.Off
  device.key_config = res
  for (let i = 0; i < device.key_config.keys.length; i++) {
    device.key_config.keys[i].key_data = device.key_config.keys[i].key_data.filter(k => k != KeyCode.None)
  }
}

function store_light_config(res: ILighting) {
  device.led_colors = []
  for (let i = 0; i < res.led_colors.length; i++) {
    device.led_colors.push(Rgb2Hex(res.led_colors[i]))
  }
  device.low_speed_color = Rgb2Hex(res.low_speed_color)
  device.high_speed_color = Rgb2Hex(res.high_speed_color)
  device.change_color_when_pressed = res.change_color_when_pressed == true ? Toggle.On : Toggle.Off
  device.random_color_mode = res.random_color_mode == true ? Toggle.On : Toggle.Off
  device.is_flow_delay = res.is_flow_delay == true ? Toggle.On : Toggle.Off
  device.max_brightness = Math.floor(res.max_brightness * 2)
  device.light_config = res
}

async function get_default_key_config() {
  const res = await api4k.get_default_key_config()
  console.dir(res)
  store_key_config(res)
}

async function get_default_light_config() {
  const res = await api4k.get_default_light_config()
  console.dir(res)
  store_light_config(res)
}

async function get_default_config() {
  store.loading = true
  setTimeout(async () => {
    await get_default_key_config()
    await get_default_light_config()

    sync_btn_type.value = "primary"
    store.status = "success"
    store.status_str = t('reset_success')
    store.loading = false
  }, 250);
}

async function get_key_config() {
  const res = await api4k.get_key_config()
  console.dir(res)
  store_key_config(res)
}

async function get_light_config() {
  const res = await api4k.get_light_config()
  console.dir(res)
  store_light_config(res)
}

async function get_config() {
  store.loading = true
  try {
    await get_key_config()
    await get_light_config()
  } catch (e) {
    const es = e as IError
    store.status = "error"
    store.status_str = getErrorMsg(t, e as IError)
    console.error(es)
    // if (es.type === "meowpad" && es.data.toString().includes("config_")) {
    //   store.status_str = t('device_config_error')
    //   setTimeout(async () => {
    //     await get_default_config()
    //     await sync_config()
    //   }, 5000)
    // }
  }
  store.loading = false
}

async function get_config_raw() {
  store.loading = true
  try {
    const res = await api4k.get_raw_config()
    device.raw_config = res
  } catch (e) {
    const es = e as IError
    store.status = "error"
    store.status_str = getErrorMsg(t, e as IError)
    console.error(es)
  }
  store.loading = false
}

const need_check = ref(false)

async function sync_key_config() {
  for (let i = 0; i < device.key_config!.keys.length; i++) {
    while (device.key_config!.keys[i].key_data.length < 6) {
      device.key_config!.keys[i].key_data.push(KeyCode.None)
    }

    while (device.key_config!.keys[i].key_data.length > 6) {
      device.key_config!.keys[i].key_data.pop()
    }
  }
  device.key_config!.jitters_elimination_time = Math.round(device.jitters_elimination_time * 8)
  device.key_config!.continuous_report = device.continuous_report == Toggle.On ? true : false
  device.key_config!.kalman_filter = device.kalman_filter == Toggle.On ? true : false
  device.key_config!.enable_hs = device.enable_hs == Toggle.On ? true : false

  await api4k.set_key_config(device.key_config!);

  for (let i = 0; i < device.key_config!.keys.length; i++) {
    device.key_config!.keys[i].key_data = device.key_config!.keys[i].key_data.filter(k => k != KeyCode.None)

    if (device.key_config!.keys[i].dead_zone < 5) {
      need_check.value = true
    }
    if (device.key_config!.keys[i].press_percentage < 3) {
      need_check.value = true
    }
    if (device.key_config!.keys[i].release_percentage < 3) {
      need_check.value = true
    }
  }
}


async function sync_light_config() {
  device.light_config!.led_colors = []
  for (let i = 0; i < device.led_colors!.length; i++) {
    device.light_config!.led_colors.push(Hex2Rgb(device.led_colors![i]))
  }
  device.light_config!.low_speed_color = Hex2Rgb(device.low_speed_color!)
  device.light_config!.high_speed_color = Hex2Rgb(device.high_speed_color!)

  device.light_config!.change_color_when_pressed = device.change_color_when_pressed == Toggle.On ? true : false
  device.light_config!.random_color_mode = device.random_color_mode == Toggle.On ? true : false
  device.light_config!.is_flow_delay = device.is_flow_delay == Toggle.On ? true : false

  device.light_config!.max_brightness = Math.round(device.max_brightness / 2)

  await api4k.set_light_config(device.light_config!);
}

async function save_config() {
  store.loading = true
  await api4k.save_key_config()
  await api4k.save_light_config()
  need_check.value = false
  store.status = "success"
  store.status_str = t('sync_success')
  store.loading = false
}

async function sync_config() {
  store.loading = true
  store.status = "warning"
  store.status_str = t('syncing_config')
  try {
    await sync_key_config()
    await sync_light_config()
    if (need_check.value) {
      store.status = "warning"
      store.status_str = t('applied_config')
      message.warning(t('check_config_msg'))
    } else {
      await save_config()
    }

  } catch (e) {
    device.light_config = undefined
    device.connected = false
    store.status = "error"
    store.status_str = t('sync_error', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
  sync_btn_type.value = "default"
  store.loading = false
}

async function sync_config_raw() {
  store.loading = true
  store.status = "warning"
  store.status_str = t('syncing_config')
  try {
    await api4k.save_raw_config(device.raw_config!)
    store.status = "success"
    store.status_str = t('sync_success')
  } catch (e) {
    device.raw_config = undefined
    device.connected = false
    store.status = "error"
    store.status_str = t('sync_error', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
  store.loading = false
}

function exit_developer_mode() {
  store.developer_mode = false
  device.connected = false
  store.status = undefined
  store.status_str = t("device_disconnected")
}

function exit_iap_mode() {
  store.iap_connected = false
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

async function enter_iap() {
  try {
    await api.connect_iap()
    // 不管怎么样总之是连上了
    store.iap_connected = true
    store.status = "warning"
    store.status_str = t('iap_connected')
  } catch (e) {
    store.iap_connected = false
    store.status = "error"
    store.status_str = t('connection_broke', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
}


async function device_update() {
  store.loading = true
  store.status = "warning"
  store.status_str = t('connecting')
  try {
    await api4k.connect()
    dialog.warning({
      title: t('warning'),
      content: t('device_update_warn'),
      positiveText: t('yes'),
      negativeText: t('no'),
      maskClosable: false,
      onPositiveClick: async () => {
        await api4k.erase_firmware()
        setTimeout(async () => {
          await enter_iap()
          store.loading = false
        }, 1000);
      },
      onNegativeClick: () => {
        store.loading = false
        store.status = undefined
        store.status_str = t("device_disconnected")
      },
    })
  } catch (e) {
    await enter_iap()
    store.loading = false
  }
}

const regex = /(\d)\/d:(\d*),f:(\d*)\*/gm;



async function debug() {
  store.debug_mode = !store.debug_mode

}

async function erase_firmware() {
  try {
    await api4k.erase_firmware()
    device.connected = false
    store.status = undefined
    store.status_str = t("device_disconnected")
  } catch (e) {
    device.connected = false
    store.status = "error"
    store.status_str = t('connection_broke', { e: getErrorMsg(t, e as IError) })
    console.error(e)
  }
}
</script>

<template>
  <n-modal v-model:show="show_calibrate_msg" transform-origin="center">
    <n-card style="width: fit-content;border-radius: 5px;align-items: center;" :bordered="false" :title="$t('cali_msg')"
      role="dialog" aria-modal="true">
    </n-card>
  </n-modal>
  <div class="left">
    <n-select v-if="!device.connected" class="ml" @update:value="handleChange" v-model:value="state.currentLang"
      placeholder="Language" :options="state.options"></n-select>
    <n-button class="ml" id="msgbox" :loading="store.loading" :type="store.status" strong secondary>{{ store.status_str
      }}</n-button>
  </div>
  <div class="right">
    <div v-if="store.developer_mode">
      <div v-if="!device.connected">
        <div v-if="store.iap_connected">
          <n-button class="mr" v-if="!store.debug_mode" :disabled="store.loading" @click="exit_iap_mode">{{
    $t('exit') }}</n-button>
        </div>
        <div v-else>

          <n-button class="mr" :disabled="store.loading" @click="device_update">{{ $t('device_update') }}</n-button>
          <n-button class="mr" :disabled="store.loading" @click="connect">{{ t("connect") }}</n-button>
          <n-button class="mr" :disabled="store.loading" @click="exit_developer_mode">{{ $t('exit') }}</n-button>
        </div>
      </div>
      <div v-else>


        <n-button class="mr" :disabled="store.loading" @click="debug">
          {{ store.debug_mode ? $t('exit') : t('debug_mode') }}
        </n-button>
        <n-button class="mr" v-if="!store.debug_mode" :disabled="store.loading" @click="erase_firmware">{{
    $t('erase_firmware') }}</n-button>
        <n-button class="mr" v-if="!store.debug_mode" :disabled="store.loading || !store.can_sync"
          @click="sync_config_raw">{{
    $t('sync_config') }}</n-button>
        <n-button class="mr" v-if="!store.debug_mode" :disabled="store.loading" @click="exit_developer_mode">{{
    $t('exit') }}</n-button>

      </div>
    </div>
    <div v-else>
      <div v-if="!device.connected">
        <n-button class="mr" :disabled="store.loading" @click="developer_mode">{{ t("developer_mode") }}</n-button>
        <n-button class="mr" :disabled="store.loading" @click="connect">{{ t("connect") }}</n-button>
      </div>
      <div v-else>
        <n-button class="mr" :disabled="store.loading" @click="calibration_key">{{ $t('cali_device')
          }}</n-button>
        <n-button class="mr" :disabled="store.loading" @click="get_default_config">{{ $t('default_config') }}</n-button>
        <n-button class="mr" :disabled="store.loading" v-if="!need_check" @click="sync_config" :type="sync_btn_type">{{
    $t('sync_config') }}</n-button>
        <n-button class="mr" :disabled="store.loading" v-if="need_check" @click="save_config" type="warning">{{
          $t('save_config') }}</n-button>
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
@/apis/api@/apis/interface
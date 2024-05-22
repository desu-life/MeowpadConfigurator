<script setup lang="ts">
import { ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { Type } from "naive-ui/es/button/src/interface"
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { useI18n } from "vue-i18n";
import { setI18nLanguage, i18n } from '@/locales/index'
import { Rgb2Hex, Hex2Rgb, getErrorMsg } from '@/utils';
import { useDialog } from 'naive-ui'
import * as api from '@/apis/api'
import * as api4k from '@/apis/meowpad4k/api'
import * as api3k from '@/apis/meowpad3k/api'
import { IError } from '@/apis';
import { Toggle } from '@/interface';
import { KeyCode } from '@/keycode';
import { IKeyboard as IKB3K } from "@/apis/meowpad3k/config";
import { storeToRefs } from 'pinia';


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
    if (!await device.try_connect()) {
      store.status = "error"
      store.status_str = t('connection_broke', { e: t('device_not_found') })
      store.loading = false
      return
    }
    console.table(device.device_info)

    let firmware_version = "";
    if (device.is_4k()) firmware_version = await api4k.get_firmware_version()
    else if (device.is_3k()) firmware_version = await api3k.get_firmware_version()

    if (firmware_version === "") {
      store.loading = false
      store.status = "error"
      store.status_str = t('unknown_device')
      return
    }

    if (device.device_info!.version != firmware_version) {
      store.need_update_firmware = true // 需要更新固件
      store.loading = false
      store.status = "error"
      store.status_str = t('bad_firmware_version', { version: device.device_info!.version })
      return
    }

    if (store.version_info) {
      store.latest_firmware_download_url = store.version_info.v2_standard_edition_firmware_download_url
    }

    await device.get_status()

    if (device.device_status!.key == false || device.device_status!.light == false) {
      await device.get_default_config()
      await device.sync_config()
      await device.save_config()
    }

    if (device.device_status!.hall == false) {
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

    if (device.device_info === undefined) {
      store.status_str = t('connected')
    } else {
      store.status_str = t('connected_device', { version: device.device_info!.version })
    }
  } catch (e) {
    store.status = "error"
    store.status_str = t('connection_broke', { e: getErrorMsg(t, e as IError) })
    store.loading = false
    console.error(e)
    return;
  }
  try {
    if (store.developer_mode)
      await device.get_config_raw()
    else
      await device.get_config()
  } catch (e) {
    const es = e as IError
    store.status = "error"
    store.status_str = getErrorMsg(t, e as IError)
    store.loading = false
    console.error(es)
    return;
  }
  // 不管怎么样总之是连上了
  store.status = "success"
  device.connected = true
  store.loading = false
}

async function calibration_key() {
  store.loading = true
  try {
    await device.calibration_key()
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



async function get_default_config() {
  store.loading = true
  setTimeout(async () => {
    await device.get_default_config()

    sync_btn_type.value = "primary"
    store.status = "success"
    store.status_str = t('reset_success')
    store.loading = false
  }, 250);
}


const need_check = ref(false)

function key_config_post_process() {
  const { key_config } = storeToRefs(device)
  const cfg = key_config;
  
  for (let i = 0; i < cfg.value!.keys.length; i++) {
    cfg.value!.keys[i].key_data = cfg.value!.keys[i].key_data.filter(k => k != KeyCode.None)

    if (cfg.value!.keys[i].dead_zone < 5) {
      need_check.value = true
    }
    if (cfg.value!.keys[i].press_percentage < 3) {
      need_check.value = true
    }
    if (cfg.value!.keys[i].release_percentage < 3) {
      need_check.value = true
    }
  }

  if (device.is_3k()) {
    const cfg_3k = cfg as Ref<IKB3K>;
    cfg_3k.value!.side_btn = cfg_3k.value!.side_btn.filter(k => k != KeyCode.None)
  }
}

async function save_config() {
  store.loading = true
  await device.save_config()
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
    await device.sync_config()
    key_config_post_process()

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
    await device.save_config_raw()
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
    if (!await device.try_connect()) {
      store.status = "error"
      store.status_str = t('connection_broke', { e: t('device_not_found') })
      store.loading = false
      return
    }
    if (!device.is_4k()) {
      store.status = "error"
      store.status_str = t('device_not_support')
      store.loading = false
      return
    }
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

async function clear_config() {
  try {
    await api3k.clear_config()
    await api3k.reset_device()
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
    <template v-if="store.developer_mode">
      <template v-if="!device.connected">
        <template v-if="store.iap_connected">
          <n-button class="mr" v-if="!store.debug_mode" :disabled="store.loading" @click="exit_iap_mode">{{
    $t('exit') }}</n-button>
        </template>
        <template v-else>
          <n-button class="mr" :disabled="store.loading" @click="device_update">{{ $t('device_update') }}</n-button>
          <n-button class="mr" :disabled="store.loading" @click="connect">{{ t("connect") }}</n-button>
          <n-button class="mr" :disabled="store.loading" @click="exit_developer_mode">{{ $t('exit') }}</n-button>
        </template>
      </template>
      <template v-else>
        <n-button class="mr" :disabled="store.loading" @click="debug">
          {{ store.debug_mode ? $t('exit') : t('debug_mode') }}
        </n-button>
        <template v-if="!store.debug_mode">
          <n-button  v-if="device.is_4k()" class="mr" :disabled="store.loading"  @click="erase_firmware">{{ $t('erase_firmware') }}</n-button>
          <n-button  v-if="device.is_3k()" class="mr" :disabled="store.loading"  @click="clear_config">{{ $t('clear_config') }}</n-button>
          <n-button class="mr" :disabled="store.loading || !store.can_sync" @click="sync_config_raw">{{$t('sync_config') }}</n-button>
          <n-button class="mr" :disabled="store.loading" @click="exit_developer_mode">{{ $t('exit') }}</n-button>
        </template>
      </template>
    </template>
    <template v-else>
      <template v-if="!device.connected">
        <n-button class="mr" :disabled="store.loading" @click="developer_mode">{{ t("developer_mode") }}</n-button>
        <n-button class="mr" :disabled="store.loading" @click="connect">{{ t("connect") }}</n-button>
      </template>
      <template v-else>
        <n-button class="mr" :disabled="store.loading" @click="calibration_key">{{ $t('cali_device') }}</n-button>
        <n-button class="mr" :disabled="store.loading" @click="get_default_config">{{ $t('default_config') }}</n-button>
        <n-button class="mr" :disabled="store.loading" v-if="!need_check" @click="sync_config" :type="sync_btn_type">{{ $t('sync_config') }}</n-button>
        <n-button class="mr" :disabled="store.loading" v-if="need_check" @click="save_config" type="warning">{{ $t('save_config') }}</n-button>
      </template>
    </template>
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
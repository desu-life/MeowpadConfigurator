<script setup lang="ts">
import { ref } from 'vue'
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
import { emit, listen } from '@tauri-apps/api/event'
import emitter from "@/mitt";



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

emitter.on('header-msg-update', (event: { status: Type; str: string }) => {
  store.loading = false
  store.status = event.status
  store.status_str = event.str
})

emitter.on('header-loading', (event: { str: string }) => {
  store.loading = true
  store.status = "warning"
  store.status_str = event.str
})

emitter.on('loading', () => {
  store.loading = true
})

emitter.on('loaded', () => {
  store.loading = false
})

emitter.on('sync-btn-highlight', (event: {status: boolean}) => {
  if (event.status) {
    sync_btn_type.value = "primary"
  } else {
    sync_btn_type.value = "default"
  }
})





async function calibration_key() {
  emitter.emit('calibration-key')
  show_calibrate_msg.value = true
  setTimeout(() => {
    show_calibrate_msg.value = false
  }, 3000)
}



async function get_default_config() {
  setTimeout(async () => {
    emitter.emit('get-default-config')
  }, 250);
}


async function save_config() {
  emitter.emit('get-default-config')
  store.need_check = false
}

async function sync_config() {
  emitter.emit('sync-config')
}

async function sync_config_raw() {
  emitter.emit('header-loading', { str: t('syncing_config') })
  try {
    await device.save_config_raw()
    emitter.emit('header-msg-update', { status: "success", str: t('sync_success') })
  } catch (e) {
    emitter.emit('connection-broke', {e: e as IError})
    emitter.emit('header-msg-update', { status: "error", str: t('sync_error', { e: getErrorMsg(t, e as IError) }) })
  }
}


function exit_developer_mode() {
  store.developer_mode = false
  emitter.emit('connection-broke', { e: null })
  emitter.emit('header-msg-update', { status: "default", str: t('device_disconnected') })
}

function exit_iap_mode() {
  store.iap_connected = false
  emitter.emit('header-msg-update', { status: "default", str: t('device_disconnected') })
}

async function device_update() {
  emitter.emit('header-loading', { str: t('connecting') })

  try {
    await api.connect_iap()
    // 不管怎么样总之是连上了
    store.iap_connected = true
    emitter.emit('header-msg-update', { status: "warning", str: t('iap_connected') })
    return
  } catch (e) {
    store.iap_connected = false
  }

  try {
    if (!await device.try_connect()) {
      emitter.emit('header-msg-update', { status: "error", str: t('connection_broke', { e: t('device_not_found') }) })
      return
    }
    if (!device.is_4k()) {
      emitter.emit('header-msg-update', { status: "error", str: t('device_not_support') })
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
          try {
            await api.connect_iap()
            // 不管怎么样总之是连上了
            store.iap_connected = true
            emitter.emit('header-msg-update', { status: "warning", str: t('iap_connected') })
          } catch (e) {
            emitter.emit('connection-broke', {e: e as IError})
          }
        }, 1000);
      },
      onNegativeClick: () => {
        emitter.emit('header-msg-update', { status: "default", str: t('device_disconnected') })
      },
    })
  } catch (e) {
    emitter.emit('connection-broke', {e: e as IError})
  }
}

const regex = /(\d)\/d:(\d*),f:(\d*)\*/gm;



async function debug() {
  store.debug_mode = !store.debug_mode

}

async function erase_firmware() {
  try {
    await api4k.erase_firmware()
    emitter.emit('connection-broke', { e: null })
    emitter.emit('header-msg-update', { status: "default", str: t('device_disconnected') })
  } catch (e) {
    emitter.emit('connection-broke', {e: e as IError})
  }
}

async function clear_config() {
  try {
    await api3k.clear_config()
    await api3k.reset_device()
    emitter.emit('connection-broke', { e: null })
    emitter.emit('header-msg-update', { status: "default", str: t('device_disconnected') })
  } catch (e) {
    emitter.emit('connection-broke', {e: e as IError})
  }
}


</script>

<template>
  <n-modal v-model:show="show_calibrate_msg" transform-origin="center">
    <n-card style="width: fit-content;border-radius: 5px;align-items: center;" :bordered="false" :title="$t('cali_msg')"
      role="dialog" aria-modal="true">
    </n-card>
  </n-modal>
  <div class="header">
    <div class="left">
      <n-button class="ml" id="msgbox" :loading="store.loading" :type="store.status" strong secondary>{{ store.status_str
        }}</n-button>
    </div>
    <div class="right">
      <template v-if="store.developer_mode">
        <template v-if="!device.connected">
          <template v-if="store.iap_connected">
            <n-button class="mr" v-if="!store.debug_mode" :disabled="store.loading" @click="exit_iap_mode">{{ $t('exit') }}</n-button>
          </template>
          <template v-else>
            <n-button class="mr" :disabled="store.loading" @click="device_update">{{ $t('device_update') }}</n-button>
            <n-button class="mr" :disabled="store.loading" @click="exit_developer_mode">{{ $t('exit') }}</n-button>
          </template>
        </template>
        <template v-else>
          <n-button v-if="device.is_4k() || device.is_3k()" class="mr" :disabled="store.loading" @click="debug">
            {{ store.debug_mode ? $t('exit') : t('debug_mode') }}
          </n-button>
          <template v-if="!store.debug_mode">
            <n-button  v-if="device.is_4k() || device.is_pure()" class="mr" :disabled="store.loading"  @click="erase_firmware">{{ $t('erase_firmware') }}</n-button>
            <n-button  v-if="device.is_3k()" class="mr" :disabled="store.loading"  @click="clear_config">{{ $t('clear_config') }}</n-button>
            <n-button class="mr" :disabled="store.loading || !store.can_sync" @click="sync_config_raw">{{$t('sync_config') }}</n-button>
            <n-button class="mr" :disabled="store.loading" @click="exit_developer_mode">{{ $t('exit') }}</n-button>
          </template>
        </template>
      </template>
      <template v-else>
        <template v-if="!device.connected">
          <!-- <n-button class="mr" :disabled="store.loading" @click="developer_mode">{{ t("developer_mode") }}</n-button> -->
          <!-- <n-button class="mr" :disabled="store.loading" @click="connect">{{ t("connect") }}</n-button> -->
          <n-select v-if="!device.connected" class="mr" @update:value="handleChange" v-model:value="state.currentLang"
          placeholder="Language" :options="state.options"></n-select>
        </template>
        <template v-else>
          <n-button v-if="device.is_4k() || device.is_3k()" class="mr" :disabled="store.loading" @click="calibration_key">{{ $t('cali_device') }}</n-button>
          <n-button class="mr" :disabled="store.loading" @click="get_default_config">{{ $t('default_config') }}</n-button>
          <n-button class="mr" :disabled="store.loading" v-if="!store.need_check" @click="sync_config" :type="sync_btn_type">{{ $t('sync_config') }}</n-button>
          <n-button class="mr" :disabled="store.loading" v-if="store.need_check" @click="save_config" type="warning">{{ $t('save_config') }}</n-button>
        </template>
      </template>
    </div>
  </div>
</template>

<style scoped lang="scss">
.header {
  position: relative;
  z-index: 1000;
  top: 15px;
  height: 100%;
  width: 95%;
  background-color: var(--n-color);
  border-radius: 10px;
  display: flex;
  align-items: center;
  flex-direction: row;
  justify-content: space-between;
}

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
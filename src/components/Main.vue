<script setup lang="ts">
import FirmwareUpdate from '@/components/FirmwareUpdate.vue'
import Settings4K from '@/components/meowpad4k/Settings.vue'
import Settings3K from '@/components/meowpad3k/Settings.vue'
import DeveloperSettings from '@/components/DeveloperSetting/DeveloperSettings.vue'
import { useI18n } from "vue-i18n";
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import emitter from "@/mitt";
import * as api from '@/apis/api'
import * as api4k from '@/apis/meowpad4k/api'
import * as api3k from '@/apis/meowpad3k/api'
import { useDialog } from 'naive-ui'
import { IError } from '@/apis';
import { getErrorMsg } from '@/utils';

const { t } = useI18n();
const dialog = useDialog()
const store = useStore()
const device = useDeviceStore()

emitter.on('connection-broke', async (event: {e: IError}) => {
  emitter.emit('header-msg-update', { status: "error", str: t('connection_broke', { e: getErrorMsg(t, event.e) }) })
  console.error(event.e)
  device.connected = false
  device.device_info = undefined
  device.raw_config = undefined
  device.key_config = undefined
  device.light_config = undefined
})


emitter.on('connect', async () => {
  emitter.emit('header-loading', { str: t('connecting') })
  try {
    if (!await device.try_connect()) {
      emitter.emit('header-msg-update', { status: "error", str: t('connection_broke', { e: t('device_not_found') }) })
      return
    }
    console.table(device.device_info)

    let firmware_version = "";
    if (device.is_4k()) firmware_version = await api4k.get_firmware_version()
    else if (device.is_3k()) firmware_version = await api3k.get_firmware_version()

    if (firmware_version === "") {
      emitter.emit('header-msg-update', { status: "error", str: t('unknown_device') })
      return
    }

    if (device.device_info!.version != firmware_version) {
      store.need_update_firmware = true // 需要更新固件
      emitter.emit('header-msg-update', { status: "error", str: t('bad_firmware_version', { version: device.device_info!.version }) })
      return
    }

    if (store.version_info) {
      store.latest_firmware_download_url = store.version_info.v2_standard_edition_firmware_download_url
    }

    await device.get_status()

    if (device.device_status!.key === false) {
      if (device.is_4k()) {
        await api4k.set_key_config(await api4k.get_default_key_config())
        await api4k.save_key_config()
      }
      if (device.is_3k()) {
        await api3k.set_key_config(await api3k.get_default_key_config())
        await api3k.save_key_config()
      }
    }

    if (device.device_status!.light != undefined && device.device_status!.light != null) {
      if (device.device_status!.light === false) {
        if (device.is_4k()) {
          await api4k.set_light_config(await api4k.get_default_light_config())
          await api4k.save_light_config()
        }
        if (device.is_3k()) {
          await api3k.set_light_config(await api3k.get_default_light_config())
          await api3k.save_light_config()
        }
      }
    }
    
    if (store.developer_mode) {
      if (device.is_4k()) {
        device.raw_config = await api4k.get_raw_config()
      }
      if (device.is_3k()) {
        device.raw_config = await api3k.get_raw_config()
      }
    } else {
      if (device.is_4k()) {
        device.key_config = await api4k.get_key_config()
        device.extract_key_config_4k()
        device.light_config = await api4k.get_light_config()
        device.extract_light_config_4k()
      }
      if (device.is_3k()) {
        device.key_config = await api3k.get_key_config()
        device.extract_key_config_3k()
        device.light_config = await api3k.get_light_config()
        device.extract_light_config_3k()
      }
    }

    if (device.device_info === undefined) {
      emitter.emit('header-msg-update', { status: "success", str: t('connected') })
    } else {
      emitter.emit('header-msg-update', { status: "success", str: t('connected_device', { version: device.device_info!.version }) })
    }

    device.connected = true

    if (device.device_status!.hall == false) {
      dialog.warning({
        title: t('warning'),
        content: t('device_cali_warn'),
        positiveText: t('yes'),
        negativeText: t('no'),
        maskClosable: false,
        onPositiveClick: () => {
          emitter.emit('calibration-key')
        },
      })
    }
    
  } catch (e) {
    emitter.emit('connection-broke', {e: e as IError})
    console.error(e)
    return;
  }
})

</script>

<template>
  <n-spin :show="store.loading" id="spin-cover">
    <div id="main">
      <div v-if="store.developer_mode" class="debug">
        <DeveloperSettings></DeveloperSettings>
      </div>
      
      <div v-else-if="device.connected">
        <div v-if="device.is_4k()">
          <Settings4K></Settings4K>
        </div>
        <div v-else-if="device.is_3k()">
          <Settings3K></Settings3K>
        </div>
      </div>
      
      <div v-else-if="store.need_update_firmware">
        <FirmwareUpdate></FirmwareUpdate>
      </div>
      
      <n-empty :description="t('no_device')" size="huge" v-else>
      </n-empty>
    </div>
  </n-spin>

</template>

<style lang="scss" scoped>
#main {
  width: 100%;
  height: calc(100vh - var(--header-height));
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

#spin-cover {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 100;
}
</style>
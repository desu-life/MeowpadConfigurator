<script setup lang="ts">
import FirmwareUpdate from '@/components/FirmwareUpdate.vue'
import Settings4K from '@/components/meowpad4k/Settings.vue'
import Settings3K from '@/components/meowpad3k/Settings.vue'
import DeviceList from '@/components/DeviceList.vue'
import Pure64 from '@/components/meowboard/Keyboard.vue'
import DeveloperSettings from '@/components/DeveloperSetting/DeveloperSettings.vue'
import { useI18n } from "vue-i18n";
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import emitter from "@/mitt";
import * as api from '@/apis/api'
import * as api4k from '@/apis/meowpad4k/api'
import * as api3k from '@/apis/meowpad3k/api'
import * as apib from '@/apis/meowboard/api'
import { useDialog } from 'naive-ui'
import { IError, IHidDeviceInfo } from '@/apis';
import { compareArray, getErrorMsg } from '@/utils';
import { appWindow, LogicalSize } from '@tauri-apps/api/window';

const { t } = useI18n();
const dialog = useDialog()
const store = useStore()
const device = useDeviceStore()

emitter.on('refresh-device-list', async (event: { e: IError }) => {
  if (store.refreshing_device_list) { return }
  
  store.refreshing_device_list = true

  try {
    store.device_list = await api.device_list();
    console.table(store.device_list)

    if (device.device_hid_info != undefined) {
      let has_device = false

      for (let i = 0; i < store.device_list.length; i++) {
        if (store.device_list[i].product_id == device.device_hid_info.product_id && store.device_list[i].vendor_id == device.device_hid_info.vendor_id) {
          if (compareArray(store.device_list[i].path, device.device_hid_info.path)) {
            has_device = true
          }
        }
      }
      
      if (!has_device) {
        emitter.emit('connection-broke', {
          e: {
            type: 'device_disconnected',
            data: undefined
          }
        })
      }
    }
  } finally {
    store.refreshing_device_list = false
  }
})

emitter.on('connection-broke', async (event: { e: IError | null }) => {
  if (event.e != null) { 
    emitter.emit('header-msg-update', { status: "error", str: t('connection_broke', { e: getErrorMsg(t, event.e) }) })
    console.error(event.e)
  }
  device.connected = false
  device.device_hid_info = undefined;
  device.device_info = undefined
  device.raw_config = undefined
  device.key_config = undefined
  device.light_config = undefined
  store.iap_connected = false
  store.developer_mode = false
  appWindow.setSize(new LogicalSize(800, 600))
})


emitter.on('connect', async (event: { device: IHidDeviceInfo }) => {
  emitter.emit('header-loading', { str: t('connecting') })
  try {
    let device_hid_info = event.device;

    if (!await api.connect_device(device_hid_info)) {
      emitter.emit('header-msg-update', { status: "error", str: t('connection_broke', { e: t('device_not_found') }) })
      return
    }

    device.device_hid_info = device_hid_info;

    let firmware_version = store.firmware_versions.get(device_hid_info.device_name);
    if (firmware_version == undefined) {
      emitter.emit('header-msg-update', { status: "error", str: t('unknown_device') })
      return
    }

    if (device.is_4k()) {
      device.device_info = await api4k.get_device_info()
    } else if (device.is_3k()) {
      device.device_info = await api3k.get_device_info()
    } else if (device.is_pure()) {
      device.device_info = await apib.get_device_info()
    }

    console.table(device.device_info)

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
      if (device.is_pure()) {
        await apib.set_key_config(await apib.get_default_key_config())
        await apib.save_key_config()
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
      try {
        if (device.is_4k()) {
        device.raw_config = await api4k.get_raw_config()
        }
        if (device.is_3k()) {
          device.raw_config = await api3k.get_raw_config()
        }
        if (device.is_pure()) {
          device.raw_config = await apib.get_raw_config()
        }
      } catch (e: IError | any) {
        if (e.type === 'meowpad' && e.data === 'config_cbor_parse_failed') {
          device.raw_config = "配置不支持，可能是固件版本不匹配"
        } else {
          throw e
        }
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
      if (device.is_pure()) {
        device.device_config = await apib.get_key_config()
        device.extract_key_config_pure64()
      }
    }

    if (device.device_info === undefined) {
      emitter.emit('header-msg-update', { status: "success", str: t('connected') })
    } else {
      emitter.emit('header-msg-update', { status: "success", str: t('connected_device', { version: device.device_info!.version }) })
    }

    device.connected = true

    if (device.is_pure() && !store.developer_mode) {
      appWindow.setSize(new LogicalSize(1200, 750))
    }

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
    emitter.emit('connection-broke', { e: e as IError })
    console.error(e)
    return;
  }
})

</script>

<template>
  <n-spin :show="store.loading" id="spin-cover">
    <div id="main">
      <template v-if="store.developer_mode">
        <DeveloperSettings></DeveloperSettings>
      </template>

      <template v-else-if="device.connected">
        <template v-if="device.is_4k()">
          <Settings4K></Settings4K>
        </template>
        <template v-else-if="device.is_3k()">
          <Settings3K></Settings3K>
        </template>
        <template v-else-if="device.is_pure()">
          <Pure64></Pure64>
        </template>
      </template>

      <template v-else-if="store.need_update_firmware">
        <FirmwareUpdate></FirmwareUpdate>
      </template>

      <template v-else>
        <div v-if="store.device_list.length > 0">
          <DeviceList></DeviceList>
        </div>
        <div v-else>
          <n-empty :description="t('no_device')" size="huge"></n-empty>
        </div>
      </template>

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
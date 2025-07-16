<script setup lang="ts">
import FirmwareUpdate from '@/components/FirmwareUpdate.vue'
import SettingsV3 from '@/components/meowpadv3/Settings.vue'
import SettingsV2 from '@/components/meowpadv2/Settings.vue'
import SettingsV2SE from '@/components/meowpadv2se/Settings.vue'
import SettingsV21SE from '@/components/meowpadv21se/Settings.vue'
import Pure64 from '@/components/pure64/Keyboard.vue'
import DeviceList from '@/components/DeviceList.vue'
import DeveloperSettings from '@/components/DeveloperSetting/DeveloperSettings.vue'
import { useI18n } from "vue-i18n";
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import emitter from "@/mitt";
import * as api from '@/apis/api'
import * as apiv2 from '@/apis/meowpadv2/api'
import * as apiv2se from '@/apis/meowpadv2se/api'
import * as apiv21se from '@/apis/meowpadv21se/api'
import * as apib from '@/apis/pure64/api'
import * as apiv3 from '@/apis/meowpadv3/api'
import { useDialog } from 'naive-ui'
import { IError, IHidDeviceInfo } from '@/apis';
import { compareArray, getErrorMsg } from '@/utils';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize } from '@tauri-apps/api/dpi'

const { t } = useI18n();
const dialog = useDialog()
const message = useMessage();
const store = useStore()
const device = useDeviceStore()

emitter.on('refresh-device-list', async (event: { e: IError }) => {
  if (store.refreshing_device_list) { return }
  
  store.refreshing_device_list = true

  try {
    store.device_list = await api.device_list();
    console.log(store.device_list)

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
  const appWindow = getCurrentWebviewWindow()
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
  const appWindow = getCurrentWebviewWindow()
  emitter.emit('header-loading', { str: t('connecting') })
  try {
    let device_hid_info = event.device;

    if (!await api.connect_device(device_hid_info)) {
      emitter.emit('header-msg-update', { status: "error", str: t('connection_broke', { e: t('device_not_found') }) })
      return
    }

    device.device_hid_info = device_hid_info;

    
    await device.get_info();
    
    if (device.device_info === undefined) {
      emitter.emit('header-msg-update', { status: "error", str: t('unknown_device') })
      return
    }

    console.table(device.device_info)

    if (store.version_info) {
      store.latest_firmware_download_url = "https://desu.life/#device"
    }

    await device.get_status()

    if (store.developer_mode) {
      try {
        await device.get_config_raw()
      } catch (e: IError | any) {
        if (e.type === 'meowpad' && e.data === 'config_cbor_parse_failed') {
          device.raw_config = t('unsupported_config')
        } else {
          throw e
        }
      }
    } else {
      // 重置状态
      let firmware_version = store.firmware_versions.get(device_hid_info.device_name);
      if (firmware_version == undefined) {
        emitter.emit('header-msg-update', { status: "error", str: t('unknown_device') })
        return
      }

      if (!firmware_version.includes(device.device_info!.version)) {
        if (!store.developer_mode) {
          store.need_update_firmware = true // 需要更新固件
          emitter.emit('header-msg-update', { status: "error", str: t('bad_firmware_version', { version: device.device_info!.version }) })
          return
        }
      }

      if (device.device_status!.key === false) {
        if (device.is_v2()) {
          await apiv2.set_key_config(await apiv2.get_default_key_config())
          await apiv2.save_key_config()
        }
        if (device.is_v2se()) {
          await apiv2se.set_key_config(await apiv2se.get_default_key_config())
          await apiv2se.save_key_config()
        }
        if (device.is_pure()) {
          await apib.set_key_config(await apib.get_default_key_config())
          await apib.save_key_config()
        }
        if (device.is_v21se()) {
          await apiv21se.set_key_config(await apiv21se.get_default_key_config())
          await apiv21se.save_key_config()
        }
        if (device.is_v3()) {
          await apiv3.set_key_config(await apiv3.get_default_key_config())
          await apiv3.save_key_config()
        }
      }

      if (device.device_status!.light != undefined && device.device_status!.light != null) {
        if (device.device_status!.light === false) {
          if (device.is_v2()) {
            await apiv2.set_light_config(await apiv2.get_default_light_config())
            await apiv2.save_light_config()
          }
          if (device.is_v2se()) {
            await apiv2se.set_light_config(await apiv2se.get_default_light_config())
            await apiv2se.save_light_config()
          }
          if (device.is_v21se()) {
            await apiv21se.set_light_config(await apiv21se.get_default_light_config())
            await apiv21se.save_light_config()
          }
        }
      }

      if (device.is_v2()) {
        device.key_config = await apiv2.get_key_config()
        device.extract_key_config_v2()
        device.light_config = await apiv2.get_light_config()
        device.extract_light_config_v2()
      }
      if (device.is_v2se()) {
        device.key_config = await apiv2se.get_key_config()
        device.extract_key_config_v2se()
        device.light_config = await apiv2se.get_light_config()
        device.extract_light_config_v2se()
      }
      if (device.is_pure()) {
        device.device_config = await apib.get_key_config()
        device.extract_key_config_pure64()
      }
      if (device.is_v21se()) {
        device.key_config = await apiv21se.get_key_config()
        device.extract_key_config_v21se()
        device.light_config = await apiv21se.get_light_config()
        device.extract_light_config_v21se()
      }
      if (device.is_v3()) {
        device.device_config = await apiv3.get_key_config()
        device.extract_key_config_v3()
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

    if (device.device_status!.hall == false && !store.developer_mode) {
      if (device.is_pure()) {
        message.warning(t('device_cali_suggest'));
      } else {
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
        <template v-if="device.is_v2()">
          <SettingsV2></SettingsV2>
        </template>
        <template v-else-if="device.is_v2se()">
          <SettingsV2SE></SettingsV2SE>
        </template>
        <template v-else-if="device.is_pure()">
          <Pure64></Pure64>
        </template>
        <template v-else-if="device.is_v21se()">
          <SettingsV21SE></SettingsV21SE>
        </template>
        <template v-else-if="device.is_v3()">
          <SettingsV3></SettingsV3>
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
  z-index: 500;
}
</style>
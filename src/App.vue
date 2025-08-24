<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Main from '@/components/Main.vue'
import Application from './components/Application.vue';
import { darkTheme } from "naive-ui";
import { LogicalSize } from '@tauri-apps/api/dpi';
import { getCurrentWebviewWindow, WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { NConfigProvider, GlobalThemeOverrides } from 'naive-ui'
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { IVersion } from './apis';
import { check_update, device_list, get_latest_version, get_theme, open_update_url, refresh_devices, update_firmware_call } from './apis/api';
import { useI18n } from 'vue-i18n';
import emitter from '@/mitt';
import * as apiv2 from '@/apis/meowpadv2/api'
import * as apiv2se from '@/apis/meowpadv2se/api'
import * as apiv21se from '@/apis/meowpadv21se/api'
import * as apip64 from '@/apis/pure64/api'
import * as apiv3 from '@/apis/meowpadv3/api'
import { emit } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";


const lightThemeOverrides: GlobalThemeOverrides = {
  Layout: {
    color: '#F7F7F7',
    headerColor: '#F7F7F7',
  } 
}

const { t } = useI18n();
const store = useStore()
const theme = ref<string>()


// 禁用webkit右键菜单
document.body.onselectstart = document.body.oncontextmenu = () => false

async function get_firmware_versions() {
  store.firmware_versions.set("Meowpad", await apiv2.get_firmware_version())
  store.firmware_versions.set("Meowpad SE v2", await apiv2se.get_firmware_version())
  store.firmware_versions.set("Meowpad SE v2.1", await apiv21se.get_firmware_version())
  store.firmware_versions.set("Pure64", await apip64.get_firmware_version())
  store.firmware_versions.set("MeowpadV3", await apiv3.get_firmware_version())
  console.log(store.firmware_versions)
}

onMounted(async () => {
  const appWindow = getCurrentWebviewWindow()
  await store.load()
  await store.save()
  store.status_str = t("device_disconnected")

  get_latest_version().then(async (version) => {
    console.log(version)
    store.version_info = version
    if (version.length > 0) {
      let need_update = await check_update(version)
      if (need_update) {
        await open_update_url(version[0], t('update_warning'))
      }
    }
  });

  theme.value = await get_theme();
  await appWindow.onThemeChanged(async ({ payload: t }) => {
    theme.value = await get_theme();
  })

  appWindow.setSize(new LogicalSize(800, 600))
  await appWindow.show()

  await get_firmware_versions()

  emitter.emit('refresh-device-list')

  const interval = setInterval(async () => {
    if (store.refreshing_devices) { return }

    try {
      store.refreshing_devices = true
      let changes = await refresh_devices();
      if (changes) {
        emitter.emit('refresh-device-list')
      }
    } finally {
      store.refreshing_devices = false
    }
  }, 1000)
})

store.key_detection_status = false
document.addEventListener('keydown', function (event) {
  if (store.key_detection_status === false) {
    if (event.code === "F5") {
      event.preventDefault();
      location.reload();
    }
  
    if (event.code === "Escape") {
      event.preventDefault();
      emitter.emit('connection-broke', { e: null })
      store.status_str = t("device_disconnected")
      store.status = undefined
    }
  }
});

</script>

<template>
  <!-- <n-theme-editor> -->
  <n-config-provider :theme="theme == 'dark' ? darkTheme : undefined"
    :theme-overrides="theme == 'dark' ? null : lightThemeOverrides">
    <Application>
      <n-layout>
        <n-layout-header class="header">
          <Header />
        </n-layout-header>
        <n-layout-content class="main">
          <Main />
        </n-layout-content>
      </n-layout>
    </Application>
  </n-config-provider>
  <!-- </n-theme-editor> -->
</template>

<style lang="scss" scoped>
.header {
  height: var(--header-height);
  background-color: var(--color-background) !important;
  display: flex;
  align-items: center;
  justify-content: center;
}

.main {
  height: calc(100vh - var(--header-height));
  color: var(--color-text);
  background: var(--color-background);


}
</style>
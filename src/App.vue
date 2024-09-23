<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Main from '@/components/Main.vue'
import Application from './components/Application.vue';
import { darkTheme } from "naive-ui";
import { appWindow, Theme } from "@tauri-apps/api/window";
import { NConfigProvider, GlobalThemeOverrides } from 'naive-ui'
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { IVersion } from './apis';
import { check_update, get_latest_version, get_theme, open_update_url } from './apis/api';
import { useI18n } from 'vue-i18n';

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


onMounted(async () => {
  get_latest_version().then(async (version: IVersion) => {
    store.version_info = version
    let need_update = await check_update(version)
    if (need_update) {
      await open_update_url(version, t('update_warning'))
    }
  });

  theme.value = await get_theme();
  console.log(theme.value)
  await appWindow.onThemeChanged(({ payload: t }) => {
    theme.value = t as string
  })
  
  await appWindow.show()
})
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
  display: flex;
  align-items: center;
  flex-direction: row;
  justify-content: space-between;
}

.main {
  height: calc(100vh - var(--header-height));
  color: var(--color-text);
  background: var(--color-background);
}

</style>
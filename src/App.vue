<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Main from '@/components/Main.vue'
import Application from './components/Application.vue';
import { darkTheme } from "naive-ui";
import { appWindow, LogicalSize, Theme } from "@tauri-apps/api/window";
import { NConfigProvider, GlobalThemeOverrides } from 'naive-ui'
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { IVersion } from './apis';
import { check_update, get_latest_version } from './apis/api';
import emitter from './mitt';

function onGlobalMouseMove(event: MouseEvent) {
  // TODO: 写这一块的时候发现了性能问题，配置器动不动就会拿到10%以上的cpu占用
  emitter.emit('global-mouse-move', event)
}

emitter.off("mouse-move-for-key-drag")

function onGlobalMouseUp(event: MouseEvent) {
  emitter.emit('global-mouse-up', event)
}

const lightThemeOverrides: GlobalThemeOverrides = {
  Layout: {
    color: '#F7F7F7',
    headerColor: '#F7F7F7',
  }
}

const store = useStore()
const theme = ref<Theme | null>(null)

// 禁用webkit右键菜单
document.body.onselectstart = document.body.oncontextmenu = () => false

onMounted(async () => {
  let show = true;
  
  // theme.value = await appWindow.theme()
  // await appWindow.onThemeChanged(({ payload: t }) => {
  //   theme.value = t
  // })

  theme.value = "dark"

  setTimeout(async () => {
    if (show) {
      await appWindow.show()
    }
  }, 500)

  // appWindow.setSize(new LogicalSize(1080, 900))
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
        <n-layout-content>
          <div id="main"  @mousemove="onGlobalMouseMove" @mouseup="onGlobalMouseUp">
            <Main />
          </div>
        </n-layout-content>
      </n-layout>
    </Application>
  </n-config-provider>
  <!-- </n-theme-editor> -->
</template>

<style lang="scss">
$header-height: 56px;

.header {
  height: $header-height;
  display: flex;
  align-items: center;
  flex-direction: row;
  justify-content: space-between;
}


#main {
  height: calc(100vh - $header-height);
  overflow: hidden;
}
</style>
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createI18n } from 'vue-i18n'
import '@/style.scss'
import 'animate.css';
import App from '@/App.vue'
import { i18n } from '@/locales/index'

import { createMemoryHistory, createRouter } from 'vue-router'

import DeviceList from '@/components/DeviceList.vue'
import FirmwareUpdate from '@/components/FirmwareUpdate.vue'
import SettingsV3 from '@/components/meowpadv3/Settings.vue'
import SettingsV2 from '@/components/meowpadv2/Settings.vue'
import SettingsV2SE from '@/components/meowpadv2se/Settings.vue'
import SettingsV21SE from '@/components/meowpadv21se/Settings.vue'
import Pure64 from '@/components/pure64/Keyboard.vue'
import DeveloperSettings from '@/components/DeveloperSetting/DeveloperSettings.vue';

const routes = [
  { path: '/', component: DeviceList },
  { path: '/firmware-update', component: FirmwareUpdate },
  { path: '/meowpadv3', component: SettingsV3 },
  { path: '/meowpadv2', component: SettingsV2 },
  { path: '/meowpadv2se', component: SettingsV2SE },
  { path: '/meowpadv21se', component: SettingsV21SE },
  { path: '/pure64', component: Pure64 },
  { path: '/developer-settings', component: DeveloperSettings },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

createApp(App).use(createPinia()).use(i18n).use(router).mount('#app')

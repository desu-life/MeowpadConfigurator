import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createI18n } from 'vue-i18n'
import '@/style.scss'
import 'animate.css';
import App from '@/App.vue'
import { i18n } from '@/locales/index'

createApp(App).use(createPinia()).use(i18n).mount('#app')

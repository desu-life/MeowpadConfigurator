import { createApp } from 'vue'
import { createPinia } from 'pinia'
import '@/style.scss'
import 'animate.css';
import App from '@/App.vue'

createApp(App).use(createPinia()).mount('#app')

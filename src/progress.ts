import { createApp } from "vue";
import Progress from '@/components/Progress.vue';
import { i18n } from '@/locales/index'
import '@/style.scss'

createApp(Progress).use(i18n).mount("#app");
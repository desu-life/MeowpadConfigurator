//index.ts
import { createI18n } from 'vue-i18n'        //引入vue-i18n组件
import zh from './zh.json'
import en from './en.json'

export const SUPPORT_LOCALES = ['zh', 'en']

const messages = {
    zh: {
        ...zh,
    },
    en: {
        ...en,
    }
}

function setupI18n() {
    const i18n = createI18n({
        legacy: false,
        messages: messages,
        locale: 'zh',
        fallbackLocale: 'zh',
    })
    return i18n
}

export function setI18nLanguage(i18n, locale) {
    if (i18n.mode === 'legacy') {
        i18n.global.locale = locale
    } else {
        i18n.global.locale.value = locale
    }
    /**
     * NOTE:
     * If you need to specify the language setting for headers, such as the `fetch` API, set it here.
     * The following is an example for axios.
     *
     * axios.defaults.headers.common['Accept-Language'] = locale
     */
    document.querySelector('html')!.setAttribute('lang', locale)
}

export const i18n = setupI18n();

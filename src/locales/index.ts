//index.ts
import { createI18n } from "vue-i18n"; //引入vue-i18n组件
import zh from "./zh.json";
import en from "./en.json";
import ja from "./ja.json";
import ko from "./ko.json";
import zhHant from "./zh-Hant.json";

export declare type LOCALES = "zh" | "en" | "ja" | "ko" | "zhHant";

const messages = {
  zh: {
    ...zh,
  },
  en: {
    ...en,
  },
  ja: {
    ...ja,
  },
  ko: {
    ...ko,
  },
  zhHant: {
    ...zhHant,
  },
};

function setupI18n() {
  const i18n = createI18n({
    legacy: false,
    messages: messages,
    locale: "en",
    fallbackLocale: "zh",
  });
  return i18n;
}

export function setI18nLanguage(locale: string) {
  if (
    locale === "zh" ||
    locale === "en" ||
    locale === "ja" ||
    locale === "ko" ||
    locale === "zhHant"
  ) {
    i18n.global.locale.value = locale;
    return true;
  }
  return false;

  /**
   * NOTE:
   * If you need to specify the language setting for headers, such as the `fetch` API, set it here.
   * The following is an example for axios.
   *
   * axios.defaults.headers.common['Accept-Language'] = locale
   */
  // document.querySelector('html')!.setAttribute('lang', locale)
}

export const i18n = setupI18n();

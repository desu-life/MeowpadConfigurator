<template>
  <n-config-provider :theme="theme == 'dark' ? darkTheme : undefined"
    :theme-overrides="theme == 'dark' ? null : lightThemeOverrides">
    <n-layout-content>
      <div class="window-content">
        <div class="labels-row">
          <div class="label1">{{ t('updating_firmware') }}</div>
          <div class="label2">{{ progressText }}</div>
        </div>
        <div class="progress-container">
          <n-progress :percentage="progress" :show-indicator="false" :height="18" border-radius="2" />
        </div>
      </div>
    </n-layout-content>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { GlobalThemeOverrides, NProgress } from "naive-ui";
import { get_theme } from "@/apis/api";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { darkTheme } from "naive-ui";
import { useI18n } from "vue-i18n";
import { LazyStore } from "@tauri-apps/plugin-store";
import { setI18nLanguage } from "@/locales";

document.body.onselectstart = document.body.oncontextmenu = () => false

if (navigator.language === "zh-CN") {
  setI18nLanguage("zh");
}

const { t } = useI18n();
const app_store = new LazyStore(".settings.dat");

const lightThemeOverrides: GlobalThemeOverrides = {
  Layout: {
    color: '#F7F7F7',
    headerColor: '#F7F7F7',
  }
}
const theme = ref<string>()

const progress = ref(0);
const progressText = ref(Math.round(progress.value) + '%');

onMounted(async () => {
  const appWindow = getCurrentWebviewWindow()
  await listen<number>("progress-update", async (event) => {
    progress.value = event.payload;
    progressText.value = Math.round(progress.value) + '%';
  });

  await listen("progress-close", (_) => {
    appWindow.close();
  });

  theme.value = await get_theme();
  await appWindow.onThemeChanged(async ({ payload: t }) => {
    theme.value = await get_theme();
  })

  const language = await app_store.get<string>("language");
  if (language) {
    setI18nLanguage(language);
  }
});
</script>

<style scoped>
.window-content {
  padding: 14px 12px;
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.labels-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 16px;
}

.label1 {
  font-size: 11px;
  line-height: 16px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
  margin-right: 10px;
}

.label2 {
  font-size: 11px;
  line-height: 16px;
  white-space: nowrap;
}

.progress-container {
  height: 18px;
}
</style>
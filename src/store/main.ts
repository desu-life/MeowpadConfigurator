import { defineStore, acceptHMRUpdate } from "pinia";
import { DeviceName, IDevicePreset, IHidDeviceInfo, IVersion } from "@/apis";
import { Type } from "naive-ui/es/button/src/interface";
import { Toggle } from "@/interface";
import { LOCALES, setI18nLanguage } from "@/locales";
import { Store } from "tauri-plugin-store-api";

export const useStore = defineStore("main", () => {
  const app_store = new Store(".settings.dat");
  const device_presets_store = new Store(".device.presets.dat");
  const status = ref<Type | undefined>(undefined);
  const status_str = ref("");
  const lang = ref<LOCALES>("en");
  const refreshing_device_list = ref(false);
  const loading = ref(false);
  const iap_connected = ref(false);
  const version_info = ref<IVersion[] | undefined>(undefined);
  const latest_firmware_download_url = ref<string | null>(null);
  const need_update_firmware = ref<boolean>(false);
  const developer_mode = ref<boolean>(false);
  const debug_mode = ref<boolean>(false);
  const can_sync = ref<boolean>(false);
  const need_check = ref(false);
  const bottom_dz_available = ref(Toggle.Off);
  const device_list = ref<IHidDeviceInfo[]>([]);
  const firmware_versions = ref<Map<DeviceName, string>>(new Map());
  const presets = ref<IDevicePreset[]>([]);
  const current_preset = ref<IDevicePreset | null>(null);

  if (navigator.language === "zh-CN") {
    setLang("zh");
  }

  function setLang(e: string) {
    if (setI18nLanguage(e)) {
      lang.value = e as LOCALES;
      return true;
    }
    return false;
  }

  async function save() {
    await app_store.set("language", lang.value);
    await app_store.set(
      "bottom_dz_available",
      bottom_dz_available.value === Toggle.On
    );
    await app_store.save();

    await device_presets_store.set("presets", presets.value);
    await device_presets_store.save();
  }

  async function load() {
    const language = await app_store.get<string>("language");
    if (language) {
      await setLang(language);
    }

    const bottom_dz = await app_store.get<boolean>("bottom_dz_available");
    if (bottom_dz) {
      bottom_dz_available.value = bottom_dz ? Toggle.On : Toggle.Off;
    } else {
      bottom_dz_available.value = Toggle.Off;
    }

    const p = await device_presets_store.get<IDevicePreset[]>("presets")
    if (p) {
      presets.value = p
    } else {
      presets.value = []
    }
  }

  return {
    status,
    status_str,
    iap_connected,
    version_info,
    need_update_firmware,
    developer_mode,
    debug_mode,
    can_sync,
    latest_firmware_download_url,
    loading,
    need_check,
    device_list,
    firmware_versions,
    refreshing_device_list,
    bottom_dz_available,
    lang,
    app_store,
    presets,
    device_presets_store,
    setLang,
    save,
    load,
    current_preset
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}

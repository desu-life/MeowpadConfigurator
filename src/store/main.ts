import { defineStore, acceptHMRUpdate } from "pinia";
import { IVersion } from "@/apis";
import { Type } from "naive-ui/es/button/src/interface";

export const useStore = defineStore("main", () => {
  const status = ref<Type | undefined>(undefined);
  const status_str = ref("");
  const loading = ref(false);
  const iap_connected = ref(false);
  const version_info = ref<IVersion | undefined>(undefined);
  const latest_firmware_download_url = ref<string | null>(null);
  const need_update_firmware = ref<boolean>(false);
  const developer_mode = ref<boolean>(false);
  const debug_mode = ref<boolean>(false);
  const can_sync = ref<boolean>(false);

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
    loading
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}

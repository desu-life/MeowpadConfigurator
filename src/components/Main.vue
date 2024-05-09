<script setup lang="ts">
import { useStore } from '@/store'
import FirmwareUpdate from '@/components/FirmwareUpdate.vue'
import Settings from '@/components/meowpad4k/Settings.vue'
import DeveloperSettings from '@/components/DeveloperSettings.vue'
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const store = useStore()
</script>

<template>
  <n-spin :show="store.loading" style="max-width: 640px;">
    <div v-if="store.developer_mode" class="debug">
      <DeveloperSettings></DeveloperSettings>
    </div>

    <div v-else-if="store.key_config != undefined && store.light_config != undefined && store.connected">
      <Settings></Settings>
    </div>

    <div v-else-if="store.need_update_firmware">
      <FirmwareUpdate></FirmwareUpdate>
    </div>

    <n-empty :description="t('no_device')" size="huge" v-else>
    </n-empty>
  </n-spin>
</template>

<style lang="scss">
.debug {
  width: 100%;
  height: 100%;
  min-height: 80vh;
}
</style>
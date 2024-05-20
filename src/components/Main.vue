<script setup lang="ts">
import FirmwareUpdate from '@/components/FirmwareUpdate.vue'
import Settings4K from '@/components/meowpad4k/Settings.vue'
import Settings3K from '@/components/meowpad3k/Settings.vue'
import DeveloperSettings from '@/components/DeveloperSettings.vue'
import { useI18n } from "vue-i18n";
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';

const { t } = useI18n();
const store = useStore()
const device = useDeviceStore()
</script>

<template>
  <n-spin :show="store.loading" style="max-width: 640px;">
    <div v-if="store.developer_mode" class="debug">
      <DeveloperSettings></DeveloperSettings>
    </div>

    <div v-else-if="device.connected">
      <div v-if="device.is_4k()">
        <Settings4K></Settings4K>
      </div>
      <div v-else-if="device.is_3k()">
        <Settings3K></Settings3K>
      </div>
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
</style>@/store/store
<script setup lang="ts">
import FirmwareUpdate from '@/components/FirmwareUpdate.vue'
import DeveloperSettings from '@/components/DeveloperSettings.vue'
import Keyboard from '@/components/Keyboard.vue'
import { useI18n } from "vue-i18n";
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';

const { t } = useI18n();
const store = useStore()
const device = useDeviceStore()
</script>

<template>
  <n-spin :show="store.loading">
    <div class="flex-main" v-if="store.developer_mode">
      <div  class="debug">
        <DeveloperSettings></DeveloperSettings>
      </div>
    </div>

    <template v-else>
      <template v-if="device.connected">
        <Keyboard></Keyboard>
      </template>
  
      <template v-else-if="store.need_update_firmware">
        <FirmwareUpdate></FirmwareUpdate>
      </template>
  
      <div class="flex-main"  v-else>
        <n-empty :description="t('no_device')" size="huge">
        </n-empty>
      </div>
    </template>
  </n-spin>
</template>

<style lang="scss">
$header-height: 56px;

.debug {
  width: 85%;
  height: 90%;
}

.flex-main {
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  height: calc(100vh - $header-height);
}
</style>@/store/store
<script setup lang="ts">
import { ref } from 'vue'
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import IAP from '@/components/DeveloperSetting/IAP.vue'
import { useI18n } from "vue-i18n";
import { FormValidationStatus } from 'naive-ui';

const { t } = useI18n();
const store = useStore()
const device = useDeviceStore()

const input_status = ref<FormValidationStatus | undefined>(undefined)

async function check_raw_config(value: string): Promise<void> {
  const res = await device.check_config_raw()
  if (res) {
    store.can_sync = true
    input_status.value = undefined
  } else {
    store.can_sync = false
    input_status.value = "error"
  }
}

</script>

<template>
  <div class="developer-config">
    <n-alert :title="$t('warning')" type="warning" style="margin-bottom: 10px;">
      {{ $t('developer_warning_2') }} </n-alert>
    <template v-if="device.connected">
      <template v-if="store.debug_mode">
        <template v-if="device.is_v3()">
          <DebugForV3></DebugForV3>
        </template>
        <template v-else>
          <Debug></Debug>
        </template>
      </template>
      <template v-else-if="device.raw_config != undefined">
        <n-input type="textarea" v-model:value="device.raw_config" :on-input="check_raw_config" :status="input_status"
          :autosize="{
            minRows: 3,
            maxRows: 15
          }" />
      </template>
    </template>
    <template v-else-if="store.iap_connected">
      <IAP></IAP>
    </template>
  </div>
</template>


<style lang="scss" scoped>
.developer-config {
  height: 460px;
  max-width: 85vw;
}
</style>
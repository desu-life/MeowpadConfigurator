<script setup lang="ts">
import { IKeyConfigBoard } from '@/apis/meowboard/config';
import { Toggle } from '@/interface';
import { useDeviceStore } from '@/store/device';
import { useStore } from '@/store/main';
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const device = useDeviceStore()
const store = useStore()

const ToggleSel = [
  {
    value: Toggle.On,
    label: t('on')
  },
  {
    value: Toggle.Off,
    label: t('off')
  },
]

function bottom_dz_available_change() {
  store.save()
}

</script>

<template>
  <n-card :bordered="false" class="keyboard-config-card" content-class="keyboard-config-card-content">
    <n-grid :cols="24" :x-gap="9" :y-gap="16">
      <n-form-item-gi :span="8" :label="t('light_color')" path="led_color" :show-feedback="false">
        <n-color-picker v-model:value="device.led_colors![0]" :show-alpha="false" :modes="['hex']"></n-color-picker>
      </n-form-item-gi>

      <n-form-item-gi :span="8" :label="t('light_brightness')" path="max_brightness" :show-feedback="false">
        <n-input-number v-model:value="device.max_brightness" :min="0" :max="100" :step="1">
          <template #suffix>
            %
          </template>
        </n-input-number>
      </n-form-item-gi>

      <n-gi :span="8"></n-gi>


      <n-form-item-gi :span="8" path="high_reportrate" :show-feedback="false">
        <n-select v-model:value="device.enable_hs" :options="ToggleSel" />
        <template #label>
          <n-tooltip trigger="hover" :delay="200">
            <template #trigger>
              <n-text underline>
                {{ t('enable_hs') }}
              </n-text>
            </template>
            <template #default>
              {{ $t('hs_desc') }}
            </template>
          </n-tooltip>
        </template>
      </n-form-item-gi>

      <n-form-item-gi :span="8" path="auto_calibration" :show-feedback="false">
        <n-select v-model:value="device.auto_calibration" :options="ToggleSel" />
        <template #label>
          <n-tooltip trigger="hover" :delay="200">
            <template #trigger>
              <n-text underline>
                {{ $t('auto_calibration') }}
              </n-text>
            </template>
            <template #default>
              {{ $t('auto_calibration_desc') }}
            </template>
          </n-tooltip>
        </template>
      </n-form-item-gi>

      <n-form-item-gi :span="8" path="bottom_dz_available" :show-feedback="false">
        <n-select v-model:value="store.bottom_dz_available" @update:value="bottom_dz_available_change"  :options="ToggleSel" />
        <template #label>
          {{ $t('bottom_dz_available') }}
        </template>
      </n-form-item-gi>
    </n-grid>
  </n-card>
</template>

<style scoped lang="scss">
.keyboard-config-card {
  --item-padding: 16px;

  background-color: var(--color-background-soft);
  border-radius: 8px;
  width: fit-content;

  .single-key-config {
    padding-bottom: var(--item-padding);
  }

  .device-config-card {
    background-color: var(--color-background-soft);
    border-radius: 8px;
    border: 1px solid var(--color-border);
    height: 100%;

    .device-config-key-config {
      width: 90%;
      display: flex;
      flex-direction: column;
      gap: 1px;
    }
  }
}
</style>

<style lang="scss">
// 这个不能scoped
.device-config-key-config .n-form-item-blank {
  max-width: 250px;
  gap: 1px;
}

.device-config-card-content {
  padding: 0px !important;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
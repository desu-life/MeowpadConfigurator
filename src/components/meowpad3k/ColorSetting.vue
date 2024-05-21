<script setup lang="ts">
import { LightingMode } from '@/apis/meowpad4k/config';
import { Toggle } from '@/interface';
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { Rgb2Hex, Hex2Rgb, IsModifierKey, compareArray } from '@/utils';
import { useI18n } from "vue-i18n";
import { storeToRefs } from 'pinia';
import { ILighting } from "@/apis/meowpad3k/config";


const { t } = useI18n();
const store = useDeviceStore()
const { light_config } = storeToRefs(store)
const light_cfg = light_config as Ref<ILighting>;

const lightingMode = ref(LightingMode.Solid)

</script>

<template>
  <n-grid :cols="24" :x-gap="18" style="width: 82vw;">
    <n-gi :span="20">
      <n-grid :cols="20" :x-gap="20">
        <n-gi :span="5">
          <n-collapse-transition>
            <n-form-item :label="$t('led_color_num', { num: 'K1' })" path="led_colors_0">
              <n-color-picker v-model:value="store.led_colors![0]" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition>
            <n-form-item :label="$t('led_color_num', { num: 'K2' })" path="led_colors_1">
              <n-color-picker v-model:value="store.led_colors![1]" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition>
            <n-form-item :label="$t('led_color_num', { num: 'K3' })" path="led_colors_2">
              <n-color-picker v-model:value="store.led_colors![2]" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition :show="lightingMode != LightingMode.Off">
            <n-form-item :label="$t('maximum_brightness')" path="max_brightness">
              <n-slider v-model:value="store.max_brightness" :step="1" :max="100" :min="1" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
      </n-grid>
      <n-collapse-transition :show="lightingMode == LightingMode.Off">
        <n-empty :description="$t('no_option')" size="huge" style="margin-top: 40px;"></n-empty>
      </n-collapse-transition>
    </n-gi>
  </n-grid>
</template>
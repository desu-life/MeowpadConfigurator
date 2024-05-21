<script setup lang="ts">
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import KeyModal from '@/components/KeyModal.vue'
import Keyboard from './Keyboard.vue'
import { h } from 'vue'
import { useI18n } from "vue-i18n";
import { IKeymap, Toggle } from '@/interface';

import meowpad from '@/meowpad3k.json'
const keymap: IKeymap[][] = meowpad;

import { ILighting } from "@/apis/meowpad3k/config";
import { storeToRefs } from 'pinia';

const { t } = useI18n();
const message = useMessage()
const dialog = useDialog()
const store = useDeviceStore()
const { light_config } = storeToRefs(store)
const light_cfg = light_config as Ref<ILighting>;


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

function GetToggleSel() {
  return ToggleSel
}
</script>

<template>
  <div style="width: 80vw;height: 30vh;" class="key-config">
    <Keyboard :keymap="keymap"></Keyboard>
  </div>

  <n-grid :cols="20" :x-gap="18" style="width: 80vw;">

    <n-gi :span="5">
      <n-form-item :label="$t('keyboard_jitters_elimination_time')" path="jitters_elimination_time">
        <n-input-number v-model:value="store.jitters_elimination_time" :min="0" :max="50" :step="0.5">
          <template #suffix>
            ms
          </template>
        </n-input-number>
      </n-form-item>
    </n-gi>
    <n-gi :span="5">
      <n-form-item :label="$t('device_sleep_idle_time')" path="sleep_time">
        <n-input-number v-model:value="light_cfg!.sleep_time" :min="0" :max="65535">
          <template #suffix>
            {{ $t('sec') }}
          </template>
        </n-input-number>
      </n-form-item>
    </n-gi>
    <n-gi :span="5">
      <n-form-item :label="$t('continous_report')" path="continuous_report">
        <n-select v-model:value="store.continuous_report" :options="GetToggleSel()" />
      </n-form-item>
    </n-gi>
    <n-gi :span="3">
      <n-form-item :label="$t('kalman_filter')" path="kalman_filter">
        <n-select v-model:value="store.kalman_filter" :options="GetToggleSel()" />
      </n-form-item>
    </n-gi>
  </n-grid>
</template>

<style>
/* .hs_key_setting > .n-form-item-feedback-wrapper {
  display: block;
} */

.badge {
  position: absolute;
  height: 18px;
  line-height: 18px;
  border-radius: 9px;
  padding: 0 6px;
  text-align: center;
  font-size: var(--n-font-size);
  transform: translateX(-50%);
  left: 100%;
  bottom: calc(100% - 9px);
  /* bottom: calc(100% - 9px); */
  font-variant-numeric: tabular-nums;
  z-index: 1;
  display: flex;
  align-items: center;
  width: fit-content;
}
</style>@/store/store
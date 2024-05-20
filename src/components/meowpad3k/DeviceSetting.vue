<script setup lang="ts">
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import KeyModal from '@/components/KeyModal.vue'
import Keyboard from '@/components/Keyboard.vue'
import { h } from 'vue'
import { useI18n } from "vue-i18n";
import { IKeymap, Toggle } from '@/interface';
import { LightingMode } from '@/apis/meowpad4k/config';
import meowpad from '@/meowpad4k.json'
const keymap: IKeymap[][] = meowpad;


const { t } = useI18n();
const message = useMessage()
const dialog = useDialog()
const store = useDeviceStore()


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

const LighingModeSelWooting = [
  {
    value: LightingMode.Off,
    label: t('off')
  },
  {
    value: LightingMode.Solid,
    label: t('solid')
  },
  {
    value: LightingMode.RainbowMode,
    label: t('rainbow_gradient_switch')
  },
  {
    value: LightingMode.RainbowFlowMode,
    label: t('rainbow_flow')
  },
  {
    value: LightingMode.PressRadianceMode,
    label: t('ya-gan-mo-shi')
  },
  {
    value: LightingMode.BreatheGlowMode,
    label: t('rainbow_breath_sync')
  },
  {
    value: LightingMode.BreatheGlowAsyncMode,
    label: t('rainbow_breath_switch')
  },
  {
    value: LightingMode.RainDropMode,
    label: t('rain_drop')
  },
  {
    value: LightingMode.TapToGlowMode,
    label: t('press_and_light')
  },
  {
    value: LightingMode.SpeedLightMode,
    label: t('speed_press')
  },
]

function GetLighingModeSel() {
  return LighingModeSelWooting
}
function GetToggleSel() {
  return ToggleSel
}




</script>

<template>
  <!-- <n-space justify="space-between"> -->

  <div style="width: 80vw;height: 30vh;" class="key-config">
    <Keyboard :keymap="keymap"></Keyboard>
  </div>

  <!-- </n-space> -->
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
    <n-gi :span="10">
      <n-form-item :label="$t('lighting_mode')" path="lighting_mode">
        <n-select v-model:value="store.light_config!.lighting_mode" :options="GetLighingModeSel()" />
      </n-form-item>
    </n-gi>
    <n-gi :span="5">
      <n-form-item :label="$t('device_sleep_idle_time')" path="sleep_time">
        <n-input-number v-model:value="store.light_config!.sleep_time" :min="0" :max="65535">
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
    <n-gi :span="10">
      <n-form-item :label="$t('lighting_mode_sleep')" path="lighting_mode_sleep">
        <n-select v-model:value="store.light_config!.lighting_mode_sleep" :options="GetLighingModeSel()" />
      </n-form-item>
    </n-gi>
    <n-gi :span="5">
      <n-form-item :label="$t('enable_hs')" path="kalman_filter">
        <n-select v-model:value="store.enable_hs" :options="GetToggleSel()" />
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
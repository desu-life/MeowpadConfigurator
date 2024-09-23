<script setup lang="ts">
import { LightingMode } from '@/apis/meowpad4k/config';
import { Toggle } from '@/interface';
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { Rgb2Hex, Hex2Rgb, IsModifierKey, compareArray } from '@/utils';
import { useI18n } from "vue-i18n";
import { storeToRefs } from 'pinia';
import { ILighting } from "@/apis/meowpad4k/config";

const { t } = useI18n();
const store = useDeviceStore()
const { light_config } = storeToRefs(store)
const light_cfg = light_config as Ref<ILighting>;

const canChangeColor = ref(true)
const lightingMode = ref(LightingMode.Solid)

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


const LighingMode = [
  {
    key: LightingMode.Solid,
    label: t('solid')
  },
  {
    key: LightingMode.RainbowMode,
    label: t('rainbow_gradient_switch')
  },
  {
    key: LightingMode.RainbowFlowMode,
    label: t('rainbow_flow')
  },
  {
    key: LightingMode.BreatheGlowAsyncMode,
    label: t('rainbow_breath_switch')
  },
  {
    key: LightingMode.RainDropMode,
    label: t('rain_drop')
  },
  {
    key: LightingMode.TapToGlowMode,
    label: t('press_and_light')
  },
  {
    key: LightingMode.SpeedLightMode,
    label: t('speed_press')
  },
]



function lightingModeUpdate(type: string) {
  console.log(type)
  CanCanChangeColor()
  return
}

function CanCanChangeColor() {
  if (light_cfg == null) { canChangeColor.value = false; return; }

  switch (lightingMode.value) {
    case null:
      canChangeColor.value = false
      break
    case LightingMode.Off:
      canChangeColor.value = false
      break
    case LightingMode.RainbowFlowMode:
      canChangeColor.value = false
      break
    case LightingMode.RainbowMode:
      canChangeColor.value = false
      break
    case LightingMode.BreatheGlowMode:
      canChangeColor.value = false
      break
    case LightingMode.BreatheGlowAsyncMode:
      canChangeColor.value = false
      break
    case LightingMode.RainDropMode:
      canChangeColor.value = false
      break
    default:
      canChangeColor.value = true
      break
  }
}
</script>

<template>
  <n-grid :cols="24" :x-gap="18" style="width: 82vw;">
    <n-gi :span="4" >
      <n-form-item :label="$t('lighting_mode')" path="lighting_mode">
        <n-menu v-model:value="lightingMode" :options="LighingMode" inverted :indent="18"
          @update:value="lightingModeUpdate" />
      </n-form-item>
    </n-gi>
    <n-gi :span="20">
      <n-grid :cols="20" :x-gap="20">
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item :label="$t('led_color_num', { num: 'K1' })" path="led_colors_0">
              <n-color-picker v-model:value="store.led_colors![0]" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item :label="$t('led_color_num', { num: 'K2' })" path="led_colors_1">
              <n-color-picker v-model:value="store.led_colors![1]" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item :label="$t('led_color_num', { num: 'K3' })" path="led_colors_2">
              <n-color-picker v-model:value="store.led_colors![2]" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item :label="$t('led_color_num', { num: 'K4' })" path="led_colors_3">
              <n-color-picker v-model:value="store.led_colors![3]" :show-alpha="false" :modes="['hex']" />
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
        <n-gi :span="10">
          <n-collapse-transition
            :show="lightingMode == LightingMode.BreatheGlowMode || lightingMode == LightingMode.BreatheGlowAsyncMode">
            <n-form-item :label="$t('breath_maximum_light_duration')" path="breath_maximum_light_duration">
              <n-slider v-model:value="light_cfg!.max_keep_time" :step="1" :max="1000" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition
            :show="lightingMode == LightingMode.BreatheGlowMode || lightingMode == LightingMode.BreatheGlowAsyncMode">
            <n-form-item :label="$t('breath_minimum_light_duration')" path="breath_minimum_light_duration">
              <n-slider v-model:value="light_cfg!.min_keep_time" :step="1" :max="1000" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition
            :show="lightingMode == LightingMode.BreatheGlowMode || lightingMode == LightingMode.BreatheGlowAsyncMode">
            <n-form-item :label="$t('breath_speed')" path="breath_speed">
              <n-slider v-model:value="light_cfg!.breathing_speed" :step="1" :max="20" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition
            :show="lightingMode == LightingMode.RainbowMode">
            <n-form-item :label="$t('rainbow_light_switching_speed')" path="rainbow_light_switching_speed">
              <n-slider v-model:value="light_cfg!.rainbow_speed" :step="1" :min="1" :max="30" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition
            :show="lightingMode == LightingMode.RainbowFlowMode">
            <n-form-item :label="$t('rainbow_light_switching_step')" path="rainbow_flow_speed">
              <n-slider v-model:value="light_cfg!.rainbow_flow_speed" :step="10" :min="10" :max="500" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition
            :show="lightingMode == LightingMode.RainbowFlowMode">
            <n-form-item :label="$t('color_change_rate')" path="color_change_rate">
              <n-slider v-model:value="light_cfg!.color_change_rate" :step="1" :min="1" :max="10" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition
            :show="lightingMode == LightingMode.RainbowFlowMode">
            <n-form-item :label="$t('delay_process')" path="is_flow_delay">
              <n-select v-model:value="store.is_flow_delay" :options="ToggleSel" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition :show="lightingMode == LightingMode.TapToGlowMode">
            <n-form-item :label="$t('press_light_duration')" path="press_light_duration">
              <n-slider v-model:value="light_cfg!.tap_to_glow_speed" :step="1" :min="1" :max="40" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.TapToGlowMode">
            <n-form-item :label="$t('random_color_mode')" path="random_color_mode">
              <n-select v-model:value="store.random_color_mode" :options="ToggleSel" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.TapToGlowMode">
            <n-form-item :label="$t('change_color_when_pressed')" path="change_color_when_pressed">
              <n-select v-model:value="store.random_color_mode" :options="ToggleSel" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedLightMode">
            <n-form-item :label="$t('speed_press_high_color')" path="speed_press_high_color">
              <n-color-picker v-model:value="store.high_speed_color" :show-alpha="false" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedLightMode">
            <n-form-item :label="$t('speed_press_low_color')" path="speed_press_low_color">
              <n-color-picker v-model:value="store.low_speed_color" :show-alpha="false" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedLightMode">
            <n-form-item :label="$t('speed_press_difficulty')" path="speed_press_trans_speed">
              <n-slider v-model:value="light_cfg!.increase_difficulty" :step="1" :min="1" :max="40" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.RainDropMode">
            <n-form-item :label="$t('speed')" path="rain_drop_speed">
              <n-slider v-model:value="light_cfg!.rain_drop_speed" :step="1" :min="1" :max="40" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.RainDropMode">
            <n-form-item :label="$t('random_rain_chance')" path="random_rain_chance">
              <n-slider v-model:value="light_cfg!.random_rain_chance" :step="10" :min="10" :max="1000" />
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
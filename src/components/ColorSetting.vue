<script setup lang="ts">
import { useStore } from '@/store'
import { LightingMode, KeyCode, jsToHid, IRgb } from "@/interface";
import { Rgb2Hex, Hex2Rgb, IsModifierKey, compareArray } from '@/utils';
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const store = useStore()

const canChangeColor = ref(true)
const lightingMode = ref(LightingMode.Solid)


const LighingMode = [
  {
    key: LightingMode.Solid,
    label: t('solid')
  },
  {
    key: LightingMode.RainbowFlowMode,
    label: t('rainbow_breath_switch')
  },
  {
    key: LightingMode.RainbowMode,
    label: t('rainbow_gradient_switch')
  },
  {
    key: LightingMode.PressRadianceMode,
    label: t('ya-gan-mo-shi')
  }
]



function lightingModeUpdate(type: string) {
  console.log(type)
  CanCanChangeColor()
  return
}

function CanCanChangeColor() {
  if (store.config == null) { canChangeColor.value = false; return; }

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
        <!-- <n-gi :span="20">
          <n-collapse-transition
            :show="lightingMode == LightingMode.Breath || lightingMode == LightingMode.RainbowBreathSync || lightingMode == LightingMode.RainbowBreathSwitch">
            <n-form-item :label="$t('breath_minimum_brightness')" path="breath_minimum_brightness">
              <n-slider v-model:value="store.config!.breath_minimum_brightness" :step="1" :max="100" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition
            :show="lightingMode == LightingMode.Breath || lightingMode == LightingMode.RainbowBreathSync || lightingMode == LightingMode.RainbowBreathSwitch">
            <n-form-item :label="$t('breath_maximum_light_duration')" path="breath_maximum_light_duration">
              <n-slider v-model:value="store.config!.breath_maximum_light_duration" :step="1" :max="1000" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition
            :show="lightingMode == LightingMode.Breath || lightingMode == LightingMode.RainbowBreathSync || lightingMode == LightingMode.RainbowBreathSwitch">
            <n-form-item :label="$t('breath_minimum_light_duration')" path="breath_minimum_light_duration">
              <n-slider v-model:value="store.config!.breath_minimum_light_duration" :step="1" :max="1000" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition
            :show="lightingMode == LightingMode.Breath || lightingMode == LightingMode.RainbowBreathSync || lightingMode == LightingMode.RainbowBreathSwitch">
            <n-form-item :label="$t('breath_speed')" path="breath_speed">
              <n-slider v-model:value="store.breath_speed" :step="1" :max="20" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition
            :show="lightingMode == LightingMode.RainbowGradientSync || lightingMode == LightingMode.RainbowBreathSync || lightingMode == LightingMode.RainbowGradientSwitch || lightingMode == LightingMode.RainbowBreathSwitch">
            <n-form-item :label="$t('rainbow_light_switching_speed')" path="rainbow_light_switching_speed">
              <n-slider v-model:value="store.rainbow_light_switching_speed" :step="1" :min="1" :max="30" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition :show="lightingMode == LightingMode.PressAndLight">
            <n-form-item :label="$t('press_light_minimum_brightness')" path="press_light_minimum_brightness">
              <n-slider v-model:value="store.config!.press_light_minimum_brightness" :step="1" :max="100" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition :show="lightingMode == LightingMode.PressAndLight">
            <n-form-item :label="$t('press_light_duration')" path="press_light_duration">
              <n-slider v-model:value="store.config!.press_light_duration" :step="1" :min="0" :max="10" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedPress">
            <n-form-item :label="$t('speed_press_high_color')" path="speed_press_high_color">
              <n-color-picker v-model:value="store.speed_press_high_color" :show-alpha="false" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedPress">
            <n-form-item :label="$t('speed_press_low_color')" path="speed_press_low_color">
              <n-color-picker v-model:value="store.speed_press_low_color" :show-alpha="false" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedPress">
            <n-form-item :label="$t('speed_press_trans_speed')" path="speed_press_trans_speed">
              <n-slider v-model:value="store.config!.speed_press_trans_speed" :step="1" :min="10" :max="40" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi> -->
      </n-grid>
      <n-collapse-transition :show="lightingMode == LightingMode.Off">
        <n-empty :description="$t('no_option')" size="huge" style="margin-top: 40px;"></n-empty>
      </n-collapse-transition>
    </n-gi>
  </n-grid>
</template>

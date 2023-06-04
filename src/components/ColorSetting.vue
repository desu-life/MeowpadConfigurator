<script setup lang="ts">
import { useStore } from '@/store'
import { LightingMode, KeyCode, jsToHid, IRgb, JitterEliminationMode } from "@/interface";
import { Rgb2Hex, Hex2Rgb, IsModifierKey, compareArray } from '@/utils';
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const store = useStore()

const canChangeColor = ref(true)
const lightingMode = ref(LightingMode.Solid)


const LighingMode = [
  {
    key: LightingMode.Off,
    label: t('off')
  },
  {
    key: LightingMode.Solid,
    label: t('solid')
  },
  {
    key: LightingMode.Breath,
    label: t('breath')
  },
  {
    key: LightingMode.RainbowBreathSync,
    label: t('rainbow_breath_switch')
  },
  {
    key: LightingMode.RainbowGradientSync,
    label: t('rainbow_gradient_switch')
  },
  {
    key: LightingMode.PressAndLight,
    label: t('press_and_light')
  },
  {
    key: LightingMode.SpeedPress,
    label: t('speed_press')
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
    case LightingMode.RainbowBreathSync:
      canChangeColor.value = false
      break
    case LightingMode.RainbowGradientSync:
      canChangeColor.value = false
      break
    case LightingMode.RainbowBreathSwitch:
      canChangeColor.value = false
      break
    case LightingMode.RainbowGradientSwitch:
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
            <n-form-item :label="$t('led_color_l')" path="led_color_l">
              <n-color-picker v-model:value="store.led_color_l" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item :label="$t('led_color_r')" path="led_color_r">
              <n-color-picker v-model:value="store.led_color_r" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item :label="$t('led_color_btm_l')" path="led_color_l">
              <n-color-picker v-model:value="store.led_color_btm_l" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item :label="$t('led_color_btm_r')" path="led_color_r">
              <n-color-picker v-model:value="store.led_color_btm_r" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition :show="lightingMode != LightingMode.Off">
            <n-form-item :label="$t('maximum_brightness')" path="maximum_brightness">
              <n-slider v-model:value="store.config!.maximum_brightness" :step="1" :max="100" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
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
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedPress">
            <n-form-item :label="$t('speed_press_trans_speed')" path="speed_press_trans_speed">
              <n-slider v-model:value="store.config!.speed_press_trans_speed" :step="1" :min="0" :max="20" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedPress">
            <n-form-item :label="$t('press_light_step')" path="press_light_step">
              <n-slider v-model:value="store.config!.press_light_step" :step="1" :min="20" :max="100" />
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

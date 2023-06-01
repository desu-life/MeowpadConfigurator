<script setup lang="ts">
import { useStore } from '@/store'
import { LightingMode, KeyCode, jsToHid, IRgb, JitterEliminationMode } from "@/interface";
import { Rgb2Hex, Hex2Rgb, IsModifierKey, compareArray } from '@/utils';

const store = useStore()

const canChangeColor = ref(true)
const lightingMode = ref(LightingMode.Solid)


const LighingMode = [
  {
    key: LightingMode.Off,
    label: "关闭"
  },
  {
    key: LightingMode.Solid,
    label: "常亮"
  },
  {
    key: LightingMode.Breath,
    label: '呼吸'
  },
  {
    key: LightingMode.RainbowBreathSync,
    label: '彩虹呼吸'
  },
  {
    key: LightingMode.RainbowGradientSync,
    label: '彩虹渐变'
  },
  {
    key: LightingMode.PressAndLight,
    label: '触发模式'
  },
  {
    key: LightingMode.SpeedPress,
    label: '手速计'
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
      <n-form-item label="灯效模式" path="lighting_mode">
        <n-menu v-model:value="lightingMode" :options="LighingMode" inverted :indent="18"
          @update:value="lightingModeUpdate" />
      </n-form-item>
    </n-gi>
    <n-gi :span="20">
      <n-grid :cols="20" :x-gap="20">
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item label="颜色（左灯）" path="led_color_l">
              <n-color-picker v-model:value="store.led_color_l" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item label="颜色（右灯）" path="led_color_r">
              <n-color-picker v-model:value="store.led_color_r" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item label="颜色（底部左灯）" path="led_color_l">
              <n-color-picker v-model:value="store.led_color_btm_l" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="5">
          <n-collapse-transition :show="canChangeColor">
            <n-form-item label="颜色（底部右灯）" path="led_color_r">
              <n-color-picker v-model:value="store.led_color_btm_r" :show-alpha="false" :modes="['hex']" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition :show="lightingMode != LightingMode.Off">
            <n-form-item label="最大亮度" path="maximum_brightness">
              <n-slider v-model:value="store.config!.maximum_brightness" :step="1" :max="100" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition
            :show="lightingMode == LightingMode.Breath || lightingMode == LightingMode.RainbowBreathSync || lightingMode == LightingMode.RainbowBreathSwitch">
            <n-form-item label="呼吸灯最小亮度" path="breath_minimum_brightness">
              <n-slider v-model:value="store.config!.breath_minimum_brightness" :step="1" :max="100" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition
            :show="lightingMode == LightingMode.Breath || lightingMode == LightingMode.RainbowBreathSync || lightingMode == LightingMode.RainbowBreathSwitch">
            <n-form-item label="呼吸灯维持时间（最亮点）" path="breath_maximum_light_duration">
              <n-slider v-model:value="store.config!.breath_maximum_light_duration" :step="1" :max="1000" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition
            :show="lightingMode == LightingMode.Breath || lightingMode == LightingMode.RainbowBreathSync || lightingMode == LightingMode.RainbowBreathSwitch">
            <n-form-item label="呼吸灯维持时间（最暗点）" path="breath_minimum_light_duration">
              <n-slider v-model:value="store.config!.breath_minimum_light_duration" :step="1" :max="1000" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition
            :show="lightingMode == LightingMode.Breath || lightingMode == LightingMode.RainbowBreathSync || lightingMode == LightingMode.RainbowBreathSwitch">
            <n-form-item label="呼吸灯速度" path="breath_speed">
              <n-slider v-model:value="store.breath_speed" :step="1" :max="20" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition
            :show="lightingMode == LightingMode.RainbowGradientSync || lightingMode == LightingMode.RainbowBreathSync || lightingMode == LightingMode.RainbowGradientSwitch || lightingMode == LightingMode.RainbowBreathSwitch">
            <n-form-item label="彩虹渐变速度" path="rainbow_light_switching_speed">
              <n-slider v-model:value="store.rainbow_light_switching_speed" :step="1" :min="1" :max="30" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition :show="lightingMode == LightingMode.PressAndLight">
            <n-form-item label="最小亮度" path="press_light_minimum_brightness">
              <n-slider v-model:value="store.config!.press_light_minimum_brightness" :step="1" :max="100" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="20">
          <n-collapse-transition :show="lightingMode == LightingMode.PressAndLight">
            <n-form-item label="亮度衰减时长" path="press_light_duration">
              <n-slider v-model:value="store.config!.press_light_duration" :step="1" :min="0" :max="10" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedPress">
            <n-form-item label="高速状态颜色" path="speed_press_high_color">
              <n-color-picker v-model:value="store.speed_press_high_color" :show-alpha="false" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedPress">
            <n-form-item label="低速状态颜色" path="speed_press_low_color">
              <n-color-picker v-model:value="store.speed_press_low_color" :show-alpha="false" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedPress">
            <n-form-item label="颜色切换速度" path="speed_press_trans_speed">
              <n-slider v-model:value="store.config!.speed_press_trans_speed" :step="1" :min="0" :max="20" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
        <n-gi :span="10">
          <n-collapse-transition :show="lightingMode == LightingMode.SpeedPress">
            <n-form-item label="颜色变速步长" path="press_light_step">
              <n-slider v-model:value="store.config!.press_light_step" :step="1" :min="20" :max="100" />
            </n-form-item>
          </n-collapse-transition>
        </n-gi>
      </n-grid>
      <n-collapse-transition :show="lightingMode == LightingMode.Off">
        <n-empty description="无选项" size="huge" style="margin-top: 40px;"></n-empty>
      </n-collapse-transition>
    </n-gi>
  </n-grid>
</template>

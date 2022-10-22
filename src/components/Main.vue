<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { FormInst } from 'naive-ui'
import { useStore } from '@/store'
import { Keyboard24Regular, Lightbulb24Regular } from '@vicons/fluent'
import { LightingMode, KeyCode, jsToHid, IRgb, JitterEliminationMode } from "@/interface";
import { Rgb2Hex, Hex2Rgb } from '@/utils';

// defineProps<{ msg: string }>()

const store = useStore()
const formRef = ref<FormInst | null>(null)
const message = useMessage()
const configType = ref(0)
const canChangeColor = ref(true)
const showModal = ref(false)
const showDrawer = ref(true)
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
  },
]

const JEModeSel = [
  {
    value: JitterEliminationMode.Lazy,
    label: "激进模式（先发送）"
  },
  {
    value: JitterEliminationMode.Active,
    label: "稳定模式（先消抖）"
  }
]

const LighingModeSel = [
  {
    value: LightingMode.Off,
    label: "关闭"
  },
  {
    value: LightingMode.Solid,
    label: "常亮"
  },
  {
    value: LightingMode.Breath,
    label: '呼吸'
  },
  {
    value: LightingMode.SpeedPress,
    label: '手速计'
  },
  {
    value: LightingMode.RainbowBreathSwitch,
    label: '彩虹呼吸'
  },
  {
    value: LightingMode.RainbowGradientSwitch,
    label: '彩虹渐变'
  },
  {
    value: LightingMode.RainbowBreathSync,
    label: '彩虹呼吸（同步）'
  },
  {
    value: LightingMode.RainbowGradientSync,
    label: '彩虹渐变（同步）'
  },
  {
    value: LightingMode.PressAndLight,
    label: '触发模式'
  },
]

function setKeys(keyNum: number) {
  showModal.value = true
  document.onkeydown = (_e) => {
    if (showModal.value === false) { return }
    _e.preventDefault()
    const e = _e || window.event || arguments.callee.caller.arguments[0]
    console.log(e)
    const hidCode: KeyCode = jsToHid[e.code] || undefined
    if (hidCode in KeyCode) {
      switch (keyNum) {
        case 1:
          if (store.config!.key_1 === hidCode) {
            store.config!.key_1 = KeyCode.NONE
          } else {
            store.config!.key_1 = hidCode
          }
          break
        case 2:
          if (store.config!.key_2 === hidCode) {
            store.config!.key_2 = KeyCode.NONE
          } else {
            store.config!.key_2 = hidCode
          }
          break
        case 3:
          if (store.config!.key_3 === hidCode) {
            store.config!.key_3 = KeyCode.NONE
          } else {
            store.config!.key_3 = hidCode
          }
          break
        case 4:
          if (store.config!.key_4 === hidCode) {
            store.config!.key_4 = KeyCode.NONE
          } else {
            store.config!.key_4 = hidCode
          }
          break
        case 5:
          if (store.config!.key_5 === hidCode) {
            store.config!.key_5 = KeyCode.NONE
          } else {
            store.config!.key_5 = hidCode
          }
          break
        default:
          break
      }
    } else {
      message.error('未知按键：' + e.code)
    }
    document.onkeydown = null
    showModal.value = false
  }
}

async function handleValidateClick(e: MouseEvent) {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      message.success('Valid')
    } else {
      console.log(errors)
      message.error('Invalid')
    }
  })
}

function switchConfig() {
  if (configType.value === 1) {
    configType.value = 0
  } else {
    configType.value = 1
  }
}

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

function formatms(value: number | null) {
  if (value === null) return ''
  return `${value} ms`
}

function parsems(value: string) {
  if (value === '') return null
  return parseInt(value)
}
</script>

<template>
  <!-- <n-drawer v-model:show="showDrawer" placement="left" to=".main">
  </n-drawer> -->
  <n-modal v-model:show="showModal" transform-origin="center" :close-on-esc="false">
    <n-card style="width: fit-content;border-radius: 8px;" :bordered="false" :title='"在键盘上按下要设置的键\n(若未改动则重置该键)"'
      role="dialog" aria-modal="true">
    </n-card>
  </n-modal>
  <n-spin :show="store.loading">
    <n-form class="h-96" ref="formRef" :label-width="80" label-placement="top" :model="store.config" size="medium"
      style="margin-bottom: 40px;" v-if="store.config != undefined">
      <transition mode="out-in" enter-active-class="animate__animated animate__fadeIn animate__slower"
        leave-active-class="animate__animated animate__fadeOut" style="animation-duration: 0.2s;">
        <div v-if="configType !== 1">
          <n-button-group>
            <n-button size="large" @click="setKeys(1)">
              {{ KeyCode[store.config!.key_1] }}
            </n-button>
            <n-button size="large" @click="setKeys(2)">
              {{ KeyCode[store.config!.key_2] }}
            </n-button>
            <n-button size="large" @click="setKeys(3)">
              {{ KeyCode[store.config!.key_3] }}
            </n-button>
            <n-button size="large" @click="setKeys(4)">
              {{ KeyCode[store.config!.key_4] }}
            </n-button>
            <n-button size="large" @click="setKeys(5)">
              {{ KeyCode[store.config!.key_5] }}
            </n-button>
          </n-button-group>
          <n-grid :cols="20" :x-gap="18" style="margin: 50px;">
            <n-gi :span="10">
              <n-form-item label="灯效模式（按键）" path="lighting_mode_key">
                <n-select v-model:value="store.config!.lighting_mode_key" :options="LighingModeSel" />
              </n-form-item>
            </n-gi>
            <n-gi :span="10">
              <n-form-item label="灯效模式（底部）" path="lighting_mode_btm">
                <n-select v-model:value="store.config!.lighting_mode_btm" :options="LighingModeSel" />
              </n-form-item>
            </n-gi>
            <n-gi :span="10">
              <n-form-item label="消抖模式" path="keyboard_jitters_elimination_mode">
                <n-select v-model:value="store.config!.keyboard_jitters_elimination_mode" :options="JEModeSel" />
              </n-form-item>
            </n-gi>
            <n-gi :span="10">
              <n-form-item label="消抖时长" path="keyboard_jitters_elimination_time">
                <n-input-number v-model:value="store.config!.keyboard_jitters_elimination_time" />
              </n-form-item>
            </n-gi>
          </n-grid>
        </div>


        <div v-else>
          <n-grid :cols="24" :x-gap="18">
            <n-gi :span="4" class="border-4 border-indigo-200 border-r-indigo-500">
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
                    <n-form-item label="即按即亮最小亮度" path="press_light_minimum_brightness">
                      <n-slider v-model:value="store.config!.press_light_minimum_brightness" :step="1" :max="100" />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="20">
                  <n-collapse-transition :show="lightingMode == LightingMode.PressAndLight">
                    <n-form-item label="即按即亮衰减时长" path="press_light_duration">
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
                    <n-form-item label="手速灯变速步长" path="press_light_step">
                      <n-slider v-model:value="store.config!.press_light_step" :step="1" :min="1" :max="100" />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
              </n-grid>
              <n-collapse-transition :show="lightingMode == LightingMode.Off">
                <n-empty description="无选项" size="huge" style="margin-top: 40px;"></n-empty>
              </n-collapse-transition>
            </n-gi>
          </n-grid>

        </div>
      </transition>
      <div style="position: fixed;
        z-index: 10;
        bottom: 40px;
        right: calc(40px);">
        <n-tooltip trigger="hover" :delay="400" :duration="200">
          <template #trigger>
            <n-button round type="warning" @click="switchConfig">
              <template #icon>
                <n-icon>
                  <transition mode="out-in" enter-active-class="animate__animated animate__fadeIn animate__slower"
                    leave-active-class="animate__animated animate__fadeOut" style="animation-duration: 0.15s;">
                    <Keyboard24Regular v-if="configType === 1" />
                    <Lightbulb24Regular v-else />
                  </transition>
                </n-icon>
              </template>
              <transition mode="out-in" enter-active-class="animate__animated animate__fadeIn animate__slower"
                leave-active-class="animate__animated animate__fadeOut" style="animation-duration: 0.15s;">
                <span v-if="configType === 1">返回</span>
                <span v-else>灯效配置</span>
              </transition>
            </n-button>
          </template>
          切换页面
        </n-tooltip>
      </div>
    </n-form>
    <n-empty description="请先连接设备" size="huge" v-else>
    </n-empty>
  </n-spin>
</template>

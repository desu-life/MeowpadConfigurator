<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { FormInst } from 'naive-ui'
import { useStore } from '@/store'
import { Keyboard24Regular, Lightbulb24Regular } from '@vicons/fluent'
import { LightingMode, KeyCode, jsToHid } from "@/interface";
// defineProps<{ msg: string }>()

const store = useStore()
const formRef = ref<FormInst | null>(null)
const message = useMessage()
const configType = ref(0)
const canChangeColor = ref(true)
const PressAndLightValue = ref([0, 255])
const LightMinMaxValue = ref([0, 255])
const showModal = ref(false)

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
    key: LightingMode.RainbowBreath,
    label: '彩虹呼吸'
  },
  {
    key: LightingMode.RainbowGradient,
    label: '彩虹渐变'
  },
  {
    key: LightingMode.PressAndLight,
    label: '即按即亮'
  },
  {
    key: LightingMode.SpeedPress,
    label: '手速模式'
  }
]

const rules = {
  user: {
    name: {
      required: true,
      message: '请输入姓名',
      trigger: 'blur'
    },
    age: {
      required: true,
      message: '请输入年龄',
      trigger: ['input', 'blur']
    }
  },
  phone: {
    required: true,
    message: '请输入电话号码',
    trigger: ['input']
  }
}

function setKeys(keyNum: number) {
  showModal.value = true
  document.onkeydown = (_e) => {
    _e.preventDefault()
    const e = _e || window.event || arguments.callee.caller.arguments[0]
    console.log(e)
    const hidCode: KeyCode = jsToHid[e.code] || undefined
    if (hidCode in KeyCode) {
      switch (keyNum) {
        case 1:
          store.config!.key_1 = hidCode
          break
        case 2:
          store.config!.key_2 = hidCode
          break
        case 3:
          store.config!.key_3 = hidCode
          break
        case 4:
          store.config!.key_4 = hidCode
          break
        case 5:
          store.config!.key_5 = hidCode
          break
        default:
          break
      }
    } else {
      message.error('未知按键')
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

  switch (store.config.lighting_mode) {
    case null:
      canChangeColor.value = false
      break
    case LightingMode.Off:
      canChangeColor.value = false
      break
    case LightingMode.RainbowBreath:
      canChangeColor.value = false
      break
    case LightingMode.RainbowGradient:
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
  <n-modal v-model:show="showModal" transform-origin="center" :close-on-esc="false">
    <n-card style="width: fit-content" :bordered="false" title="请在键盘上按下要设定的键" size="huge" role="dialog"
      aria-modal="true">
    </n-card>
  </n-modal>
  <n-spin :show="store.loading">
    <n-form class="m-5 h-96" ref="formRef" :label-width="80" label-placement="top" :model="store.config" :rules="rules"
      size="medium" v-if="store.config != undefined">
      <transition mode="out-in" enter-active-class="animate__animated animate__fadeIn animate__slower"
        leave-active-class="animate__animated animate__fadeOut" style="animation-duration: 0.2s;">
        <n-button-group v-if="configType !== 1">
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

        <div v-else>
          <n-grid :cols="24" :x-gap="18">
            <n-gi :span="4" class="border-4 border-indigo-200 border-r-indigo-500">
              <n-form-item label="灯效模式" path="lighting_mode">
                <n-menu v-model:value="store.config!.lighting_mode" :options="LighingMode" inverted :indent="18"
                  @update:value="lightingModeUpdate" />
              </n-form-item>
            </n-gi>
            <n-gi :span="20">
              <n-grid :cols="20" :x-gap="20">
                <n-gi :span="10">
                  <n-collapse-transition :show="canChangeColor">
                    <n-form-item label="颜色（左灯）" path="color">
                      <n-color-picker v-model:value="store.config!.led_color_l" :show-alpha="false" />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="10">
                  <n-collapse-transition :show="canChangeColor">
                    <n-form-item label="颜色（右灯）" path="color">
                      <n-color-picker v-model:value="store.config!.led_color_r" :show-alpha="false" />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="20">
                  <n-collapse-transition :show="store.config!.lighting_mode == LightingMode.Solid">
                    <n-form-item label="最大亮度" path="max&min">
                      <n-slider v-model:value="store.config!.maximum_brightness" :tooltip="false" :step="1"
                        :max="255" />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="20">
                  <n-collapse-transition
                    :show="store.config!.lighting_mode == LightingMode.Breath || store.config!.lighting_mode == LightingMode.RainbowBreath">
                    <n-form-item label="最大/最小亮度" path="max&min">
                      <n-slider v-model:value="LightMinMaxValue" range :tooltip="false" :step="1" :max="255" />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="20">
                  <n-collapse-transition
                    :show="store.config!.lighting_mode == LightingMode.RainbowGradient || store.config!.lighting_mode == LightingMode.RainbowBreath">
                    <n-form-item label="渐变灯切换速度" path="u8">
                      <n-slider v-model:value="store.config!.fade_light_switching_speed" :tooltip="false" :step="10"
                        :min="10" :max="2550" reverse />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="20">
                  <n-collapse-transition :show="store.config!.lighting_mode == LightingMode.PressAndLight">
                    <n-form-item label="最大/最小亮度" path="max&min">
                      <n-slider v-model:value="PressAndLightValue" range :tooltip="false" :step="1" :max="255" />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="20">
                  <n-collapse-transition :show="store.config!.lighting_mode == LightingMode.PressAndLight">
                    <n-form-item label="即按即亮衰减速度" path="u64">
                      <n-slider v-model:value="store.config!.press_light_duration" :tooltip="false" :step="10" :min="10"
                        :max="2550" reverse />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="10">
                  <n-collapse-transition :show="store.config!.lighting_mode == LightingMode.SpeedPress">
                    <n-form-item label="高速状态颜色（左灯）" path="color">
                      <n-color-picker v-model:value="store.config!.speed_press_color_l" :show-alpha="false" />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="10">
                  <n-collapse-transition :show="store.config!.lighting_mode == LightingMode.SpeedPress">
                    <n-form-item label="高速状态颜色（右灯）" path="color">
                      <n-color-picker v-model:value="store.config!.speed_press_color_r" :show-alpha="false" />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="10">
                  <n-collapse-transition :show="store.config!.lighting_mode == LightingMode.SpeedPress">
                    <n-form-item label="颜色切换速度（越大越满）" path="u8">
                      <n-slider v-model:value="store.config!.color_switching_speed" :tooltip="false" :step="1" :min="1"
                        :max="100" reverse />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
                <n-gi :span="10">
                  <n-collapse-transition :show="store.config!.lighting_mode == LightingMode.SpeedPress">
                    <n-form-item label="手速灯变速步长" path="u8">
                      <n-slider v-model:value="store.config!.press_light_step" :tooltip="false" :step="1" :min="1"
                        :max="255" />
                    </n-form-item>
                  </n-collapse-transition>
                </n-gi>
              </n-grid>
              <n-collapse-transition :show="store.config!.lighting_mode == LightingMode.Off">
                <n-empty description="无选项" size="huge"></n-empty>
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
                <span v-if="configType === 1">按键</span>
                <span v-else>灯光</span>
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

<script setup lang="ts">
import { JitterEliminationMode, KeyCode, LightingMode, jsToHid } from '@/interface';
import { useStore } from '@/store'
import { formatKeys, IsModifierKey, compareArray } from '@/utils'
import KeyModal from '@/components/KeyModal.vue'
import { invoke } from "@tauri-apps/api/tauri";
import { h } from 'vue'
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const store = useStore()
const message = useMessage()
const dialog = useDialog()

const showModal = ref(false)
const pressedkeycodes = ref<KeyCode[]>([])
const presskeycodes = ref<KeyCode[]>([])

async function set_auto_config(): Promise<void> {
  const res: any = await invoke("get_auto_config", {})
  console.log(res)
  dialog.warning({
    title: t('warning'),
    content: () => t('recommend_config_help'),
    positiveText: t('yes'),
    negativeText: t('no'),
    onPositiveClick: () => {
      store.config!.dead_zone = res["DeadZone"]
      store.config!.key_release_degree = res["KeyReleaseDegree"]
      store.config!.key_trigger_degree = res["KeyTriggerDegree"]
      message.success(t('applied_recommend_config'))
    },
    onNegativeClick: () => {

    }
  })
}


const JEModeSel = [
  {
    value: JitterEliminationMode.Lazy,
    label: t('lazy_mode')
  },
  {
    value: JitterEliminationMode.Active,
    label: t('active_mode')
  }
]


const keyscanlist = [
  {
    value: 0,
    label: t('nolimit')
  },
  {
    value: 1,
    label: "1000hz"
  },
  {
    value: 2,
    label: "500hz"
  },
  {
    value: 3,
    label: "333hz"
  },
  {
    value: 4,
    label: "250hz"
  },
  {
    value: 5,
    label: "200hz"
  },
  {
    value: 6,
    label: "166hz"
  },
  {
    value: 7,
    label: "125hz"
  },
  {
    value: 8,
    label: "100hz"
  },
  {
    value: 9,
    label: "90hz"
  },
  {
    value: 10,
    label: "76hz"
  },
  {
    value: 11,
    label: "62hz"
  }
]


const LighingModeSel = [
  {
    value: LightingMode.Off,
    label: t('off')
  },
  {
    value: LightingMode.Solid,
    label: t('solid')
  },
  {
    value: LightingMode.Breath,
    label: t('breath')
  },
  {
    value: LightingMode.SpeedPress,
    label: t('speed_press')
  },
  {
    value: LightingMode.RainbowBreathSwitch,
    label: t('rainbow_breath_switch')
  },
  {
    value: LightingMode.RainbowGradientSwitch,
    label: t('rainbow_gradient_switch')
  },
  {
    value: LightingMode.RainbowBreathSync,
    label: t('rainbow_breath_sync')
  },
  {
    value: LightingMode.RainbowGradientSync,
    label: t('rainbow_gradient_sync')
  },
  {
    value: LightingMode.PressAndLight,
    label: t('press_and_light')
  }
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
    value: LightingMode.Breath,
    label: t('breath')
  },
  {
    value: LightingMode.SpeedPress,
    label: t('speed_press')
  },
  {
    value: LightingMode.RainbowBreathSwitch,
    label: t('rainbow_breath_switch')
  },
  {
    value: LightingMode.RainbowGradientSwitch,
    label: t('rainbow_gradient_switch')
  },
  {
    value: LightingMode.RainbowBreathSync,
    label: t('rainbow_breath_sync')
  },
  {
    value: LightingMode.RainbowGradientSync,
    label: t('rainbow_gradient_sync')
  },
  {
    value: LightingMode.PressAndLight,
    label: t('press_and_light')
  },
  {
    value: LightingMode.根据按压力度决定LED发光程度,
    label: t('ya-gan-mo-shi')
  },
]

function GetLighingModeSel() {
  if (store.is_hs) {
    return LighingModeSelWooting
  } else {
    return LighingModeSel
  }
}



function needkey(key: KeyCode) {
  if (!presskeycodes.value.includes(key)) {
    if (IsModifierKey(key)) {
      return true
    } else {
      if (presskeycodes.value.filter((k) => !IsModifierKey(k)).length < 3) {
        return true
      } else {
        message.error(t('most_3_key_error'))
      }
    }
  }
  return false
}

function clearset() {
  pressedkeycodes.value = []
  presskeycodes.value = []
  document.onkeydown = null
  document.onkeyup = null
}

function setKeys(keyNum: number) {
  pressedkeycodes.value = []
  showModal.value = true
  let keycodes: KeyCode[] = []
  document.onkeydown = (_e) => {
    if (showModal.value === false) { return }
    _e.preventDefault()
    const e = _e || window.event || arguments.callee.caller.arguments[0]
    const HidCodeDown: KeyCode = jsToHid[e.code] || undefined
    if (HidCodeDown in KeyCode && needkey(HidCodeDown)) {
      console.log("pressed " + e.code)
      pressedkeycodes.value.push(HidCodeDown)
      presskeycodes.value.push(HidCodeDown)
      document.onkeyup = (_e) => {
        _e.preventDefault()
        document.onkeydown = null // 当有键松开时，清除按下键盘的监听函数
        const e = _e || window.event || arguments.callee.caller.arguments[0]
        console.log("released " + e.code)
        const HidCodeUP: KeyCode = jsToHid[e.code] || undefined
        if (presskeycodes.value.includes(HidCodeUP)) {
          presskeycodes.value.splice(presskeycodes.value.indexOf(HidCodeUP), 1)
          keycodes.push(HidCodeUP)
          if (presskeycodes.value.length === 0) {
            keycodes = keycodes.sort((l, r) => r - l)
            switch (keyNum) {
              case 1:
                compareArray(store.config!.key_1, keycodes) ? store.config!.key_1 = [] : store.config!.key_1 = keycodes
                break
              case 2:
                compareArray(store.config!.key_2, keycodes) ? store.config!.key_2 = [] : store.config!.key_2 = keycodes
                break
              case 3:
                compareArray(store.config!.key_3, keycodes) ? store.config!.key_3 = [] : store.config!.key_3 = keycodes
                break
              case 4:
                compareArray(store.config!.key_4, keycodes) ? store.config!.key_4 = [] : store.config!.key_4 = keycodes
                break
              case 5:
                compareArray(store.config!.key_5, keycodes) ? store.config!.key_5 = [] : store.config!.key_5 = keycodes
                break
              default:
                break
            }
            showModal.value = false
            document.onkeyup = null
          }
        }
      }
    } else {
      // message.error('未知按键：' + e.code)
    }
  }
}



</script>

<template>
  <KeyModal v-model:show="showModal" :pressedkeycodes="pressedkeycodes" :leave-func="clearset"></KeyModal>
  <!-- <n-space justify="space-between"> -->
  <n-grid x-gap="12" :cols="2" style="width: 80vw;padding-bottom: 40px;">
    <n-gi>

      <div style="padding-top: 10px;">
        <n-button-group style="padding-bottom: 5px;">
          <n-button size="large" @click="setKeys(1)" :disabled="showModal">
            {{ formatKeys(store.config!.key_1) }}
          </n-button>
          <n-button size="large" @click="setKeys(2)" :disabled="showModal">
            {{ formatKeys(store.config!.key_2) }}
          </n-button>
        </n-button-group>
        <br>
        <n-button-group>
          <n-button @click="setKeys(3)" :disabled="showModal">
            {{ formatKeys(store.config!.key_3) }}
          </n-button>
          <n-button @click="setKeys(4)" :disabled="showModal">
            {{ formatKeys(store.config!.key_4) }}
          </n-button>
          <n-button @click="setKeys(5)" :disabled="showModal">
            {{ formatKeys(store.config!.key_5) }}
          </n-button>
        </n-button-group>
      </div>
    </n-gi>
    <n-gi>



      <div style="max-width: 400px;min-width: 200px;">
        <n-button type="error" class="badge" @click="set_auto_config">
          {{ $t('need_help') }} </n-button>
        <n-form-item :label="$t('dead_zone')" path="dead_zone" label-placement="left" :show-feedback="false" v-if="store.is_hs">
          <n-input-number v-model:value="store.config!.dead_zone" :min="1" :max="100" :placeholder="$t('no_data')">
            <template #suffix>
              %
            </template>
          </n-input-number>
        </n-form-item>
        <n-form-item :label="$t('key_trigger_degree')" path="key_trigger_degree" label-placement="left" :show-feedback="false"
          v-if="store.is_hs">
          <n-input-number v-model:value="store.config!.key_trigger_degree" :min="1" :max="100" :placeholder="$t('no_data')">
            <template #suffix>
              %
            </template>
          </n-input-number>
        </n-form-item>
        <n-form-item :label="$t('key_release_degree')" path="key_release_degree" label-placement="left" :show-feedback="false"
          v-if="store.is_hs">
          <n-input-number v-model:value="store.config!.key_release_degree" :min="1" :max="100" :placeholder="$t('no_data')">
            <template #suffix>
              %
            </template>
          </n-input-number>

        </n-form-item>
      </div>
    </n-gi>
  </n-grid>




  <!-- </n-space> -->
  <n-grid :cols="20" :x-gap="18" style="width: 80vw;">

    <n-gi :span="10">
      <n-form-item :label="$t('keyboard_jitters_elimination_mode')" path="keyboard_jitters_elimination_mode">
        <n-select v-model:value="store.config!.keyboard_jitters_elimination_mode" :options="JEModeSel" />
      </n-form-item>
    </n-gi>
    <n-gi :span="5">
      <n-form-item :label="$t('keyboard_jitters_elimination_time')" path="keyboard_jitters_elimination_time">
        <n-input-number v-model:value="store.config!.keyboard_jitters_elimination_time" :min="1"
          :max="10000"></n-input-number>
      </n-form-item>
    </n-gi>
    <n-gi :span="5">
      <n-form-item :label="$t('key_scan_rate')" path="key_scan_rate">
        <n-select v-model:value="store.config!.key_scan_rate" :options="keyscanlist" />
      </n-form-item>
    </n-gi>
    <n-gi :span="10">
      <n-form-item :label="$t('lighting_mode_key')" path="lighting_mode_key">
        <n-select v-model:value="store.config!.lighting_mode_key" :options="GetLighingModeSel()" />
      </n-form-item>
    </n-gi>
    <n-gi :span="10">
      <n-form-item :label="$t('lighting_mode_btm')" path="lighting_mode_btm">
        <n-select v-model:value="store.config!.lighting_mode_btm" :options="GetLighingModeSel()" />
      </n-form-item>
    </n-gi>
    <n-gi :span="4">
      <n-form-item :label="$t('device_sleep_idle_time')" path="device_sleep_idle_time">
        <n-input-number v-model:value="store.config!.device_sleep_idle_time" :min="1" :max="65535">
          <template #suffix>
            {{ $t('sec') }} 
          </template>
        </n-input-number>
      </n-form-item>
    </n-gi>
    <n-gi :span="8" v-if="store.config!.device_sleep_idle_time != 0">
      <n-form-item :label="$t('sleep_lighting_mode_key')" path="sleep_lighting_mode_key">
        <n-select v-model:value="store.config!.sleep_lighting_mode_key" :options="GetLighingModeSel()" />
      </n-form-item>
    </n-gi>
    <n-gi :span="8" v-if="store.config!.device_sleep_idle_time != 0">
      <n-form-item :label="$t('sleep_lighting_mode_btm')" path="sleep_lighting_mode_btm">
        <n-select v-model:value="store.config!.sleep_lighting_mode_btm" :options="GetLighingModeSel()" />
      </n-form-item>
    </n-gi>
    <n-gi :span="5">
      <n-form-item :label="$t('force_key_switch')" path="force_key_switch" :show-feedback="false">
        <n-switch v-model:value="store.config!.force_key_switch" />
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
</style>

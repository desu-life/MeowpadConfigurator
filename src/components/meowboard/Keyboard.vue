<script setup lang="ts">
/*
 * KeyCalibrate和KeyDebug两个组件的ref存在了两个列表里，可能操作到的变量我expose出来了已经
 *
 * 调试模式下基本就只是显示信息，xe说要一个显示百分比的功能，我写进去了
 * 传hall值的时候除了具体的hall值还要传一个百分比
 *
 * 校准模式的多选做了，space键太大，选中缩放0.9动画幅度有点大
 *
 * 校准模式的时候，需要你给每个键传入isCalibrated，就是否校准，校准了的按键背景色会是绿色，没校准的没颜色
 * 然后选中了之后，要开始校准，就给要校准的键传一个isCalibrating=true，待校准按键就会背景没颜色，但是显示一个黄色的边框
 * 校准完了再把isCalibrating=false传进去，别忘了isCalibrated=true也要传进去
 *
 * 还有个问题是配色，已经校准的按键的背景色是绿的，但是选中按键显示的图标也是绿的，你看看能不能改个好看的配色
 */

import KeyFrame from "@/components/meowboard/Keyboard/KeyFrame.vue";
import KeyDebug from "@/components/meowboard/Keyboard/KeyDebug.vue";
import KeyCalibrate from "@/components/meowboard/Keyboard/KeyCalibrate.vue";
import KeyHall from "@/components/meowboard/Keyboard/KeyHall.vue";
import KeyModify from "@/components/meowboard/Keyboard/KeyModify.vue";
import KeyModifyOption from "@/components/meowboard/Keyboard/KeyModifyOption.vue";
import { ComponentPublicInstance, createVNode } from "vue";

import * as apib from '@/apis/meowboard/api'
import { IError, KeyState } from "@/apis";
import { useDeviceStore } from '@/store/device';
import { KeyCode, mapping } from "@/keycode";
import { useStore } from "@/store/main";
import { IKeyboard, IKeyConfigBoard, IMixedKey } from "@/apis/meowboard/config";
import emitter from "@/mitt";
import { getErrorMsg, most, splitArray } from "@/utils";
import { useI18n } from "vue-i18n";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { useKeyboard } from "@/store/keyboard";
import { Toggle } from "@/interface";
import { storeToRefs } from "pinia";
import ModifyKeys from "./ModifyKeys.vue";
import ConfigKeys from "./ConfigKeys.vue";
import { MenuGroupOption, MenuOption, NIcon } from "naive-ui";
import { MenuMixedOption } from "naive-ui/es/menu/src/interface";

const message = useMessage()
const dialog = useDialog()
const { t } = useI18n();
const store = useStore()
const kb = useKeyboard()
const device = useDeviceStore()

const keyconfig = ref<IKeyConfigBoard | null>(null)

let default_height = ref(55)
let default_width = ref(55)
let default_margin = ref(2)
let default_font_size = ref(15)

let keymapStyle = ref({
  "--default-key-height": default_height.value + "px",
  "--default-key-width": default_width.value + "px",
  "--default-key-margin": default_margin.value + "px",
  "--default-key-font-size": default_font_size.value + "px",
})

const DebugSel = [
  {
    value: 0,
    label: "百分比模式"
  },
  {
    value: 1,
    label: "行程模式"
  },
]

const keyLayer = ref(0);
const keyShowMode = ref(0)
const totalDistance = ref(4.0)
const KeysOfEachLine = [14, 14, 13, 14, 9];
const keyIndexesRaw = computed(() => splitArray(
  Array.from({ length: 64 }, (v, i) => i),
  KeysOfEachLine
))
const keyIndexes = ref(keyIndexesRaw.value)
const keyWidth = ref([  // 每个键的ui里的宽度
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
  1.5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1.5,
  1.75, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2.25,
  2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
  1.25, 1.25, 1.25, 6.25, 1, 1, 1, 1, 1
])

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions: MenuMixedOption[] = [
  {
    type: 'group',
    key: 'keyboard',
    label: '键盘',
    children: [
      {
        label: '键盘设置',
        key: 0,
      },
      {
        label: '按键绑定',
        key: 1,
      },
      {
        label: '按键校准',
        key: 2,
      },
      {
        label: '设备调试',
        key: 3,
      },
    ]
  }
]

// 0-校准 1-调试
function selectKeyCalibrate() {
  let indexs: number[] = []
  for (let i = 0; i < 64; i++) {
    if (kb.keyVarsRefs[i].isSelected) {
      indexs.push(i)
    }
  }
  console.log(indexs)
  if (indexs.length == 0) return

  try {
    apib.calibration_key(indexs)
  } catch (e) {
    emitter.emit('connection-broke', {e: e as IError})
  }

  for (let i = 0; i < indexs.length; i++) {
    kb.keyVarsRefs[indexs[i]].isSelected = false;
    kb.keyCalibrateRefs[indexs[i]].isCalibrating = true;
    kb.keyCalibrateRefs[indexs[i]].isCalibrated = false;
  }

  const interval = setInterval(async () => {
    try {
      if (device.connected == false) {
        return
      }

      let success_count = 0;

      let cali_status = await apib.get_key_calibrate_status()
      for (let i = 0; i < indexs.length; i++) {
        if (cali_status[indexs[i]]) {
          kb.keyCalibrateRefs[indexs[i]].isCalibrated = true
          kb.keyCalibrateRefs[indexs[i]].isCalibrating = false
          success_count += 1
        }
      }

      if (success_count == indexs.length) {
        clearInterval(interval)
      }
    } catch (e) {
      clearInterval(interval)
      emitter.emit('connection-broke', {e: e as IError})
    }
  }, 100)
}

onMounted(async () => {
  keyLayer.value = 0;
  setLayer(keyLayer.value)

  const interval = setInterval(async () => {
    try {
      if (device.connected == false) {
        return
      }

      await kb.updateKey()
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
    }
  }, 100)

  onUnmounted(() => {
    clearInterval(interval)
  })
})



const currentDraggedKey = ref<IMixedKey | null>(null);
const currentOnKey = ref<number>(-1);


function startDragKey(key: IMixedKey) {
  currentDraggedKey.value = key
}

function endDragKey(key: IMixedKey) {
  if (currentOnKey.value !== -1 && currentDraggedKey.value !== null) {
    KeyStrModify({index: currentOnKey.value, currentkey: currentDraggedKey.value})
  }
  currentDraggedKey.value = null
}

emitter.on('key-str-modify', KeyStrModify)
function KeyStrModify({index, currentkey}: {index: number, currentkey: IMixedKey | null}) {
  if (kb.mode != 1) return
  
  let key: IMixedKey = currentkey ?? { t: "None", c: 0 }
  kb.showkeys[index] = key

  if (keyLayer.value == 0) {
    device.device_config!.normal_layer[index] = key
  } else if (keyLayer.value == 1) {
    device.device_config!.fn_layer[index] = key
  }
}

function onLayerUpdate() {
  if (kb.mode == 1) {
    setLayer(keyLayer.value)
  } else {
    setLayer(0)
  }
}

function onModeChange(mode) {
  kb.mode = mode
  onLayerUpdate()
}
function onLayerChange() {
  if (keyLayer.value == 0) {
    keyLayer.value = 1
  } else {
    keyLayer.value = 0
  }

  onLayerUpdate()
}

function setLayer(layer) {
  if (layer == 0) {
    for (let i = 0; i < 64; i++) {
      kb.showkeys[i] = device.device_config!.normal_layer[i]
    }
  } else {
    for (let i = 0; i < 64; i++) {
      kb.showkeys[i] = device.device_config!.fn_layer[i]
    }
  }
}



let select_indexs = ref<number[]>([])
emitter.on('key-select', () => {
  select_indexs.value = []
  for (let i = 0; i < 64; i++) {
    if (kb.keyVarsRefs[i].isSelected) {
      select_indexs.value.push(i)
    }
  }
  if (select_indexs.value.length == 0) {
    keyconfig.value = null
  } else {
    let press_percentages: number[] = []
    let release_percentages: number[] = []
    let dead_zones: number[] = []
    let r_dead_zones: number[] = []
    for (let i = 0; i < select_indexs.value.length; i++) {
      press_percentages.push(device.device_config!.keys[select_indexs.value[i]].press_percentage)
      release_percentages.push(device.device_config!.keys[select_indexs.value[i]].release_percentage)
      dead_zones.push(device.device_config!.keys[select_indexs.value[i]].dead_zone)
      r_dead_zones.push(device.device_config!.keys[select_indexs.value[i]].release_dead_zone)
    }

    // todo: 另外两个属性
    keyconfig.value = {
      press_percentage: most(press_percentages),
      release_percentage: most(release_percentages),
      dead_zone: most(dead_zones),
      release_dead_zone: most(r_dead_zones),
      rt_enabled: true
    }
  }
})

function updateKeyConfig() {
  if (keyconfig.value == null) return
  for (let i = 0; i < select_indexs.value.length; i++) {
    device.device_config!.keys[select_indexs.value[i]].press_percentage = keyconfig.value.press_percentage
    device.device_config!.keys[select_indexs.value[i]].release_percentage = keyconfig.value.release_percentage
    device.device_config!.keys[select_indexs.value[i]].dead_zone = keyconfig.value.dead_zone
    device.device_config!.keys[select_indexs.value[i]].release_dead_zone = keyconfig.value.release_dead_zone
  }
}


emitter.on('get-default-config', async () => {
  emitter.emit('loading')
  if (device.is_pure()) {
    try {
      device.device_config = await apib.get_default_key_config()
      device.extract_key_config_pure64()

      onLayerUpdate()
      kb.selectAllKey(false)
      keyconfig.value = null

      emitter.emit('header-msg-update', { status: "success", str: t('reset_success') })
      emitter.emit('sync-btn-highlight', { status: true })
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
    }
  }
  emitter.emit('loaded')
})

emitter.on('save-config', async () => {
  emitter.emit('loading')
  if (device.is_pure()) {
    try {
      await apib.save_key_config()
      emitter.emit('header-msg-update', { status: "success", str: t('sync_success') })
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
    }
  }
  emitter.emit('loaded')
})


emitter.on('sync-config', async () => {
  emitter.emit('header-loading', { str: t('syncing_config') })
  if (device.is_pure()) {
    const { device_config } = storeToRefs(device)
    const cfg = device_config;

    try {
      for (let i = 0; i < cfg.value!.keys.length; i++) {
        if (cfg.value!.keys[i].dead_zone < 3) {
          store.need_check = true
        }
        if (cfg.value!.keys[i].press_percentage < 2) {
          store.need_check = true
        }
        if (cfg.value!.keys[i].release_percentage < 2) {
          store.need_check = true
        }
      }

      device.store_key_config_pure64()
      await apib.set_key_config(device.device_config!)
      device.extract_key_config_pure64()

      if (store.need_check) {
        emitter.emit('header-msg-update', { status: "warning", str: t('applied_config') })
        message.warning(t('check_config_msg'))
      } else {
        emitter.emit('save-config')
      }

      onLayerUpdate()
      kb.selectAllKey(false)
      keyconfig.value = null
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
      emitter.emit('header-msg-update', { status: "error", str: t('sync_error', { e: getErrorMsg(t, e as IError) }) })
    }
  }
  emitter.emit('sync-btn-highlight', { status: false })
  emitter.emit('loaded')
})



</script>

<template>
  <n-layout has-sider class="pure-config">
    <n-layout-sider content-class="side-pure-config" :width="200">
      <n-menu class="pure-menu" :icon-size="0" :options="menuOptions" v-model:value="kb.mode" :on-update:value="onModeChange" />
    </n-layout-sider>

    <n-layout content-class="main-pure-config">
      <div class="content">
        <div class="main-keyboard">
          <div class="keyboard-frame" v-if="device.device_config != null">
            <div class="keyboard" :style="keymapStyle">
              <div v-for="(keyIndex, lineIndex) in keyIndexes" :key="lineIndex" class="keyLine">
                <div v-for="i in keyIndex" :key="i">
                  <KeyFrame :unit-width="keyWidth[i]">
                    <KeyHall v-if="kb.mode === 0" :key-show="kb.showkeys[i]" v-model:isSelected="kb.keyVarsRefs[i].isSelected"
                      v-model:press_percentage="device.device_config.keys[i].press_percentage"
                      v-model:release_percentage="device.device_config.keys[i].release_percentage"
                      v-model:dead_zone="device.device_config.keys[i].dead_zone"
                      v-model:release_dead_zone="device.device_config.keys[i].release_dead_zone"
                      v-model:rt_enabled="device.device_config.keys[i].rt_enabled" />
      
                    <KeyModify v-if="kb.mode === 1" :key-show="kb.showkeys[i]" :key-str-index="i" v-model:key-current-index="currentOnKey"/>
      
                    <KeyCalibrate v-if="kb.mode === 2" :key-show="kb.showkeys[i]"
                      v-model:isSelected="kb.keyVarsRefs[i].isSelected"
                      v-model:isCalibrated="kb.keyCalibrateRefs[i].isCalibrated"
                      v-model:isCalibrating="kb.keyCalibrateRefs[i].isCalibrating" />
      
                    <KeyDebug v-if="kb.mode === 3" v-model:hallValue="kb.keyDebugRefs[i].hallValue"
                      v-model:hallValuePercentage="kb.keyDebugRefs[i].hallValuePercentage"
                      v-model:isPressed="kb.keyDebugRefs[i].isPressed" v-model:keyShowMode="keyShowMode"
                      v-model:totalDistance="totalDistance" />
                  </KeyFrame>
                </div>
              </div>
            </div>
          </div>
          <div class="keyboard-footer">
            <n-button-group v-if="kb.isSelectAble()">
                <n-button @click="() => kb.selectAllKey(true)">
                  全选
                </n-button>
                <n-button @click="() => kb.selectAllKey(false)">
                  取消全选
                </n-button>
                <n-button @click="() => kb.selectReverse()">
                  反选
                </n-button>
              </n-button-group>
  
              <n-input-number v-if="kb.mode === 3 && keyShowMode === 1" style="width: 100px;" v-model:value="totalDistance"
              :precision="2" :show-button="false">
              <template #suffix>mm</template>
            </n-input-number>
            <div style="padding-left: 5px;">
              <n-radio-group v-if="kb.mode === 3" v-model:value="keyShowMode">
                <n-radio-button v-for="s in DebugSel" :key="s.value" :value="s.value" :label="s.label" />
              </n-radio-group>
    
              <n-button v-if="kb.mode === 2" @click="() => selectKeyCalibrate()">
                校准选中的键
              </n-button>
              <n-button v-if="kb.mode === 1" @click="onLayerChange">
                切换Fn层
              </n-button>
            </div>
          </div>
        </div>
      </div>

      <div class="bottom-config">
        <template v-if="kb.mode === 1">
          <ModifyKeys @dragkey="startDragKey" @dragkeyend="endDragKey"></ModifyKeys>
        </template>

        <div v-if="device.device_config != null && kb.mode === 0">
          <ConfigKeys v-model:key-config="keyconfig" @update="updateKeyConfig" style="top: 20px;"></ConfigKeys>
        </div>
      </div>
    </n-layout>
  </n-layout>
</template>



<style lang="scss" scoped>

.pure-menu {
  height: 70%;
}

.keyboard-frame {
  background-color: var(--color-background-soft);
  padding: 8px 8px;
  border: 8px solid var(--color-border);
  border-radius: 16px;
  width: fit-content;
  height: fit-content;

  .keyboard {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    overflow-wrap: break-word;

    .keyLine {
      display: flex;
      flex-direction: row;
    }
  }
}


</style>

<style lang="scss" scoped>
// layout
.content {
  display: flex;
  align-items: center;
  justify-content: center;
}

.pure-config {
  width: -webkit-fill-available;
  height: -webkit-fill-available;
}

.top-bar {
  display: flex;
  align-items: center;
  justify-content: center;
}

.main-keyboard {
  width: fit-content;
}

.keyboard-footer {
  display: flex;
  justify-content: flex-end;
  margin-right: 15px;
  margin-top: 10px;
}


.bottom-config {
  display: flex;
  align-items: flex-start;
  justify-content: center;

}
</style>

<style lang="scss">
// layout


.side-pure-config {
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--color-background);
}

.main-pure-config {
  width: -webkit-fill-available;
  height: -webkit-fill-available;
  display: grid;
  justify-content: center;
}


</style>
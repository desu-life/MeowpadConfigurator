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
import { formatKey, getErrorMsg, most, splitArray } from "@/utils";
import { useI18n } from "vue-i18n";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { useKeyboard } from "@/store/keyboard";
import { Toggle } from "@/interface";
import { storeToRefs } from "pinia";

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

const HallFilterSel = [
  {
    value: 0,
    label: "关闭"
  },
  {
    value: 1,
    label: "低"
  },
  {
    value: 2,
    label: "高"
  },
  {
    value: 3,
    label: "延迟享受者"
  },
]

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
  apib.calibration_key(indexs)

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
      device.connected = false
      store.status = "error"
      store.status_str = t('connection_broke', { e: getErrorMsg(t, e as IError) })
      store.loading = false
      console.error(e)
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
      device.connected = false
      store.status = "error"
      store.status_str = t('connection_broke', { e: getErrorMsg(t, e as IError) })
      store.loading = false
      console.error(e)
    }
  }, 100)

  onUnmounted(() => {
    clearInterval(interval)
  })
})


const keyModifyDraggedKey = ref<IMixedKey | null>(null);

const keyDraggingStyle = ref({
  left: "0px",
  top: "0px",
});

const keyLayer = ref(0);

function startDragKey(key: IMixedKey) {
  keyModifyDraggedKey.value = key;
  document.onmousemove = (event: MouseEvent) => {
    keyDraggingStyle.value.left = `${event.pageX - 32}px`;
    keyDraggingStyle.value.top = `${event.pageY - 32}px`;
  }

  document.onmouseup = () => {
    document.onmousemove = null;
    document.onmouseup = null;
    keyModifyDraggedKey.value = null;
  }
}



emitter.on('key-str-modify', (event: { rawIndex: number; newValue: IMixedKey }) => {
  if (kb.mode != 1) return

  kb.showkeys[event.rawIndex] = event.newValue

  if (keyLayer.value == 0) {
    device.device_config!.normal_layer[event.rawIndex] = event.newValue
  } else if (keyLayer.value == 1) {
    device.device_config!.fn_layer[event.rawIndex] = event.newValue
  }
})

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
    for (let i = 0; i < select_indexs.value.length; i++) {
      press_percentages.push(device.device_config!.keys[select_indexs.value[i]].press_percentage)
      release_percentages.push(device.device_config!.keys[select_indexs.value[i]].release_percentage)
      dead_zones.push(device.device_config!.keys[select_indexs.value[i]].dead_zone)
    }

    // todo: 另外两个属性
    keyconfig.value = {
      press_percentage: most(press_percentages),
      release_percentage: most(release_percentages),
      dead_zone: most(dead_zones),
      release_dead_zone: 5,
      rt_enabled: true
    }
  }
})


function onKeyPUpdate(v) {
  if (keyconfig.value == null) return
  keyconfig.value.press_percentage = v
  onKeyConfigUpdate()
}

function onKeyRUpdate(v) {
  if (keyconfig.value == null) return
  keyconfig.value.release_percentage = v
  onKeyConfigUpdate()
}

function onKeyDUpdate(v) {
  if (keyconfig.value == null) return
  keyconfig.value.dead_zone = v
  onKeyConfigUpdate()
}

function onKeyConfigUpdate() {
  if (keyconfig.value == null) return
  for (let i = 0; i < select_indexs.value.length; i++) {
    device.device_config!.keys[select_indexs.value[i]].press_percentage = keyconfig.value.press_percentage
    device.device_config!.keys[select_indexs.value[i]].release_percentage = keyconfig.value.release_percentage
    device.device_config!.keys[select_indexs.value[i]].dead_zone = keyconfig.value.dead_zone
  }

  emitter.emit('need-save')
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

function temp_convert(key: number): IMixedKey  {
  return {
    t: "Keyboard",
    c: key
  }
}

</script>

<template>
  <Teleport to="body">
    <div class="dragging-key" v-if="keyModifyDraggedKey != null" :style="keyDraggingStyle">
      <KeyFrame :style="keymapStyle">
        {{ formatKey(keyModifyDraggedKey) }}
      </KeyFrame>
    </div>
  </Teleport>

  <div class="main-pure-config">
    <!-- 拖拽的键 -->
    <div class="top-bar">
      <n-tabs type="line" v-model:value="kb.mode" :on-update:value="onModeChange">
        <n-tab :name="0">
          霍尔
        </n-tab>
        <n-tab :name="1">
          按键
        </n-tab>
        <n-tab :name="2">
          校准
        </n-tab>
        <n-tab :name="3">
          调试
        </n-tab>
        <template #suffix>
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
        </template>
      </n-tabs>
    </div>

    <div class="main-keyboard">
      <div class="keyboard-frame" v-if="device.device_config != null">
        <div class="keyboard" :style="keymapStyle">
          <div v-for="(keyIndex, lineIndex) in keyIndexes" :key="lineIndex" class="keyLine">
            <div v-for="i in keyIndex" :key="i">
              <KeyFrame :unit-width="keyWidth[i]">
                <KeyHall v-if="kb.mode === 0" :keyStr="kb.getKeyStr(i)" v-model:isSelected="kb.keyVarsRefs[i].isSelected"
                  v-model:press_percentage="device.device_config.keys[i].press_percentage"
                  v-model:release_percentage="device.device_config.keys[i].release_percentage"
                  v-model:dead_zone="device.device_config.keys[i].dead_zone"
                  v-model:release_dead_zone="device.device_config.keys[i].release_dead_zone"
                  v-model:rt_enabled="device.device_config.keys[i].rt_enabled" />
  
                <KeyModify v-if="kb.mode === 1" :keyStr="kb.getKeyStr(i)" :keyStrIndex="i"
                  v-model:keyDragged="keyModifyDraggedKey" />
  
                <KeyCalibrate v-if="kb.mode === 2" :keyStr="kb.getKeyStr(i)"
                  v-model:isSelected="kb.keyVarsRefs[i].isSelected"
                  v-model:isCalibrated="kb.keyCalibrateRefs[i].isCalibrated"
                  v-model:isCalibrating="kb.keyCalibrateRefs[i].isCalibrating" />
  
                <KeyDebug v-if="kb.mode === 3" :keyStr="kb.getKeyStr(i)" v-model:hallValue="kb.keyDebugRefs[i].hallValue"
                  v-model:hallValuePercentage="kb.keyDebugRefs[i].hallValuePercentage"
                  v-model:isPressed="kb.keyDebugRefs[i].isPressed" v-model:keyShowMode="keyShowMode"
                  v-model:totalDistance="totalDistance" />
              </KeyFrame>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="bottom-config">
      <template v-if="kb.mode === 1">
        <n-card :bordered="false" class="key-modify-config-card" size="small">
          <template #action>
            将按键拖拽至上方
          </template>
          <n-scrollbar class="key-modify-config-scrollbar" :style="keymapStyle">
            <div class="key-modify-config">
              <KeyFrame class="key-modify-config-key" v-for="(v, i) in mapping" :key="i">
                <KeyModifyOption :keyStr="v" :KeyValue="temp_convert(parseInt(i as any))"
                  @dragkey="startDragKey" />
              </KeyFrame>
            </div>
          </n-scrollbar>
        </n-card>
      </template>

      <div v-if="device.device_config != null && kb.mode === 0">
        <n-card :bordered="false" class="keyboard-config-card" content-class="keyboard-config-card-content">
          <n-grid :cols="24" :x-gap="18">
            <n-gi :span="9">
              <n-card :bordered="false" class="device-config-card" content-class="device-config-card-content">
                <div v-if="keyconfig != null" class="device-config-key-config">

                  <n-form-item label-placement="left" label="死区" path="dead_zone" :show-feedback="false">
                    <n-input-number v-model:value="keyconfig.dead_zone" :precision="0" :step="1" :min="0" :max="100" :on-update:value="v => onKeyDUpdate(v)">
                      <template #suffix>
                        %
                      </template>
                    </n-input-number>
                    <!-- <n-slider v-model:value="keyconfig.dead_zone" :step="0.5" :min="0" :max="100"
                      :format-tooltip="formatPercentTooltip" :on-update:value="v => onKeyDUpdate(v)" /> -->
                  </n-form-item>
                  <n-form-item label-placement="left" label="按下" path="press_percentage" :show-feedback="false">
                    <n-input-number v-model:value="keyconfig.press_percentage" :precision="0" :step="1" :min="1" :max="100" :on-update:value="v => onKeyPUpdate(v)">
                      <template #suffix>
                        %
                      </template>
                    </n-input-number>
                    <!-- <n-slider v-model:value="keyconfig.press_percentage" :step="0.5" :min="0.5" :max="100"
                      :format-tooltip="formatPercentTooltip" :on-update:value="v => onKeyPUpdate(v)" /> -->
                  </n-form-item>
                  <n-form-item label-placement="left" label="抬起" path="release_percentage" :show-feedback="false">
                    <n-input-number v-model:value="keyconfig.release_percentage" :precision="0" :step="1" :min="1" :max="100" :on-update:value="v => onKeyRUpdate(v)">
                      <template #suffix>
                        %
                      </template>
                    </n-input-number>
                    <!-- <n-slider v-model:value="keyconfig.release_percentage" :step="0.5" :min="0.5" :max="100"
                      :format-tooltip="formatPercentTooltip" :on-update:value="v => onKeyRUpdate(v)" /> -->
                  </n-form-item>
                </div>
                <template v-else>
                  <n-empty description="请先选中按键">
                  </n-empty>
                </template>
              </n-card>
            </n-gi>
            <n-gi :span="15">
              <n-grid :cols="24" :x-gap="9">
                <n-form-item-gi :span="12" path="jitters_elimination_time" :show-feedback="false" class="single-key-config">
                  <n-input-number v-model:value="device.jitters_elimination_time" placeholder="Input" :min="0" :max="50"
                    :step="0.5">
                    <template #suffix>
                      ms
                    </template>
                  </n-input-number>
                  <template #label>
                    <n-tooltip trigger="hover" :delay="200">
                      <template #trigger>
                        <n-text underline>
                          消抖时长
                        </n-text>
                      </template>
                      <template #default>
                        按键的去抖，不会增加按下时的延迟，osu玩家推荐开启
                      </template>
                    </n-tooltip>
                  </template>
                </n-form-item-gi>
                <n-form-item-gi :span="12" path="hall_filter" :show-feedback="false">
                  <n-select v-model:value="device.hall_filter" :options="HallFilterSel" />
                  <template #label>
                    <n-tooltip trigger="hover" :delay="200">
                      <template #trigger>
                        <n-text underline>
                          滤波等级
                        </n-text>
                      </template>
                      <template #default>
                        用于增加精度，但会有一点延迟，推荐开启
                      </template>
                    </n-tooltip>
                  </template>
                </n-form-item-gi>
                <n-form-item-gi :span="12" label="灯光亮度" path="max_brightness" :show-feedback="false">
                  <n-input-number v-model:value="device.max_brightness" placeholder="Input" :min="0" :max="100"
                    :step="1">
                    <template #suffix>
                      %
                    </template>
                  </n-input-number>
                </n-form-item-gi>

                <n-form-item-gi :span="12" path="fangwuchu" :show-feedback="false">
                  <n-select v-model:value="device.fangwuchu" :options="ToggleSel" />
                  <template #label>
                    <n-tooltip trigger="hover" :delay="200">
                      <template #trigger>
                        <n-text underline>
                          触底防双击
                        </n-text>
                      </template>
                      <template #default>
                        防止轴体触底震动导致双击，影响底部死区，推荐开启
                      </template>
                    </n-tooltip>
                  </template>
                </n-form-item-gi>
              </n-grid>
            </n-gi>
          </n-grid>
        </n-card>
      </div>

    </div>



  </div>
</template>



<style lang="scss" scoped>
.dragging-key {
  position: absolute;
  pointer-events: none;
  user-select: none;
  opacity: 0.8;
  z-index: 1000;
  transform: rotate(15deg);
}

.keyboard-config-card {
  --item-padding: 16px;

  background-color: var(--color-background-soft);
  border-radius: 8px;
  width: fit-content;

  .single-key-config {
    padding-bottom: var(--item-padding);
  }
  
  .device-config-card {
    background-color: var(--color-background-soft);
    border-radius: 8px;
    border: 1px solid var(--color-border);
    height: 100%;
  
    .device-config-key-config {
      width: 90%;
    }
  }
}

.key-modify-config-card {
  background-color: var(--color-background-soft);
  border-radius: 8px;
  width: fit-content;

  .key-modify-config {
    display: flex;
    flex-wrap: wrap;
    overflow-wrap: break-word;
  }
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

    .keyLine {
      display: flex;
      flex-direction: row;
    }
  }
}
</style>

<style lang="scss" scoped>
// layout
.main-pure-config {
  width: -webkit-fill-available;
  height: -webkit-fill-available;
  display: grid;
  justify-content: center;
}

.top-bar {
  display: flex;
  align-items: center;
  justify-content: center;
}

.main-keyboard {
  display: flex;
  align-items: center;
  justify-content: center;
}

.bottom-config {
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>

<style lang="scss">
// 这个不能scoped
.key-modify-config-scrollbar {
  max-height: calc(var(--default-key-height) * 3);
  max-width: calc((var(--default-key-width) * 16) + var(--n-scrollbar-width) + var(--default-key-margin));
}

.device-config-card-content {
  padding: 0px !important;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
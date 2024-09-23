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
import KeySelect from "@/components/meowboard/Keyboard/KeySelect.vue";
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
import { IKeyboard, IKeyConfigBoard } from "@/apis/meowboard/config";
import emitter from "@/mitt";
import { getErrorMsg } from "@/utils";
import { useI18n } from "vue-i18n";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { useKeyboard } from "@/store/keyboard";
import { Toggle } from "@/interface";

const message = useMessage()
const dialog = useDialog()
const { t } = useI18n();
const store = useStore()
const kb = useKeyboard()
const device = useDeviceStore()

const keyconfig = ref<IKeyConfigBoard | null>(null)

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

function splitArray<T>(array: T[], sizes: number[]): T[][] {
  let result: T[][] = [];
  let index = 0;

  for (let size of sizes) {
    let subArray = array.slice(index, index + size);
    result.push(subArray);
    index += size;
  }

  return result;
}

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


const keyModifyDraggedKey = ref(-1);
const keyDraggingStyle = ref({
  left: "0px",
  top: "0px",
});

const keyLayer = ref(0);

emitter.on('global-mouse-move', onMouseMove)
emitter.on('global-mouse-up', onMouseUp)


function onMouseMove(event: MouseEvent) {
  keyDraggingStyle.value.left = `${event.pageX - 32}px`;
  keyDraggingStyle.value.top = `${event.pageY - 32}px`;
}

function onMouseUp(event: MouseEvent) {
  keyModifyDraggedKey.value = -1;
}

function onMouseEnter(event: MouseEvent) {
  keyModifyDraggedKey.value = -1;
}

emitter.on('key-str-modify', onKeyStrModify)
function onKeyStrModify(event: { rawIndex: number; newValue: number }) {
  if (event.newValue == -1) return
  if (kb.mode != 1) return
  if (event.rawIndex == 60) {
    message.warning("Fn键无法修改")
    return
  }

  if (event.newValue > 0) {
    for (let i = 0; i < 64; i++) {
      if (kb.showkeys[i] == event.newValue) {
        message.warning("有重复的按键，请检查")
        return
      }
    }
  }

  kb.showkeys[event.rawIndex] = event.newValue
  kb.showkeys[60] = 0

  if (keyLayer.value == 0) {
    device.device_config!.normal_layer[event.rawIndex] = event.newValue
  } else if (keyLayer.value == 1) {
    device.device_config!.fn_layer[event.rawIndex] = event.newValue
  }
}

emitter.on('kb-init', onKbInit);

function onKbInit() {
  onLayerUpdate()
  kb.selectAllKey(false)
  keyconfig.value = null
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

function most(arr) {
  let obj = arr.reduce((p, n) => (
    p[n]++ || (p[n] = 1),
    (p.max = p.max >= p[n] ? p.max : p[n]),
    (p.key = p.max > p[n] ? p.key : n), p), {})
  return obj.key
}

let select_indexs = ref<number[]>([])

emitter.on('key-select', onKeySelect)
function onKeySelect() {
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
}

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

function formatPercentTooltip(percentage: number) {
  return `${percentage}%`
}


</script>

<template>
  <Teleport to="body">
    <div class="dragging-key" v-if="keyModifyDraggedKey != -1" :style="keyDraggingStyle">
      <KeyFrame :width="(1 * 64).toString() + 'px'">
        {{ mapping[keyModifyDraggedKey] ?? '' }}
      </KeyFrame>
    </div>
  </Teleport>

  <div class="main-keyboard">
    <!-- 拖拽的键 -->

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

    <div class="keyboard-frame" v-if="device.device_config != null">
      <div class="keyboard">
        <div v-for="(keyIndex, lineIndex) in keyIndexes" :key="lineIndex" class="keyLine">
          <div v-for="i in keyIndex" :key="i" class="keyIndividual">
            <KeyFrame :width="(keyWidth[i] * 64).toString() + 'px'">

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

    <div class="keyboard-config">


      <template v-if="kb.mode === 1">
        <n-card :bordered="false" class="keyboard-config-card">
          <template #action>
            将按键拖拽至上方
          </template>
          <n-scrollbar class="key-modify-config-scrollbar">
            <div class="key-modify-config">
              <div v-for="(v, i) in mapping" :key="i">
                <KeyFrame class="key-modify-config-key" :width="(1 * 64).toString() + 'px'">
                  <KeyModifyOption :keyStr="v" :KeyValue="parseInt(i as any)"
                    v-model:keyDragged="keyModifyDraggedKey" />
                </KeyFrame>
              </div>
            </div>
          </n-scrollbar>
        </n-card>
      </template>

      <div v-if="device.device_config != null && kb.mode === 0">
        <n-card :bordered="false" class="keyboard-config-card">


          <n-grid :cols="24" :x-gap="18" class="device-config">
            <n-gi :span="9">
              <n-card :bordered="false" class="device-config-card">
                <div v-if="keyconfig != null" class="device-config-key-config">

                  <n-form-item label-placement="left" label="死区" path="dead_zone">
                    <n-input-number v-model:value="keyconfig.dead_zone" :precision="0" :step="1" :min="0" :max="100" :on-update:value="v => onKeyDUpdate(v)">
                      <template #suffix>
                        %
                      </template>
                    </n-input-number>
                    <!-- <n-slider v-model:value="keyconfig.dead_zone" :step="0.5" :min="0" :max="100"
                      :format-tooltip="formatPercentTooltip" :on-update:value="v => onKeyDUpdate(v)" /> -->
                  </n-form-item>
                  <n-form-item label-placement="left" label="按下" path="press_percentage">
                    <n-input-number v-model:value="keyconfig.press_percentage" :precision="0" :step="1" :min="1" :max="100" :on-update:value="v => onKeyPUpdate(v)">
                      <template #suffix>
                        %
                      </template>
                    </n-input-number>
                    <!-- <n-slider v-model:value="keyconfig.press_percentage" :step="0.5" :min="0.5" :max="100"
                      :format-tooltip="formatPercentTooltip" :on-update:value="v => onKeyPUpdate(v)" /> -->
                  </n-form-item>
                  <n-form-item label-placement="left" label="抬起" path="release_percentage">
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
                <n-form-item-gi :span="12" path="jitters_elimination_time">
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
                <n-form-item-gi :span="12" path="hall_filter">
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
                <n-form-item-gi :span="12" label="灯光亮度" path="max_brightness">
                  <n-input-number v-model:value="device.max_brightness" placeholder="Input" :min="0" :max="100"
                    :step="1">
                    <template #suffix>
                      %
                    </template>
                  </n-input-number>
                </n-form-item-gi>

                <n-form-item-gi :span="12" path="fangwuchu">
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

<style lang="scss">
.temp-test {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-start;
  gap: 8px;
}

.dragging-key {
  position: absolute;
  pointer-events: none;
  user-select: none;
  opacity: 0.8;
  z-index: 1000;
  transform: rotate(15deg);
}

.key-modify-config-scrollbar {
  max-height: 250px;
  max-width: 905px;
}

.device-config {
  max-width: 905px;
}

.keyboard-config-card {
  background-color: #242424;
  border-radius: 8px;
  width: fit-content;

  .n-card__content {
    padding-bottom: 0px !important;
  }
}

.device-config-card .n-form-item .n-form-item-feedback-wrapper {
  min-height: 3px !important;
}

.device-config-card {
  background-color: #242424;
  border-radius: 8px;
  border: 1px solid #363636;
  height: 85%;



  .n-card__content {
    padding: 0px !important;
    display: flex;
    align-items: center;
    justify-content: center;

  }

  .device-config-key-config {
    width: 90%;
  }
}

.keyboard-config {
  display: flex;
  align-items: center;
  justify-content: center;
}

.key-modify-config {
  display: flex;
  flex-wrap: wrap;
  overflow-wrap: break-word;
}

.keyboard-frame {
  background-color: #242424;
  padding: 8px 8px 24px;
  border: 8px solid #363636;
  border-radius: 16px;
  width: fit-content;
  height: fit-content;

  .keyboard {
    display: flex;
    flex-direction: column;
    gap: 16px;

    .keyLine {
      display: flex;
      flex-direction: row;

      .keyIndividual {
        height: 48px;
        //width: var(--key-width);
      }
    }
  }
}

$header-height: 56px;


.main-keyboard {
  position: fixed;
  width: -webkit-fill-available;
  height: -webkit-fill-available;
  display: grid;
  justify-content: center;
  margin: 20px;
}
</style>
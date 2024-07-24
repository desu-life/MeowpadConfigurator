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

import KeyFrame from "@/components/Keyboard/KeyFrame.vue";
import KeyDebug from "@/components/Keyboard/KeyDebug.vue";
import KeySelect from "@/components/Keyboard/KeySelect.vue";
import KeyCalibrate from "@/components/Keyboard/KeyCalibrate.vue";
import { ComponentPublicInstance, createVNode } from "vue";

import * as apib from '@/apis/meowboard/api'
import { KeyState } from "@/apis";
import { KeyCode, mapping } from "@/keycode";
import { useStore } from "@/store/main";
import { IKeyboard } from "@/apis/meowboard/config";
import KeyHall from "./Keyboard/KeyHall.vue";
import KeyModify from "./Keyboard/KeyModify.vue";
import KeyModifyOption from "./Keyboard/KeyModifyOption.vue";
import emitter from "@/mitt";

const store = useStore()


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

// const keyStrs = ref<string[]>(new Array(64))
const keyStrs = ref([
  '\`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 'Backspace',
  'Tab', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '[', ']', '\\',
  'Caps', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';', '\'', 'Enter',
  'Shift', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '/', 'Shift', '↑', 'Del',
  'Ctrl', 'Win', 'Alt', 'Space', 'Ctrl', 'Fn', '←', '↓', '→'
])

// 0-校准 1-调试
const mode = ref(1);



interface DebugVars {
  hallValue: number;
  hallValuePercentage: number;
  isPressed: boolean;
}

interface CalibrateVars {
  isSelected: boolean;
  isCalibrated: boolean;
  isCalibrating: boolean;
}

const keyCalibrateRefs = ref<CalibrateVars[]>(new Array(64));
const keyDebugRefs = ref<DebugVars[]>(new Array(64));

for (let i = 0; i < 64; i++) {
  keyCalibrateRefs.value[i] = {
    isSelected: false,
    isCalibrated: false,
    isCalibrating: false
  }
  keyDebugRefs.value[i] = {
    hallValue: 0,
    hallValuePercentage: 0,
    isPressed: false
  }
}


function selectAllKey(on: boolean) {
  keyCalibrateRefs.value.forEach((key) => {
    key.isSelected = on;
  })
}
function selectReverse() {
  keyCalibrateRefs.value.forEach((key) => {
    key.isSelected = !key.isSelected;
  })
}



function selectKeyCalibrate() {
  let indexs: number[] = []
  for (let i = 0; i < 64; i++) {
    if (keyCalibrateRefs.value[i].isSelected) {
      indexs.push(i)
    }
  }
  console.log(indexs)
  if (indexs.length == 0) return
  apib.calibration_key(indexs)

  for (let i = 0; i < indexs.length; i++) {
    keyCalibrateRefs.value[indexs[i]].isSelected = false;
    keyCalibrateRefs.value[indexs[i]].isCalibrating = true;
    keyCalibrateRefs.value[indexs[i]].isCalibrated = false;
  }

  const interval = setInterval(async () => {
    try {
      if (is_connected.value == false) {
        return
      }

      let success_count = 0;

      let cali_status = await apib.get_key_calibrate_status()
      for (let i = 0; i < indexs.length; i++) {
        if (cali_status[indexs[i]]) {
          keyCalibrateRefs.value[indexs[i]].isCalibrated = true
          keyCalibrateRefs.value[indexs[i]].isCalibrating = false
          success_count += 1
        }
      }

      if (success_count == indexs.length) {
        clearInterval(interval)
      }
    } catch (e) {
      console.log(e)
      store.status_str = "连接断开"
      store.status = "error"
    }
  }, 100)
}

function setKeyDebugHallValue(value: number) {
  // keyDebugRefs.value[35].hallValue = 100
  // keyDebugRefs.value[35].hallValuePercentage = value
  // keyDebugRefs.value[36].hallValue = 100
  // keyDebugRefs.value[36].hallValuePercentage = value
  keyDebugRefs.value[37].hallValue = 100
  keyDebugRefs.value[37].hallValuePercentage = value
}

async function updateKey() {
  if (mode.value == 2) {
    let states = await apib.get_keystates()
    let cali_status = await apib.get_key_calibrate_status()
    for (let i = 0; i < 64; i++) {
      keyCalibrateRefs.value[i].isCalibrated = cali_status[i]
      keyCalibrateRefs.value[i].isCalibrating = states[i] == KeyState.Calibrating
    }
  } else if (mode.value == 3) {
    let value = await apib.get_debug_value()

    for (let i = 0; i < 64; i++) {
      keyDebugRefs.value[i].hallValue = value[i].adc_value
      keyDebugRefs.value[i].hallValuePercentage = value[i].press_percentage
      keyDebugRefs.value[i].isPressed = value[i].key_state == KeyState.Pressed
    }
  }
}


defineExpose({
  mode
});

const is_connected = ref(false)
const debug_index = ref(0)
const device_config = ref<IKeyboard | null>(null)

onMounted(async () => {
  console.log('mounted')
  let connected = await apib.connect();
  if (!connected) {
    console.log('connect failed')
    store.loading = false
    store.status_str = "连接失败"
    store.status = "error"
    return
  }
  is_connected.value = true
  // try {
  //   await apib.get_key_config()
  // } catch (e) {
  // }
  store.loading = true
  let status = await apib.get_device_status()
  console.log(status)
  if (status.key == false) {
    let config = await apib.get_default_key_config()
    await apib.set_key_config(config)
    await apib.save_key_config()
  }

  let config = await apib.get_key_config()
  console.log(config)

  device_config.value = config

  let hall_config = await apib.get_hall_config()
  console.log(hall_config)

  for (let i = 0; i < 64; i++) {
    keyStrs.value[i] = mapping[config.normal_layer[i]] ?? ""
  }
  keyStrs.value[60] = "Fn"

  await updateKey()

  store.loading = false
  store.status_str = "已连接"
  store.status = "success"

  for (let i in mapping) {
    console.log(i, mapping[i])
  }

  const interval = setInterval(async () => {
    try {
      if (is_connected.value == false) {
        return
      }

      await updateKey()
    } catch (e) {
      console.log(e)
      is_connected.value = false
    }
  }, 100)

  onUnmounted(() => {
    clearInterval(interval)
  })
})


const keyModifyDraggedKey = ref("no-drag");
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
  keyModifyDraggedKey.value = "no-drag";
}

function onMouseEnter(event: MouseEvent) {
  keyModifyDraggedKey.value = "no-drag";
}

emitter.on('key-str-modify', onKeyStrModify)
function onKeyStrModify(event: { rawIndex: number; newValue: string }) {
  keyStrs.value[event.rawIndex] = event.newValue
}

async function onLayerChange() {
  let config = await apib.get_key_config()

  if (keyLayer.value == 0) {
    for (let i = 0; i < 64; i++) {
      keyStrs.value[i] = mapping[config.fn_layer[i]] ?? ""
    }
    keyStrs.value[60] = "Fn"
    keyLayer.value = 1
  } else {
    for (let i = 0; i < 64; i++) {
      keyStrs.value[i] = mapping[config.normal_layer[i]] ?? ""
    }
    keyStrs.value[60] = "Fn"
    keyLayer.value = 0
  }

}

</script>

<template>
  <div class="main-keyboard">
    <!-- 拖拽的键 -->
    <Teleport to="body">
      <div class="dragging-key" v-if="keyModifyDraggedKey!='no-drag'" :style="keyDraggingStyle">
          <KeyFrame :width="(1 * 64).toString() + 'px'">
            {{ keyModifyDraggedKey }}
          </KeyFrame>
      </div>
    </Teleport>

    <div class="temp-test">
      <n-button @click="() => updateKey()">
        刷新
      </n-button>

      <n-space vertical>
        <n-radio-group v-model:value="mode" name="modes">
          <n-radio-button :key="0" :value="0" :label="'霍尔'" />
          <n-radio-button :key="1" :value="1" :label="'按键'" />
          <n-radio-button :key="2" :value="2" :label="'校准'" />
          <n-radio-button :key="3" :value="3" :label="'调试'" />
        </n-radio-group>
      </n-space>

      <n-button-group>
        <n-button v-if="mode === 0|| mode === 2" @click="() => selectAllKey(true)">
          全选
        </n-button>
        <n-button v-if="mode === 0|| mode === 2" @click="() => selectAllKey(false)">
          取消全选
        </n-button>
        <n-button v-if="mode === 0|| mode === 2" @click="() => selectReverse()">
          反选
        </n-button>
      </n-button-group>
      <n-button v-if="mode === 2" @click="() => selectKeyCalibrate()">
        校准选中的键
      </n-button>
      <n-button v-if="mode === 1" @click="onLayerChange">
        切换Fn层
      </n-button>
    </div>


    <n-divider />

    <div class="keyboard-frame" v-if="device_config != null">
      <div class="keyboard">
        <div v-for="(keyIndex, lineIndex) in keyIndexes" :key="lineIndex" class="keyLine">
          <div v-for="i in keyIndex" :key="i" class="keyIndividual">
            <KeyFrame :width="(keyWidth[i] * 64).toString() + 'px'">
              <KeyHall v-if="mode === 0" :keyStr="keyStrs[i]" v-model:isSelected="keyCalibrateRefs[i].isSelected"
                v-model:press_percentage="device_config.keys[i].press_percentage"
                v-model:release_percentage="device_config.keys[i].release_percentage"
                v-model:dead_zone="device_config.keys[i].dead_zone"
                v-model:release_dead_zone="device_config.keys[i].release_dead_zone"
                v-model:rt_enabled="device_config.keys[i].rt_enabled" />
                <KeyModify v-if="mode === 1" :keyStr="keyStrs[i]" :keyStrIndex="i" v-model:keyDragged="keyModifyDraggedKey" />
              <KeyCalibrate v-if="mode === 2" :keyStr="keyStrs[i]" v-model:isSelected="keyCalibrateRefs[i].isSelected"
                v-model:isCalibrated="keyCalibrateRefs[i].isCalibrated"
                v-model:isCalibrating="keyCalibrateRefs[i].isCalibrating" />
              <KeyDebug v-if="mode === 3" :keyStr="keyStrs[i]" v-model:hallValue="keyDebugRefs[i].hallValue"
                v-model:hallValuePercentage="keyDebugRefs[i].hallValuePercentage" v-model:isPressed="keyDebugRefs[i].isPressed" />
            </KeyFrame>
          </div>
        </div>
      </div>
    </div>

    <n-divider />

    <div>
      <div  v-if="mode === 1">
        <n-scrollbar style="max-height: 220px">
          <div class="key-modify-config">
            <div v-for="i in mapping" :key="i">
              <KeyFrame class="key-modify-config-key" :width="(1 * 64).toString() + 'px'">
                <KeyModifyOption :keyStr="i" v-model:keyDragged="keyModifyDraggedKey"/>
              </KeyFrame>
            </div>
          </div>
        </n-scrollbar>
      </div>

      <div class="keyboard-config" v-if="device_config != null && mode === 0">
        <n-input-number v-model:value="device_config!.jitters_elimination_time" :min="0" :max="50" :step="0.5">
          <template #suffix>
            ms
          </template>
        </n-input-number>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
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

.key-modify-config {
  display: flex;
  flex-wrap: wrap;
  overflow-wrap: break-word;
  max-width: 100%;

  overflow-y: 50px;
}

.keyboard-frame {
  background-color: #242424;
  padding: 8px 8px 24px;
  border: 8px solid #363636;
  border-radius: 16px;

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

.main-keyboard {
  display: flex;
  flex-direction: column;
  max-width: min-content;
}
</style>
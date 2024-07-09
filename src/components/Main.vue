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
import KeyCalibrate from "@/components/Keyboard/KeyCalibrate.vue";
import { ComponentPublicInstance, createVNode } from "vue";

import * as apib from '@/apis/meowboard/api'


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
const keyIndexes = splitArray(
    Array.from({length: 64}, (v, i) => i),
    KeysOfEachLine
);


const keyWidth = ref([  // 每个键的ui里的宽度
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
  1.5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1.5,
  1.75, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2.25,
  2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
  1.25, 1.25, 1.25, 6.25, 1, 1, 1, 1, 1
])

const keyStrs = ref([
  '\`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 'Backspace',
  'Tab', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '[', ']', '\\',
  'Caps', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';', '\'', 'Enter',
  'Shift', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '/', 'Shift', '↑', 'Del',
  'Ctrl', 'Win', 'Alt', 'Space', 'Ctrl', 'Fn', '←', '↓', '→'
])

// 0-校准 1-调试
const mode = ref(0);

interface DebugVars {
  hallValue: number;
  hallValuePercentage: number;
}

interface CalibrateVars {
  isSelected: boolean;
  isCalibrated: boolean;
  isCalibrating: boolean;
}

const keyCalibrateRefs = ref<CalibrateVars[]> (new Array(64));
const keyDebugRefs = ref<DebugVars[]>(new Array(64));

for (let i = 0; i < 64; i++) {
    keyCalibrateRefs.value[i] = {
      isSelected: false,
      isCalibrated: false,
      isCalibrating: false
    }
    keyDebugRefs.value[i] = {
      hallValue: 0,
      hallValuePercentage: 0
    }
  }


function selectAllKeyCalibrate(on: boolean) {
  keyCalibrateRefs.value.forEach((key) => {
    key.isSelected = on;
  })
}

function setKeyDebugHallValue(value: number) {
  // keyDebugRefs.value[35].hallValue = 100
  // keyDebugRefs.value[35].hallValuePercentage = value
  // keyDebugRefs.value[36].hallValue = 100
  // keyDebugRefs.value[36].hallValuePercentage = value
  keyDebugRefs.value[37].hallValue = 100
  keyDebugRefs.value[37].hallValuePercentage = value
}


defineExpose({
  mode
});

const is_connected = ref(false)
const debug_index = ref(0)

onMounted(async () => {
  console.log('mounted')
  let connected = await apib.connect();
  if (!connected) {
    console.log('connect failed')
    return
  }
  is_connected.value = true
  // try {
  //   await apib.get_key_config()
  // } catch (e) {
  // }
  let status = await apib.get_device_status()
  if (status.key == false) {
    let config = await apib.get_default_key_config()
    await apib.set_key_config(config)
    await apib.save_key_config()
  }

  let config = await apib.get_key_config()
  console.log(config)

  setTimeout(async () => {
    try {
      if (is_connected.value == false) {
        return
      }
      while (1) {
        let value = await apib.get_debug_value(debug_index.value)
        debug_index.value += 1;
        if (debug_index.value >= 8) {
          debug_index.value = 0
        }
        for (let i = 0; i < 8; i++) {
          keyDebugRefs.value[debug_index.value * 8 + i].hallValue = value[i].adc_value
          keyDebugRefs.value[debug_index.value * 8 + i].hallValuePercentage = value[i].press_percentage
        }
      }
    } catch (e) {
      console.log(e)
    }
  }, 500)



  // const interval = setInterval(async () => {
  //   try {
  //     if (is_connected.value == false) {
  //       return
  //     }

  //     let value = await apib.get_debug_value(debug_index.value)
  //     debug_index.value += 1;
  //     if (debug_index.value >= 8) {
  //       debug_index.value = 0
  //     }
  //     for (let i = 0; i < 8; i++) {
  //       keyDebugRefs.value[debug_index.value * 8 + i].hallValue = value[i].adc_value
  //       keyDebugRefs.value[debug_index.value * 8 + i].hallValuePercentage = value[i].press_percentage
  //     }
  //   } catch (e) {
  //     console.log(e)
  //   }
  // }, 50)

  // onUnmounted(() => {
  //   clearInterval(interval)
  // })
})

</script>

<template>
  <div>
    <div class="temp-test">
      <n-space vertical>
        <n-radio-group v-model:value="mode" name="modes">
          <n-radio-button
              :key="0"
              :value="0"
              :label="'校准'"
          />
          <n-radio-button
              :key="1"
              :value="1"
              :label="'调试'"
          />
        </n-radio-group>
      </n-space>

      <n-button-group>
        <n-button v-if="mode === 0" @click="() => selectAllKeyCalibrate(true)">
          select all
        </n-button>
        <n-button v-if="mode === 0" @click="() => selectAllKeyCalibrate(false)">
          unselect all
        </n-button>
        <n-button v-if="mode === 1" @click="() => setKeyDebugHallValue(0)">
          set 0%
        </n-button>
        <n-button v-if="mode === 1" @click="() => setKeyDebugHallValue(20)">
          set 20%
        </n-button>
        <n-button v-if="mode === 1" @click="() => setKeyDebugHallValue(100)">
          set 100%
        </n-button>
      </n-button-group>
    </div>

    <n-divider/>

    <div class="keyboard-frame">
      <div class="keyboard">
        <div
            v-for="(keyIndex, lineIndex) in keyIndexes"
            :key="lineIndex"
            class="keyLine"
        >
          <div
              v-for="i in keyIndex"
              :key="i"
              class="keyIndividual"
          >
            <KeyFrame
                :width="(keyWidth[i] * 64).toString() + 'px'"
            >
              <KeyCalibrate
                  v-if="mode === 0"
                  :keyStr="keyStrs[i]"
                  :isSelected="keyCalibrateRefs[i].isSelected"
                  :isCalibrated="keyCalibrateRefs[i].isCalibrated"
                  :isCalibrating="keyCalibrateRefs[i].isCalibrating"
              />
              <KeyDebug
                  v-if="mode === 1"
                  :keyStr="keyStrs[i]"
                  :hallValue="keyDebugRefs[i].hallValue"
                  :hallValuePercentage="keyDebugRefs[i].hallValuePercentage"
              />
            </KeyFrame>
          </div>
        </div>
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

</style>
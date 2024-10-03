import { defineStore, acceptHMRUpdate } from "pinia";
import { IVersion, KeyState } from "@/apis";
import { Type } from "naive-ui/es/button/src/interface";
import * as apib from '@/apis/meowboard/api'
import { mapping } from "@/keycode";
import emitter from "@/mitt";
import { IMixedKey } from "@/apis/meowboard/config";
import { format } from "path";
import { formatKey } from "@/keymap";

interface DebugVars {
  hallValue: number;
  hallValuePercentage: number;
  isPressed: boolean;
}

interface CalibrateVars {
  isCalibrated: boolean;
  isCalibrating: boolean;
}

interface KeyVars {
  isSelected: boolean;
}


export const useKeyboard = defineStore("keyboard", () => {
  const mode = ref(0);
  const keyCalibrateRefs = ref<CalibrateVars[]>(new Array(64));
  const keyDebugRefs = ref<DebugVars[]>(new Array(64));
  const keyVarsRefs = ref<KeyVars[]>(new Array(64));
  const showkeys = ref<IMixedKey[]>(new Array(64));
  
  
  for (let i = 0; i < 64; i++) {
    keyCalibrateRefs.value[i] = {
      isCalibrated: false,
      isCalibrating: false
    }
    keyDebugRefs.value[i] = {
      hallValue: 0,
      hallValuePercentage: 0,
      isPressed: false
    }
    keyVarsRefs.value[i] = {
      isSelected: false,
    }
    showkeys.value[i] = {
      t: "None",
      c: 0
    }
  }

  function selectAllKey(on: boolean) {
    keyVarsRefs.value.forEach((key) => {
      key.isSelected = on;
    })

    emitter.emit('key-select')
  }
  function selectReverse() {
    keyVarsRefs.value.forEach((key) => {
      key.isSelected = !key.isSelected;
    })

    emitter.emit('key-select')
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

  function isSelectAble() {
    if (mode.value == 0 || mode.value == 2) {
      return true
    }
    return false
  }

  function getKeyShow(index: number) {
    let key = showkeys.value[index];
    return formatKey(key);
  }
  
  
  

  return {
    mode,
    keyCalibrateRefs,
    keyDebugRefs,
    keyVarsRefs,
    selectAllKey,
    selectReverse,
    updateKey,
    isSelectAble,
    showkeys,
    getKeyShow
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useKeyboard, import.meta.hot));
}

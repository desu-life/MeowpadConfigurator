<script setup lang="ts">
import { IError, IKeyRTStatus, KeyState } from '@/apis';
import * as apiv2 from '@/apis/meowpadv2/api'
import * as apiv2se from '@/apis/meowpadv2se/api'
import * as apiv21se from '@/apis/meowpadv21se/api'
import * as apiv3 from '@/apis/meowpadv3/api'
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { getErrorMsg } from '@/utils';
import { DataTableColumns } from 'naive-ui';
import { useI18n } from "vue-i18n";
import emitter from '@/mitt';

const store = useStore()
const device = useDeviceStore()
const { t } = useI18n();

const debug_data = ref<IKeyRTStatus[]>()
const btn_state = ref<KeyState[]>()


let default_height = ref(55)
let default_width = ref(55)
let default_margin = ref(4)
let default_font_size = ref(15)

let keymapStyle = ref({
  "--default-key-height": default_height.value + "px",
  "--default-key-width": default_width.value + "px",
  "--default-key-margin": default_margin.value + "px",
  "--default-key-font-size": default_font_size.value + "px",
})

import meowpad from '@/meowpad7k.json'
import { IKeymap } from '@/interface';
import KeyDebug from '../KeyComponents/KeyDebug.vue';
const keymap: IKeymap[][] = meowpad;

function key_state_to_str(state: KeyState) {
  switch (state) {
    case KeyState.Pressed:
      return t('pressed')
    case KeyState.Released:
      return t('released')
    case KeyState.Calibrating:
      return t('calibrating')
  }
}

function key_states_to_str(states: KeyState[]) {
  let str = ''
  for (let i = 0; i < states.length; i++) {
    if (i > 0) {
      str += ', '
    }
    str += key_state_to_str(states[i])
  }
  return str
}

const columns: DataTableColumns<IKeyRTStatus> = [
  {
    title: t('adc_value'),
    key: 'adc_value'
  },
  {
    title: t('linear_adc_value'),
    key: 'linear_value'
  },
  {
    title: t('press_percentage'),
    key: 'press_percentage',
  },
  {
    title: t('key_state'),
    key: 'key_state',
    render(h) {
      return key_state_to_str(h.key_state)
    },
  }
]

onMounted(() => {
  const interval = setInterval(async () => {
    try {
      if (!device.connected) {
        return
      }
      if (device.is_v3()) {
        let v = await apiv3.get_debug_value();
        debug_data.value = v;
      }
    } catch (e) {
      emitter.emit('connection-broke', { e: e as IError })
      store.debug_mode = false
    }
  }, 50)

  onUnmounted(() => {
    clearInterval(interval)
  })
})

</script>

<template>
  <div v-if="debug_data" class="v3-debug-config">
    <div class="keyboard" :style="keymapStyle">
      <div v-for="line in keymap" class="line">
        <div v-for="(key, i) in line" :key="i" :class="key.index == undefined ? 'hidden' : ''">
          <template v-if="key.index != undefined">
            <template v-if="key.index >= 0">
              <Key :unit-width="key.width" :key-num="key.index">
                <KeyDebug :is-pressed="debug_data![key.index].key_state == KeyState.Pressed"
                  :hall-value="debug_data![key.index].adc_value"
                  :hall-value-percentage="debug_data![key.index].press_percentage"></KeyDebug>
              </Key>
            </template>
            <template v-else>
              <Key :unit-width="key.width" :key-num="key.index" rounded>
                <KeyDebug :is-pressed="debug_data![7].key_state == KeyState.Pressed" :hall-value="debug_data![7].adc_value"></KeyDebug>
              </Key>
            </template>
          </template>
          <template v-else>
            <Key :unit-width="key.width">
              ...
            </Key>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>


<style lang="scss" scoped>
.v3-debug-config {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
}

.keyboard {
  position: relative;
  background: var(--color-background-soft);
  border-radius: var(--n-border-radius);
  border: 5px solid var(--color-border);
  display: flex;
  flex-direction: column;
  height: fit-content;
  padding: 10px;
  width: fit-content;
  /* outline: var(--n-border-radius) solid var(--color-border); */
}

.line {
  display: flex;
  flex-direction: row;
}

.hidden {
  visibility: hidden;
}
</style>
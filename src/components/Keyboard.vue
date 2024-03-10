<script setup lang="ts">
import { useStore } from '@/store'
import { useI18n } from "vue-i18n";
import Key from '@/components/Key.vue'
import KeyShow from '@/components/KeyShow.vue'
import type { IKeymap } from '@/interface';
import { ref } from 'vue';
import { KeyCode, LightingMode, jsToHid } from '@/interface';
import { formatKeys, IsModifierKey, compareArray } from '@/utils'

const { t } = useI18n();
const message = useMessage()
const dialog = useDialog()
const store = useStore()

import meowpad from '@/meowpad.json'
const keymap: IKeymap[][] = meowpad;

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

const selectedKey = ref<number | null>(null)
const showModal = ref(false)
const pressedkeycodes = ref<KeyCode[]>([])
const presskeycodes = ref<KeyCode[]>([])


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
  document.onkeydown = (e) => {
    if (showModal.value === false) { return }
    e.preventDefault()
    const HidCodeDown: KeyCode = jsToHid[e.code] || undefined
    if (HidCodeDown in KeyCode && needkey(HidCodeDown)) {
      console.log("pressed " + e.code)
      pressedkeycodes.value.push(HidCodeDown)
      presskeycodes.value.push(HidCodeDown)
      document.onkeyup = (e) => {
        e.preventDefault()
        document.onkeydown = null // 当有键松开时，清除按下键盘的监听函数
        console.log("released " + e.code)
        const HidCodeUP: KeyCode = jsToHid[e.code] || undefined
        if (presskeycodes.value.includes(HidCodeUP)) {
          presskeycodes.value.splice(presskeycodes.value.indexOf(HidCodeUP), 1)
          keycodes.push(HidCodeUP)
          if (presskeycodes.value.length === 0) {
            keycodes = keycodes.sort((l, r) => r - l)
            compareArray(store.key_config!.keys[keyNum].key_data, keycodes) ? store.key_config!.keys[keyNum].key_data = [] : store.key_config!.keys[keyNum].key_data = keycodes
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


function selectKey(keyNum: number) {
  selectedKey.value = keyNum === selectedKey.value ? null : keyNum;
}

function clickKey(k: number) {
  if (k == selectedKey.value) {
    setKeys(k)
  } else {
    selectKey(k)
  }
}

function applyKeySetting() {
  if (selectedKey.value == null) { return }
  const key = store.key_config!.keys[selectedKey.value]
  for (let i = 0; i < store.key_config!.keys.length; i++) {
    if (i != selectedKey.value) {
      store.key_config!.keys[i].dead_zone = key.dead_zone
      store.key_config!.keys[i].press_percentage = key.press_percentage
      store.key_config!.keys[i].release_percentage = key.release_percentage
    }
  }
  message.success(t('apply_done'))
}

</script>


<template>
  <KeyModal v-model:show="showModal" :pressedkeycodes="pressedkeycodes" :leave-func="clearset"></KeyModal>
  <div class="key-settings">
    <div class="keyboard" :style="keymapStyle">
      <div v-for="line in  keymap " class="line">
        <div v-for="key in  line " :class="key.name == null ? 'hidden' : ''">
          <Key :unit-width="key.width" :key-num="key.index" :on-click="k => clickKey(k)"
            :selected="key.index == selectedKey">
            <div v-if="key.index != undefined && store.key_config!.keys[key.index].key_data.length <= 1">
              {{ formatKeys(store.key_config!.keys[key.index].key_data) }}
            </div>
            <div v-else>
              ...
            </div>
          </Key>
        </div>
      </div>
      <transition name="fade">
        <div class="line" v-if="selectedKey != null && store.key_config!.keys[selectedKey!].key_data.length > 1">
          <KeyShow :unit-width="4.2" style="--default-key-font-size: 13px;--default-key-height: 45px;">
            {{ formatKeys(store.key_config!.keys[selectedKey!].key_data) }}
          </KeyShow>
        </div>
      </transition>
    </div>
    <transition name="fade">
      <div v-if="selectedKey != null">
        <div style="max-width: 400px;min-width: 200px;">
          <!-- <n-button type="error" class="badge" @click="set_auto_config">
            {{ $t('need_help') }} </n-button> -->
          <n-form-item :label="$t('dead_zone')" path="dead_zone" label-placement="left" :show-feedback="false">
            <n-input-number v-model:value="store.key_config!.keys[selectedKey!].dead_zone" :min="1" :max="100"
              :placeholder="$t('no_data')">
              <template #suffix>
                %
              </template>
            </n-input-number>
          </n-form-item>
          <n-form-item :label="$t('key_trigger_degree')" path="press_percentage" label-placement="left"
            :show-feedback="false">
            <n-input-number v-model:value="store.key_config!.keys[selectedKey!].press_percentage" :min="1" :max="100"
              :placeholder="$t('no_data')">
              <template #suffix>
                %
              </template>
            </n-input-number>
          </n-form-item>
          <n-form-item :label="$t('key_release_degree')" path="release_percentage" label-placement="left"
            :show-feedback="false">
            <n-input-number v-model:value="store.key_config!.keys[selectedKey!].release_percentage" :min="1" :max="100"
              :placeholder="$t('no_data')">
              <template #suffix>
                %
              </template>
            </n-input-number>
          </n-form-item>
          <div class="apply-to-all">
            <n-button size="small" secondary @click="applyKeySetting">{{ $t('apply_to_all_key') }}</n-button>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<style>
.apply-to-all {
  display: flex;
  justify-content: flex-end;
  margin-top: 10px;
}

.key-settings {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  height: 120px;
}

.keyboard {
  position: relative;
  background: var(--color-background-soft);
  border-radius: 6px;
  /* border: 5px solid var(--color-border); */
  display: flex;
  flex-direction: column;
  height: fit-content;
  width: fit-content;
  padding: 10px;
  outline: 6px solid var(--color-border);
}

.line {
  display: flex;
  flex-direction: row;
}

.hidden {
  visibility: hidden;
}


.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.1s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

<script setup lang="ts">
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { useI18n } from "vue-i18n";
import Key from '@/components/KeyComponents/Key.vue'
import KeyShow from '@/components/KeyComponents/KeyShow.vue'
import KeyModal from '@/components/KeyComponents/KeyModal.vue'
import type { IKeymap } from '@/interface';
import { ref } from 'vue';
import { formatKeys, IsModifierKey, compareArray, detectKeys } from '@/utils'

import { IKeyboard } from "@/apis/meowpadv21se/config";
import { storeToRefs } from 'pinia';

const { t } = useI18n();
const message = useMessage()
const dialog = useDialog()
const store = useDeviceStore()
const s = useStore()
const { key_config } = storeToRefs(store)
const key_cfg = key_config as Ref<IKeyboard>;

import { KeyCode, jsToHid } from '@/keycode';

const props = defineProps<{
  keymap: IKeymap[][]
}>()

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
const isSelectHallKey = ref<boolean>(true)
const showModal = ref(false)
const pressedkeycodes = ref<KeyCode[]>([])


function setKeys(keyNum: number, isHallKeys: boolean) {
  s.key_detection_status = true
  detectKeys(showModal, null, (ks, k) => {
    if (ks.includes(k)) { return false }
    if (IsModifierKey(k)) { return true }
    if (ks.filter((k) => !IsModifierKey(k)).length < 3) {
      return true
    } else {
      message.error(t('most_3_key_error'))
      return false
    }
  }, ks => {
    // clone数组以更新dom
    pressedkeycodes.value = [...ks]
  }, ks => {
    // 先排序
    let keycodes = ks.sort((l, r) => r - l)
    // 对比新的按键和旧的是否相同
    if (keycodes.length > 0) {
      if (isHallKeys) {
        compareArray(key_cfg.value!.hall_keys[keyNum].key_data, keycodes) ? key_cfg.value!.hall_keys[keyNum].key_data = [] : key_cfg.value!.hall_keys[keyNum].key_data = keycodes
      } else {
        compareArray(key_cfg.value!.normal_keys[keyNum].key_data, keycodes) ? key_cfg.value!.normal_keys[keyNum].key_data = [] : key_cfg.value!.normal_keys[keyNum].key_data = keycodes
      }
    }
    s.key_detection_status = false
  })
}



function selectKey(keyNum: number, isHallKeys: boolean) {
  if (keyNum === selectedKey.value && isHallKeys === isSelectHallKey.value) { return }
  selectedKey.value = keyNum;
  isSelectHallKey.value = isHallKeys;
}

function clickKey(k: number, isHallKeys: boolean) {
  if (k === selectedKey.value && isHallKeys === isSelectHallKey.value) {
    setKeys(k, isHallKeys)
  } else {
    selectKey(k, isHallKeys)
    if (!isHallKeys) {
      message.info(t('click_again_to_bind_key'))
    }
  }
}

function applyKeySetting() {
  if (selectedKey.value == null) { return }
  const key = key_cfg.value!.hall_keys[selectedKey.value]
  for (let i = 0; i < key_cfg.value!.hall_keys.length; i++) {
    if (i != selectedKey.value) {
      key_cfg.value!.hall_keys[i].dead_zone = key.dead_zone
      key_cfg.value!.hall_keys[i].press_percentage = key.press_percentage
      key_cfg.value!.hall_keys[i].release_percentage = key.release_percentage
    }
  }
  message.success(t('apply_done'))
}

function getKeyDataLen(index: number, isHallKeys: boolean) {
  if (isHallKeys) {
    return key_cfg.value!.hall_keys[index].key_data.length
  } else {
    return key_cfg.value!.normal_keys[index].key_data.length
  }
}


function getKeyText(index: number, isHallKeys: boolean) {
  if (isHallKeys) {
    return formatKeys(key_cfg.value!.hall_keys[index].key_data)
  } else {
    return formatKeys(key_cfg.value!.normal_keys[index].key_data)
  }
}

</script>


<template>
  <KeyModal v-model:show="showModal" :pressedkeycodes="pressedkeycodes"></KeyModal>
  <div class="key-settings">
    <div class="keyboard" :style="keymapStyle">
      <div v-for="line in keymap" class="line">
        <div v-for="key in line" :class="key.name == null ? 'hidden' : ''">
          <Key :unit-width="key.width" :key-num="key.index" :on-click="k => clickKey(k, true)"
            :selected="key.index == selectedKey && isSelectHallKey">
            <div v-if="key.index != undefined && getKeyDataLen(key.index, true) <= 1">
              {{ getKeyText(key.index, true) }}
            </div>
            <div v-else>
              ...
            </div>
          </Key>
        </div>
        <n-divider vertical />
        <div class="keyboard-side line" :style="keymapStyle">
          <template v-for="key in [0,1,2]">
            <Key :unit-width="0.75" :key-num="key" :on-click="k => clickKey(k, false)" :selected="key == selectedKey && !isSelectHallKey">
              <div v-if="key_cfg!.normal_keys[key].key_data.length <= 1">
                {{ getKeyText(key, false) }}
              </div>
              <div v-else>
                ...
              </div>
            </Key>
          </template>
        </div>
      </div>
      <transition name="fade">
        <div class="line" v-if="selectedKey != null && getKeyDataLen(selectedKey, isSelectHallKey) > 1">
          <KeyShow :unit-width="5.23" style="--default-key-font-size: 13px;--default-key-height: 45px;">
            {{ getKeyText(selectedKey, isSelectHallKey) }}
          </KeyShow>
        </div>
      </transition>
    </div>

    <div class="hall-config-full">
      <transition name="fade">
        <template v-if="selectedKey != null && isSelectHallKey">
          <div class="hall-config">
            <!-- <n-button type="error" class="badge" @click="set_auto_config">
              {{ $t('need_help') }} </n-button> -->
            <n-form-item :label="$t('dead_zone')" path="dead_zone" label-placement="left" :show-feedback="false">
              <n-input-number v-model:value="key_cfg!.hall_keys[selectedKey!].dead_zone" :min="1" :max="100"
                :placeholder="$t('no_data')">
                <template #suffix>
                  %
                </template>
              </n-input-number>
            </n-form-item>
            <n-form-item :label="$t('key_trigger_degree')" path="press_percentage" label-placement="left"
              :show-feedback="false">
              <n-input-number v-model:value="key_cfg!.hall_keys[selectedKey!].press_percentage" :min="1" :max="100"
                :placeholder="$t('no_data')">
                <template #suffix>
                  %
                </template>
              </n-input-number>
            </n-form-item>
            <n-form-item :label="$t('key_release_degree')" path="release_percentage" label-placement="left"
              :show-feedback="false">
              <n-input-number v-model:value="key_cfg!.hall_keys[selectedKey!].release_percentage" :min="1" :max="100"
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
        </template>
        <template v-else>
          <div class="hall-config-empty">
            <n-empty :description="t('select_key_please')">
            </n-empty>
          </div>
        </template>
      </transition>
    </div>
  </div>
</template>

<style scoped>
.hall-config-full {
  width: 320px;
  height: 140px;
  position: relative;
}

.hall-config {
  position: absolute;
  top: 20px;
}

.hall-config-empty {
  position: absolute;
  width: 200px;
  top: 20px;
  left: 75px;
  padding: 20px;
  border-radius: var(--n-border-radius);
  border: 1px solid var(--color-border);
}

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

.n-divider.n-divider--vertical {
    height: auto !important;
    width: 3px;
    border-radius: 1.5px;
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


.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.1s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
<script setup lang="ts">
import { KeyCode } from '@/keycode';
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { compareKeys, formatKeys } from '@/utils'
import { mapping, IKeyMaps } from "@/keymap";
import KeyModalSelect from './KeyModalSelect.vue';
import KeyFrame from './KeyFrame.vue';
import { IMixedKey } from '@/apis';
const store = useStore()

const props = defineProps<{
  keymapping?: IKeyMaps[],
  show: boolean
  pressedkeycodes: KeyCode[]
  currkey: IMixedKey[]
  leaveFunc?: (k?: IMixedKey[]) => void
  enterSelection?: () => void
  keyMapFilter?: (key: IMixedKey) => boolean
  keySelFilter?: (key: IMixedKey[]) => boolean
}>()

const emits = defineEmits(['update:show'])

function leaveFunc(k?: IMixedKey[]) {
  emits('update:show', false)
  if (props.leaveFunc != undefined) {
    props.leaveFunc(k)
  }
  isSelfSelect.value = false
}

var isSelfSelect = ref(false)

function enter_selection() {
  if (isSelfSelect.value === false) {
    isSelfSelect.value = true

    if (props.enterSelection != undefined) {
      props.enterSelection()
    }
  }
}

function leave_selection() {
  if (isSelfSelect.value === true) {
    isSelfSelect.value = false

    let selkeys: IMixedKey[] = []
    for (const [k, v] of binding_keys.value) {
      if (v === true) {
        selkeys.push(k)
      }
    }

    leaveFunc(selkeys)
  }
}

let default_height = ref(55)
let default_width = ref(55)
let default_margin = ref(2)
let default_font_size = ref(15)
let final_key_mapping = props.keymapping ?? mapping


let keymapStyle = ref({
  "--default-key-height": default_height.value + "px",
  "--default-key-width": default_width.value + "px",
  "--default-key-margin": default_margin.value + "px",
  "--default-key-font-size": default_font_size.value + "px",
})

const binding_keys: Ref<Map<IMixedKey, boolean>> = ref(new Map())

watch([() => props.currkey], () => {
  const newMap = new Map()
  for (const m of final_key_mapping) {
    for (const k of m.keys) {
      const isMatched = props.currkey.some(b => compareKeys([b], [k.code]))
      newMap.set(k.code, isMatched)
    }
  }
  binding_keys.value = newMap
}, { immediate: true })

function handleKeyClick(key: IMixedKey) {
  console.log("Key clicked:", key);


  let selkeys: IMixedKey[] = []
  for (const [k, v] of binding_keys.value) {
    if (v === true) {
      selkeys.push(k)
    }
  }
  console.log("Key bound:", selkeys);

  if (props.keySelFilter) {
    if (props.keySelFilter(selkeys)) {
      binding_keys.value.set(key, !binding_keys.value.get(key));
    }
  } else {
    binding_keys.value.set(key, !binding_keys.value.get(key));
  }

}

</script>

<template>
  <transition name="fade">
    <div v-show="show" class="block">
      <div class="background" @click.stop="() => leaveFunc()"></div>
      <div class="block-content">
        <n-card v-if="isSelfSelect" size="medium" :hoverable="true" role="dialog" aria-modal="true">

          <template #header>
            <div class="keyselect-part">
              <span>
                {{ $t('key_modal_select_title') }}
              </span>
              <n-button @click="leave_selection">{{ $t('confirm') }}</n-button>
            </div>
          </template>
          <template #action>
            <n-scrollbar class="key-modal-select-scrollbar" :style="keymapStyle">
              <div class="key-modal-select">
                <n-collapse
                  :default-expanded-names="['BasicKeys', 'ExtendedKeys', 'FunctionKeys', 'MediaKeys', 'MouseKeys']">
                  <n-collapse-item v-for="(v, i) in final_key_mapping" :key="i" :name="v.type" disabled>
                    <template #header>
                      <n-text>{{ $t(v.type) }}</n-text>
                    </template>

                    <KeyFrame class="key-modal-select-key" v-for="k in v.keys" :key="i">
                      <KeyModalSelect :key-value="k.code" :selected="binding_keys.get(k.code)!"
                        :onClick="() => handleKeyClick(k.code)" />
                    </KeyFrame>
                  </n-collapse-item>
                </n-collapse>
              </div>
            </n-scrollbar>
          </template>
        </n-card>
        <n-card v-else size="medium" :hoverable="true" :title="$t('key_modal_title')" role="dialog" aria-modal="true">
          <template #action>
            <div class="keyselect-part">
              <span>
                {{ formatKeys(pressedkeycodes) ?? $t("none") }}
              </span>
              <n-button @click="enter_selection">{{ $t('custom_key') }}</n-button>
            </div>
          </template>
        </n-card>
      </div>
    </div>
  </transition>
</template>

<style lang="scss">
// 这个不能scoped



.key-modal-select-scrollbar {
  // max-height: calc(var(--default-key-height) * 3);

  --key-count: 15;

  max-height: calc(100vh - 200px);
  min-height: calc(var(--default-key-height) * 1.5);
  width: calc((var(--default-key-width) * var(--key-count)) + var(--n-scrollbar-width) + var(--default-key-margin));
  // width: calc((100vw - var(--n-scrollbar-width) - var(--default-key-margin)) / (var(--default-key-width) + var(--default-key-margin)));
}

@media (max-width: 800px) {
  .key-modal-select-scrollbar {
    --key-count: 10;
  }
}



.key-modal-select .n-collapse-item__content-inner {
  display: flex;
  flex-wrap: wrap;
  overflow-wrap: break-word;
  padding-top: 10px !important
}

.key-modal-select .n-collapse {
  --n-title-padding: 10px 0 0 0 !important;
  --n-item-margin: 10px 0 0 0 !important;
}

.block-content {
  .n-card {
    --n-border-radius: 8px !important;
    width: fit-content;
    box-shadow: var(--n-box-shadow);
  }
}
</style>


<style lang="scss" scoped>
.block {
  z-index: 100;
  width: 100vw;
  height: 100vh;
  position: fixed;
  top: 0;
  left: 0;
  display: flex;
  place-items: center;
  place-content: center;

  .keyselect-part {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: auto;
  }


  .background {
    background-color: var(--color-border);
    width: 100vw;
    height: 100vh;
    position: fixed;
    z-index: -1;


  }


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
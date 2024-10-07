<script setup lang="ts">
import { IMixedKey } from "@/apis";
import { KeyCode } from "@/keycode";
import { mapping, IKeyMap, formatKey } from "@/keymap";
import { getErrorMsg, most, splitArray } from "@/utils";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const emit = defineEmits<{
  (e: 'dragkey', key: IMixedKey): void
  (e: 'dragkeyend', key: IMixedKey): void
}>()

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


const keyModifyDraggedKey = ref<IMixedKey | null>(null);

const keyDraggingStyle = ref({
  left: "0px",
  top: "0px",
});



function startDragKey(key: IMixedKey) {
  emit('dragkey', key);
  keyModifyDraggedKey.value = key;
  document.onmousemove = (event: MouseEvent) => {
    keyDraggingStyle.value.left = `${event.pageX - 32}px`;
    keyDraggingStyle.value.top = `${event.pageY - 32}px`;
  }

  document.onmouseup = () => {
    document.onmousemove = null;
    document.onmouseup = null;
    keyModifyDraggedKey.value = null;
    emit('dragkeyend', key);
  }
}


</script>

<template>
  <Teleport to="body">
    <div class="dragging-key" v-if="keyModifyDraggedKey != null" :style="keyDraggingStyle">
      <KeyFrame :style="keymapStyle">
        <component :is="formatKey(keyModifyDraggedKey)" />
      </KeyFrame>
    </div>
  </Teleport>


<n-card :bordered="false" class="key-modify-config-card" size="small">
    <template #action>
    {{ $t('drag_the_key') }}
    </template>
    <n-scrollbar class="key-modify-config-scrollbar" :style="keymapStyle">
    <div class="key-modify-config">
      
      <n-collapse default-expanded-names="BasicKeys" accordion>
        <n-collapse-item v-for="(v, i) in mapping" :key="i" :title="t(v.type)" :name="v.type">
          <KeyFrame class="key-modify-config-key" v-for="k in v.keys" :key="i">
            <KeyModifyOption :KeyValue="k"
                @dragkey="startDragKey" />
          </KeyFrame>
          </n-collapse-item>
        </n-collapse>
    </div>
    </n-scrollbar>
</n-card>

</template>


<style lang="scss">
// 这个不能scoped
.key-modify-config-scrollbar {
  // max-height: calc(var(--default-key-height) * 3);
  max-height: calc((100vh - var(--header-height)) - 500px);
  min-height: calc(var(--default-key-height) * 1.5);
  width: calc((var(--default-key-width) * 15) + var(--n-scrollbar-width) + var(--default-key-margin));
}

.key-modify-config .n-collapse-item__content-inner {
  display: flex;
  flex-wrap: wrap;
  overflow-wrap: break-word;
  padding-top: 10px !important
}

.key-modify-config .n-collapse {
  --n-title-padding: 10px 0 0 0 !important;
  --n-item-margin: 10px 0 0 0 !important;
}

</style>

<style scoped lang="scss">
.dragging-key {
  position: absolute;
  pointer-events: none;
  user-select: none;
  opacity: 0.8;
  z-index: 1000;
  transform: rotate(15deg);
  text-align: center;
}

.key-modify-config-card {
  background-color: var(--color-background-soft);
  border-radius: 8px;
  width: fit-content;
}
</style>
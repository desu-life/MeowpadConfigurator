<script setup lang="tsx">
import { IMixedKey } from '@/apis/meowboard/config';
import Checkmark16Filled from '@vicons/fluent/Checkmark16Filled'
import { IKeyMap } from "@/keymap";
const props = defineProps<{
  KeyValue: IKeyMap
}>()

const emit = defineEmits<{
  (e: 'dragkey', id: IMixedKey): void
}>()

function onKeyStartDrag(event: DragEvent) {
  console.log("StartDrag", props.KeyValue)
  emit('dragkey', props.KeyValue.code)
  event.preventDefault()
}

import { h } from 'vue'

</script>

<template>
  <div
      class="key-modify-option"
      :class="{
      }"
      :draggable="true"
      @click=""
      @dragstart="onKeyStartDrag"
  >
    <div class="label-frame">
      <div class="key-modify-option-text">
        <component :is="KeyValue.key"/>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@mixin transition-bg-color($duration, $timing-function: ease-in-out) {
  transition: background-color $duration $timing-function;
}

.key-modify-option-text {
  text-align: center;
}

.key-modify-option {
  @include transition-bg-color(0.5s);

  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  position: relative;

  height: 100%;
  width: 100%;

  border-radius: inherit;
  cursor: pointer;

  transition: transform 0.2s ease-in-out, background-color 0.2s ease-in-out;

  background-color: transparent;

  font-size: 14px;

  &:hover {
    background-color: var(--color-background-mute);
  }
  // .label-frame {
  //   flex: 1;
  //   margin-left: 8px;
  // }

}
</style>
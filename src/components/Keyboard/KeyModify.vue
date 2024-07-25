<script setup lang="ts">
import emitter from "@/mitt";
import { useMessage } from 'naive-ui'

const props = defineProps({
  keyStr: {
    type: String,
    required: true
  },
  keyStrIndex: {
    type: Number,
    required: true
  }
})

const message = useMessage()

const keyDragged = defineModel("keyDragged", { type: Number, default: -1 });

function onMouseUp(event: MouseEvent) {
  if (keyDragged.value != -1) {
    console.log("Up", keyDragged.value)
    emitter.emit('key-str-modify', { rawIndex: props.keyStrIndex, newValue: keyDragged.value})
  }
}

function onClick(event: MouseEvent) {
  emitter.emit('key-str-modify', { rawIndex: props.keyStrIndex, newValue: 0})
}

</script>

<template>
  <n-popconfirm
    @positive-click="onClick"
    :show-icon="false"
    positive-text="确定"
    :negative-text="null"
  >
  <template #trigger>
  <div
      class="key-modify"
      :class="{
      }"
      :draggable="true"
      @mouseup="onMouseUp($event)"
  >
    <div class="label-frame">
      <div class="key-modify-option-text">
        {{ keyStr }}
      </div>
    </div>
  </div>
  </template>
  清除按键？
  </n-popconfirm>
</template>

<style scoped lang="scss">
@mixin transition-bg-color($duration, $timing-function: ease-in-out) {
  transition: background-color $duration $timing-function;
}

.key-modify {
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

  &:hover {
    background-color: rgba(249, 47, 47, 0.107);
  }
  // .label-frame {
  //   flex: 1;
  //   margin-left: 8px;
  // }

}
</style>
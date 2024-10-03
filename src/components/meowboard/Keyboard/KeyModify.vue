<script setup lang="ts">
import { IMixedKey } from "@/apis/meowboard/config";
import emitter from "@/mitt";
import { c, useMessage } from 'naive-ui'
import { formatKey } from "@/keymap";

const props = defineProps<{
  keyShow: IMixedKey
  keyStrIndex: number
}>()



const message = useMessage()

const keyCurrentIndex = defineModel<number>("keyCurrentIndex", { default: 0 });

function onClick(event: MouseEvent) {
  emitter.emit('key-str-modify', { index: props.keyStrIndex, currentKey: null })
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
      @mouseenter="() => keyCurrentIndex = props.keyStrIndex"
      @mouseleave="() => keyCurrentIndex = -1"
  >
    <div class="label-frame">
      <div class="key-modify-option-text">
        <component :is="formatKey(keyShow)" :key="keyShow" />
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

  font-size: 14px;

  &:hover {
    background-color: rgba(249, 47, 47, 0.107);
  }

  .label-frame {
    text-align: center;
  }

}
</style>
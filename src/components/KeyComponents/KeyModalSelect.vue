<script setup lang="ts">
import { IMixedKey } from "@/apis";
import Checkmark16Filled from '@vicons/fluent/Checkmark16Filled'
import { formatKey } from "@/keymap";

const props = defineProps<{
  keyValue: IMixedKey,
  selected: boolean,
  onClick: () => void,
}>()

// const isSelected = defineModel("isSelected", { type: Boolean, default: false });

function handleClick() {
  // isSelected.value = !isSelected.value;
  props.onClick?.();
}

</script>

<template>
  <div
      class="modal-sel-key"
      :class="{
        selected: selected,
      }"
      @click="handleClick"
  >
    <div class="label-frame">
      <div class="modal-sel-key-text">
        <component :is="formatKey(keyValue)" :key="keyValue" />
      </div>
    </div>
    <n-icon-wrapper
        v-if=" selected "
        class="selected-icon"
        :size="16"
        :border-radius="10"
    >
      <n-icon :size="12" :component="Checkmark16Filled"/>
    </n-icon-wrapper>
  </div>
</template>

<style scoped lang="scss">
@mixin transition-bg-color($duration, $timing-function: ease-in-out) {
  transition: background-color $duration $timing-function;
}

.modal-sel-key {
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

  font-size: 14px;

  transition: transform 0.2s ease-in-out, background-color 0.2s ease-in-out;

  background-color: transparent;

  &.selected {
    background-color: rgba(51, 160, 111, 0.3);
  }

  .selected-icon {
    position: absolute;

    border-radius: 999px;

    right: 4px;
    bottom: 4px;

    background-color: rgba(51, 160, 111, 0.3);

  }

  .label-frame {
    text-align: center;
  }

}
</style>
<script setup lang="ts">
import { IMixedKey } from "@/apis";
import Checkmark16Filled from '@vicons/fluent/Checkmark16Filled'
import { formatKey } from "@/keymap";

const props = defineProps<{
  keyShow: IMixedKey
}>()

const isSelected = defineModel("isSelected", { type: Boolean, default: false });
const isSocdEnabled = defineModel("isSocdEnabled", { type: Boolean, default: false });

function handleClick() {
  if (isSocdEnabled.value === true) {
    isSelected.value = false // Disable selection if SOCD is enabled
    return
  }
  isSelected.value = !isSelected.value;
}

</script>

<template>
  <div
      class="key-socd"
      :class="{
        selected: isSelected,
        socd_enabled: isSocdEnabled && !isSelected
      }"
      @click="handleClick"
  >
    <div class="label-frame">
      <div class="key-socd-text">
        <component :is="formatKey(keyShow)" :key="keyShow" />
      </div>
    </div>
    <n-icon-wrapper
        v-if="isSelected"
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

.key-socd {
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
    transform: scale(0.9);
  }

  &.socd_enabled {
    background-color: rgba(64, 158, 255, 0.37)
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
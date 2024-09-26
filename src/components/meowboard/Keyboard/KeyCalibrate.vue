<script setup lang="ts">
import Checkmark16Filled from '@vicons/fluent/Checkmark16Filled'

defineProps({
  keyStr: {
    type: String,
    required: true
  }
})

const isSelected = defineModel("isSelected", { type: Boolean, default: false });
const isCalibrated = defineModel("isCalibrated", { type: Boolean, default: false });
const isCalibrating = defineModel("isCalibrating", { type: Boolean, default: false });

function handleClick() {
  isSelected.value = !isSelected.value;
}

</script>

<template>
  <div
      class="key-calibrate"
      :class="{
        selected: isSelected,
        calibrated: isCalibrated && !isCalibrating,
        calibrating: isCalibrating
      }"
      @click="handleClick"
  >
    <div class="label-frame">
      <div class="key-calibrate-text">
        {{ keyStr }}
      </div>
    </div>
    <n-icon-wrapper
        v-if=" isSelected "
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

.key-calibrate {
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

  &.calibrated {
    background-color: rgba(85, 145, 80, 0.37)
  }

  &.calibrating {
    border: 1px solid #fffb00;
  }

  .selected-icon {
    position: absolute;

    border-radius: 999px;

    right: 4px;
    bottom: 4px;

    background-color: rgba(51, 160, 111, 0.3);

  }

  .label-frame {
    flex: 1;
    margin-left: 8px;
  }

  .percentage-frame {
    width: 5px;
    height: 40px;

    border: 1px solid var(--color-border);

    display: flex;
    flex-direction: column;
    justify-content: flex-end;

    .percentage {
      width: 5px;
      background-color: var(--vt-c-red-darker);
    }
  }
}
</style>
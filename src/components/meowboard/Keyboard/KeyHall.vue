<script setup lang="ts">
import Checkmark16Filled from '@vicons/fluent/Checkmark16Filled'
import { IKeyConfigBoard } from "@/apis/meowboard/config";
import emitter from "@/mitt";

defineProps({
  keyStr: {
    type: String,
    required: true
  }
})

const isSelected = defineModel("isSelected", { type: Boolean, default: false });
const press_percentage = defineModel("press_percentage", { type: Number, default: 0 });
const release_percentage = defineModel("release_percentage", { type: Number, default: 0 });
const dead_zone = defineModel("dead_zone", { type: Number, default: 0 });
const release_dead_zone = defineModel("release_dead_zone", { type: Number, default: 0 });
const rt_enabled = defineModel("rt_enabled", { type: Boolean, default: false });

function handleClick() {
  isSelected.value = !isSelected.value;
  emitter.emit('key-select')
}

</script>

<template>
  <div
      class="key-select"
      :class="{
        selected: isSelected,
      }"
      @click="handleClick"
  >
    <div class="label-frame">
        <div v-if="rt_enabled" class="rt-label">
          <div id="dz">
            {{ dead_zone }}
          </div>
          <div id="pr">
            {{ press_percentage }} ↓ {{ release_percentage }} ↑
          </div>
        </div>
        <div v-else class="normal-label">
          {{ press_percentage }} ↓
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

.key-select {
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

    .rt-label {
      font-size: 12px;

      #dz {
        font-size: 12px;
      }

      #pr {
        font-size: 11px;
      }
    }

    .normal-label {
      font-size: 15px;
    }
  }

  .percentage-frame {
    width: 5px;
    height: 40px;

    border: 1px solid rgba(78, 78, 78, 0.2);

    display: flex;
    flex-direction: column;
    justify-content: flex-end;

    .percentage {
      width: 5px;
      background-color: rgba(255, 0, 0, 0.4);
    }
  }
}
</style>
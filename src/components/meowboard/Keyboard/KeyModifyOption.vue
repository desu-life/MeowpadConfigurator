<script setup lang="ts">
import Checkmark16Filled from '@vicons/fluent/Checkmark16Filled'

const props = defineProps({
  keyStr: {
    type: String,
    required: true
  },
  KeyValue: {
    type: Number,
    required: true
  }
})

const keyDragged = defineModel("keyDragged", { type: Number, default: -1 });

function onKeyStartDrag(event: MouseEvent) {
  keyDragged.value = props.KeyValue
  console.log("StartDrag", keyDragged.value)
  event.preventDefault()
}

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
        {{ keyStr }}
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

  &:hover {
    background-color: rgba(121, 121, 121, 0.107);
  }
  // .label-frame {
  //   flex: 1;
  //   margin-left: 8px;
  // }

}
</style>
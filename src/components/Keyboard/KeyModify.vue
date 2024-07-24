<script setup lang="ts">
import emitter from "@/mitt";

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

const keyDragged = defineModel("keyDragged", { type: String, default: "no-drag" });

function onMouseUp(event: MouseEvent) {
  if (keyDragged.value != "no-drag") {
    console.log("Up", keyDragged.value)
    emitter.emit('key-str-modify', { rawIndex: props.keyStrIndex, newValue: keyDragged.value})
  }
}

</script>

<template>
  <div
      class="key-modify"
      :class="{
      }"
      :draggable="true"
      @click=""
      @mouseup="onMouseUp($event)"
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
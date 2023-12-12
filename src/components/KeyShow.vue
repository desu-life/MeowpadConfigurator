
<script setup lang="ts">
import { ref } from 'vue';

const props = withDefaults(defineProps<{
  unitWidth?: number,
  unitHeight?: number,
}>(), {
  unitWidth: 1,
  unitHeight: 1,
});

</script>


<template>
  <div class="key" :style="{ '--unit-width': unitWidth, '--unit-height': unitHeight }">
    <i>
      <slot></slot>
    </i>
    <div class="underline"></div>
  </div>
</template>

<style lang="scss" scoped>
.underline {
  position: fixed;
  width: calc(var(--unit-width) * var(--default-key-width) - (var(--default-key-margin) * 2) - 0.3em);
  border-bottom-left-radius: 6px;
  border-bottom-right-radius: 6px;
  border-color: rgba(242, 128, 15, 0.7);
  border-width: 0.15em;
  border-style: solid;
  border-image: initial;
  transform: translateY(calc(var(--default-key-height) - var(--default-key-margin) * 2  - 0.3em));
}

.key {
  display: flex;
  background-color: var(--color-background);
  border-radius: 6px;
  width: calc(var(--unit-width) * var(--default-key-width) - (var(--default-key-margin) * 2));
  height: calc(var(--unit-height) * var(--default-key-height) - (var(--default-key-margin) * 2));
  margin: 2px;
  box-sizing: border-box;
  font-size: var(--default-key-font-size);
  line-height: 1.0;
}


i {
  width: 100%;
  height: 100%;
  font-size: var(--default-key-font-size);
  display: flex;
  place-items: center;
  place-content: center;
  color: var(--color-text);
  overflow-wrap: break-word;
  user-select: none;
  text-align: center;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.1s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

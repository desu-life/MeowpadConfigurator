<script setup lang="ts">
const props = withDefaults(defineProps<{
  unitWidth?: number,
  unitHeight?: number,
}>(), {
  unitWidth: 1,
  unitHeight: 1,
});


</script>

<template>

  <div
      class="frame"
      :style="{ '--unit-width': unitWidth, '--unit-height': unitHeight }"
  >
    <div
        class="key"
    >
      <slot>

      </slot>
    </div>
  </div>

</template>

<style scoped lang="scss">

// .frame.selected {
//   position: absolute;
//   width: calc(var(--unit-width) * var(--default-key-width) - (var(--default-key-margin) * 2) - 0.15em);
//   height: calc(var(--unit-height) * var(--default-key-height) - (var(--default-key-margin) * 2) - 0.15em);
//   // background-color: var(--color-background-transparent) !important;
//   border-radius: 6px;
//   border-color: rgba(242, 128, 15, 0.7);
//   border-width: 0.15em;
//   border-style: solid;
//   border-image: initial;
//   cursor: inherit;
// }

.frame {
  display: flex;
  background-color: var(--color-background-l2);
  border-radius: 8px;
  width: calc(var(--unit-width) * var(--default-key-width) - (var(--default-key-margin) * 2));
  height: calc(var(--unit-height) * var(--default-key-height) - (var(--default-key-margin) * 2));
  margin: var(--default-key-margin);
  box-sizing: border-box;
  font-size: var(--default-key-font-size);
  line-height: 1.0;
  transition: transform 0.2s ease-in-out;
}

// .frame:hover {
//   background-color: var(--color-background-mute);
//   transition: background-color 0.1s;
//   border-radius: 6px;
// }

.key {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  user-select: none;
  border-radius: 8px;
  flex-wrap: nowrap;
  color: var(--color-text);
}


// .frame {
//   .key {
//     display: flex;
//     align-items: center;
//     margin: 2px;
//     justify-content: center;
//     border-radius: 8px;
//     height: 58px;
//     width: inherit - 4px;
//     position: relative;
//     background-color: #1A1A1A;
//   }
// }
</style>
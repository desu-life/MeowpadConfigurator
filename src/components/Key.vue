<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from "vue-i18n";

const props = withDefaults(defineProps<{
  unitWidth?: number,
  unitHeight?: number,
  keyNum?: number,
  onClick?: (number) => void,
  selected?: boolean,
}>(), {
  unitWidth: 1,
  unitHeight: 1,
  selected: false,
});

function click(p: MouseEvent) {
  // delay 0.1s
  setTimeout(() => {
    if (props.onClick)
      props.onClick(props.keyNum);
  }, 100);
}

const { t } = useI18n();
</script>


<template>
  <div class="key" :style="{ '--unit-width': unitWidth, '--unit-height': unitHeight }" @click.stop="click">
    <i>
      <slot></slot>
    </i>
    <n-tooltip trigger="hover" :delay="300" :duration="200">
    <template #trigger>
      <transition name="fade">
        <div v-if="selected" class="selected"></div>
      </transition>
    </template>
    {{ $t('set_key') }}
  </n-tooltip>
  </div>
</template>

<style lang="scss" scoped>

.selected {
  position: absolute;
  width: calc(var(--unit-width) * var(--default-key-width) - (var(--default-key-margin) * 2) - 0.15em);
  height: calc(var(--unit-height) * var(--default-key-height) - (var(--default-key-margin) * 2) - 0.15em);
  // background-color: var(--color-background-transparent) !important;
  border-radius: 6px;
  border-color: rgba(242, 128, 15, 0.7);
  border-width: 0.15em;
  border-style: solid;
  border-image: initial;
  cursor: inherit;
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
  cursor: pointer;

  
}

.key:hover {
  background-color: var(--color-background-mute);
  transition: background-color 0.1s;
  border-radius: 6px;
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

</style>
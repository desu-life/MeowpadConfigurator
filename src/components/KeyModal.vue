<script setup lang="ts">
import { KeyCode } from '@/interface';
import { useStore } from '@/store'
import { formatKeys } from '@/utils'
const store = useStore()

const props = defineProps<{
  show: boolean,
  pressedkeycodes: KeyCode[],
  leaveFunc: () => void
}>()

const emits = defineEmits(['update:show'])

function leaveFunc() {
  emits('update:show', false)
  props.leaveFunc()
}

</script>

<template>
  <transition name="fade">
    <div v-show="show" class="block">
      <div class="background" @click.stop="leaveFunc"></div>
      <n-card  size="medium"  :hoverable="true"
        :title="$t('key_modal_title')" role="dialog" aria-modal="true">
        <template #action>
      {{ formatKeys(pressedkeycodes) }}
    </template>
      </n-card>
    </div>
    <!-- <n-modal :show="show" @update:show="(value) => emits('update:show', value)" transform-origin="center" :close-on-esc="false" :on-after-leave="leaveFunc">
    </n-modal> -->
  </transition>
</template>


<style lang="scss" scoped>
.block {
  z-index: 100;
  width: 100vw;
  height: 100vh;
  position: fixed;
  top: 0;
  left: 0;
  display: flex;
  place-items: center;
  place-content: center;


  .background {
    background-color: var(--color-border);
    width: 100vw;
    height: 100vh;
    position: fixed;
    z-index: -1;
  }

  div {
    // border-radius: 20px;
    --n-border-radius: 10px !important;
    width: fit-content;
    box-shadow: var(--n-box-shadow);
  }
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

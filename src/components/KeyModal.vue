<script setup lang="ts">
import { JitterEliminationMode, KeyCode, LightingMode } from '@/interface';
import { useStore } from '@/store'
import { formatKeys } from '@/utils'
const store = useStore()

defineProps<{
  show: boolean,
  pressedkeycodes: KeyCode[],
  leaveFunc: () => void
}>()

const emits = defineEmits(['update:show'])

</script>

<template>
  <n-modal :show="show" @update:show="(value) => emits('update:show', value)" transform-origin="center" :close-on-esc="false" :on-after-leave="leaveFunc">
    <n-card style="width: fit-content;border-radius: 8px;align-items: center;" :bordered="false"
      :title="$t('key_modal_title')" role="dialog" aria-modal="true">
      {{ formatKeys(pressedkeycodes) }}
    </n-card>
  </n-modal>
</template>

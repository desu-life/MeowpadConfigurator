<script setup lang="ts">
import { KeyCode, IKeyRTStatus, IError } from '@/interface';
import { useStore } from '@/store'
import { getErrorMsg } from '@/utils';
import { invoke } from "@tauri-apps/api/tauri";
import { DataTableColumns } from 'naive-ui';
import { useI18n } from "vue-i18n";

const store = useStore()
const { t } = useI18n();

async function get_hall_value() : Promise<IKeyRTStatus[]> {
    return await invoke("get_hall_value")
}

const debug_data = ref<IKeyRTStatus[]>()


const columns: DataTableColumns<IKeyRTStatus> = [
  {
    title: 'ADC Value',
    key: 'adc_value'
  },
  {
    title: 'Linear ADC Value',
    key: 'linear_value'
  },
  {
    title: 'Press Percentage',
    key: 'press_percentage',
  },
  {
    title: 'Key State',
    key: 'key_state',
  }
]

onMounted(() => {
  const interval = setInterval(async () => {
    try {
      debug_data.value = await get_hall_value();
    } catch (e) {
      console.log(e)
      store.connected = false
      store.status = "error"
      store.status_str = t('connection_broke', { e: getErrorMsg(t, e as IError) })
      store.debug_mode = false
    }
  }, 50)

  onUnmounted(() => {
    clearInterval(interval)
  })
})

</script>

<template>
  <div v-if="debug_data">
    <n-data-table
    pagination-behavior-on-filter="first"
    :columns="columns"
    :data="debug_data"
  />
  </div>
</template>


<style lang="scss" scoped>

</style>

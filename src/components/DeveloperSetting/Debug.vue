<script setup lang="ts">
import { IError, IKeyRTStatus, KeyState } from '@/apis';
import * as api4k from '@/apis/meowpad4k/api'
import * as api3k from '@/apis/meowpad3k/api'
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { getErrorMsg } from '@/utils';
import { DataTableColumns } from 'naive-ui';
import { useI18n } from "vue-i18n";

const store = useStore()
const device = useDeviceStore()
const { t } = useI18n();

const debug_data = ref<IKeyRTStatus[]>()
const btn_state = ref<KeyState>()


const columns: DataTableColumns<IKeyRTStatus> = [
  {
    title: t('adc_value'),
    key: 'adc_value'
  },
  {
    title: t('linear_adc_value'),
    key: 'linear_value'
  },
  {
    title: t('press_percentage'),
    key: 'press_percentage',
  },
  {
    title: t('key_state'),
    key: 'key_state',
  }
]

onMounted(() => {
  const interval = setInterval(async () => {
    try {
      if (!device.connected) {
        return
      }
      if (device.is_4k()) {
        debug_data.value = await api4k.get_debug_value();
      }
      if (device.is_3k()) {
        let v = await api3k.get_debug_value();
        debug_data.value = v.key;
        btn_state.value = v.btn;
      }
    } catch (e) {
      console.log(e)
      device.connected = false
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
  <div v-if="btn_state" class="side-btn">
    侧键状态：{{ btn_state }}
  </div>
</template>


<style lang="scss" scoped>
.side-btn {
  padding: 10px;
}
</style>
<script setup lang="ts">
import { IKeyConfigBoard } from '@/apis/meowboard/config';
import { Toggle } from '@/interface';
import { useDeviceStore } from '@/store/device';
import { useStore } from '@/store/main';
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const device = useDeviceStore()
const store = useStore()
const keyconfig = defineModel<IKeyConfigBoard | null>("keyConfig", { default: null });

const emit = defineEmits<{
  (e: 'update', config: IKeyConfigBoard): void
}>()


const HallFilterSel = [
  {
    value: 0,
    label: t('off')
  },
  {
    value: 1,
    label: t('low')
  },
  {
    value: 2,
    label: t('high')
  },
  {
    value: 3,
    label: t('latency_enjoyer')
  },
]

const ToggleSel = [
  {
    value: Toggle.On,
    label: t('on')
  },
  {
    value: Toggle.Off,
    label: t('off')
  },
]

function onKeyPUpdate(v) {
  if (keyconfig.value == null) return
  keyconfig.value.press_percentage = v
  emit('update', keyconfig.value)
}

function onKeyRUpdate(v) {
  if (keyconfig.value == null) return
  keyconfig.value.release_percentage = v
  emit('update', keyconfig.value)
}

function onKeyDUpdate(v) {
  if (keyconfig.value == null) return
  keyconfig.value.dead_zone = v
  emit('update', keyconfig.value)
}

function onKeyRDUpdate(v) {
  if (keyconfig.value == null) return
  keyconfig.value.release_dead_zone = v
  emit('update', keyconfig.value)
}


let key_config_style = computed(() => {
  let i = 0;
  store.lang === "en" ? i = 0 : i = 9
  return {"right": i + "px"}
});

</script>

<template>
  <n-card :bordered="false" class="keyboard-config-card" content-class="keyboard-config-card-content">
    <n-grid :cols="24" :x-gap="18">
      <n-gi :span="9">
        <n-card :bordered="false" class="device-config-card" content-class="device-config-card-content">
          <div v-if="keyconfig != null" class="device-config-key-config" :style="key_config_style">
            <n-form-item label-placement="left" :label="t('dead_zone')" path="dead_zone" :show-feedback="false">
              <n-input-number v-model:value="keyconfig.dead_zone" :precision="0" :step="1" :min="0" :max="100"
                :on-update:value="v => onKeyDUpdate(v)">
                <template #suffix>
                  %
                </template>
              </n-input-number>
              <n-input-number v-model:value="keyconfig.release_dead_zone" :precision="0" :step="1" :min="0" :max="100"
                :on-update:value="v => onKeyRDUpdate(v)"
                v-if="store.bottom_dz_available == Toggle.On"
                >
                <template #suffix>
                  %
                </template>
              </n-input-number>
            </n-form-item>
            <n-form-item label-placement="left" :label="t('key_trigger_degree')" path="press_percentage" :show-feedback="false">
              <n-input-number v-model:value="keyconfig.press_percentage" :precision="0" :step="1" :min="1" :max="100"
                :on-update:value="v => onKeyPUpdate(v)">
                <template #suffix>
                  %
                </template>
              </n-input-number>
            </n-form-item>
            <n-form-item label-placement="left" :label="t('key_release_degree')" path="release_percentage" :show-feedback="false">
              <n-input-number v-model:value="keyconfig.release_percentage" :precision="0" :step="1" :min="1" :max="100"
                :on-update:value="v => onKeyRUpdate(v)">
                <template #suffix>
                  %
                </template>
              </n-input-number>
            </n-form-item>
          </div>
          <template v-else>
            <n-empty :description="t('select_key_please')">
            </n-empty>
          </template>
        </n-card>
      </n-gi>
      <n-gi :span="15">
        <n-grid :cols="24" :x-gap="9">
          <n-form-item-gi :span="12" path="jitters_elimination_time" :show-feedback="false" class="single-key-config">
            <n-input-number v-model:value="device.jitters_elimination_time" :min="0" :max="50"
              :step="0.5">
              <template #suffix>
                ms
              </template>
            </n-input-number>
            <template #label>
              <n-tooltip trigger="hover" :delay="200">
                <template #trigger>
                  <n-text underline>
                    {{ $t('keyboard_jitters_elimination_time') }}
                  </n-text>
                </template>
                <template #default>
                  $t('jitters_elimination_time_desc')
                </template>
              </n-tooltip>
            </template>
          </n-form-item-gi>
          
          <n-form-item-gi :span="12" path="hall_filter" :show-feedback="false">
            <n-select v-model:value="device.hall_filter" :options="HallFilterSel" />
            <template #label>
              <n-tooltip trigger="hover" :delay="200">
                <template #trigger>
                  <n-text underline>
                    {{ $t('filter_level') }}
                  </n-text>
                </template>
                <template #default>
                  {{ $t('filter_level_desc') }}
                </template>
              </n-tooltip>
            </template>
          </n-form-item-gi>

          <n-form-item-gi :span="12" path="key_proof" :show-feedback="false">
            <n-select v-model:value="device.key_proof" :options="ToggleSel" />
            <template #label>
              <n-tooltip trigger="hover" :delay="200">
                <template #trigger>
                  <n-text underline>
                    {{ $t('key_proof') }}
                  </n-text>
                </template>
                <template #default>
                  {{ $t('key_proof_desc') }}
                </template>
              </n-tooltip>
            </template>
          </n-form-item-gi>
        </n-grid>
      </n-gi>
    </n-grid>
  </n-card>
</template>

<style scoped lang="scss">

.keyboard-config-card {
  --item-padding: 16px;

  background-color: var(--color-background-soft);
  border-radius: 8px;
  width: fit-content;

  .single-key-config {
    padding-bottom: var(--item-padding);
  }
  
  .device-config-card {
    background-color: var(--color-background-soft);
    border-radius: 8px;
    border: 1px solid var(--color-border);
    height: 100%;
  
    .device-config-key-config {
      width: 90%;
      display: flex;
      flex-direction: column;
      gap: 1px;
      position: relative;
    }
  }
}
</style>

<style lang="scss">
// 这个不能scoped
.device-config-key-config .n-form-item-blank {
  width: 210px;
  gap: 1px;
}

.device-config-card-content {
  padding: 0px !important;
  display: flex;
  align-items: center;
  justify-content: center;
  max-width: 350px;

  .n-form-item-label {
    font-size: 13px;
    width: 56px;
  }
}
</style>
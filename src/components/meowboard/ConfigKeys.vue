<script setup lang="ts">
import { IKeyConfigBoard } from '@/apis/meowboard/config';
import { Toggle } from '@/interface';
import { useDeviceStore } from '@/store/device';
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const device = useDeviceStore()
const keyconfig = defineModel<IKeyConfigBoard | null>("keyConfig", { default: null });

const emit = defineEmits<{
  (e: 'update', config: IKeyConfigBoard): void
}>()


const HallFilterSel = [
  {
    value: 0,
    label: "关闭"
  },
  {
    value: 1,
    label: "低"
  },
  {
    value: 2,
    label: "高"
  },
  {
    value: 3,
    label: "延迟享受者"
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



</script>

<template>
  <n-card :bordered="false" class="keyboard-config-card" content-class="keyboard-config-card-content">
    <n-grid :cols="24" :x-gap="18">
      <n-gi :span="9">
        <n-card :bordered="false" class="device-config-card" content-class="device-config-card-content">
          <div v-if="keyconfig != null" class="device-config-key-config">
            <n-form-item label-placement="left" label="死区" path="dead_zone" :show-feedback="false">
              <n-input-number v-model:value="keyconfig.dead_zone" :precision="0" :step="1" :min="0" :max="100"
                :on-update:value="v => onKeyDUpdate(v)">
                <template #suffix>
                  %
                </template>
              </n-input-number>
              <n-input-number v-model:value="keyconfig.release_dead_zone" :precision="0" :step="1" :min="0" :max="100"
                :on-update:value="v => onKeyRDUpdate(v)">
                <template #suffix>
                  %
                </template>
              </n-input-number>
            </n-form-item>
            <n-form-item label-placement="left" label="按下" path="press_percentage" :show-feedback="false">
              <n-input-number v-model:value="keyconfig.press_percentage" :precision="0" :step="1" :min="1" :max="100"
                :on-update:value="v => onKeyPUpdate(v)">
                <template #suffix>
                  %
                </template>
              </n-input-number>
            </n-form-item>
            <n-form-item label-placement="left" label="抬起" path="release_percentage" :show-feedback="false">
              <n-input-number v-model:value="keyconfig.release_percentage" :precision="0" :step="1" :min="1" :max="100"
                :on-update:value="v => onKeyRUpdate(v)">
                <template #suffix>
                  %
                </template>
              </n-input-number>
            </n-form-item>
          </div>
          <template v-else>
            <n-empty description="请先选中按键">
            </n-empty>
          </template>
        </n-card>
      </n-gi>
      <n-gi :span="15">
        <n-grid :cols="24" :x-gap="9">
          <n-form-item-gi :span="12" path="jitters_elimination_time" :show-feedback="false" class="single-key-config">
            <n-input-number v-model:value="device.jitters_elimination_time" placeholder="Input" :min="0" :max="50"
              :step="0.5">
              <template #suffix>
                ms
              </template>
            </n-input-number>
            <template #label>
              <n-tooltip trigger="hover" :delay="200">
                <template #trigger>
                  <n-text underline>
                    消抖时长
                  </n-text>
                </template>
                <template #default>
                  按键的去抖，不会增加按下时的延迟，osu玩家推荐开启
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
                    滤波等级
                  </n-text>
                </template>
                <template #default>
                  用于增加精度，但会有一点延迟，推荐开启
                </template>
              </n-tooltip>
            </template>
          </n-form-item-gi>
          <n-form-item-gi :span="12" label="灯光亮度" path="max_brightness" :show-feedback="false">
            <n-input-number v-model:value="device.max_brightness" placeholder="Input" :min="0" :max="100" :step="1">
              <template #suffix>
                %
              </template>
            </n-input-number>
          </n-form-item-gi>

          <n-form-item-gi :span="12" path="fangwuchu" :show-feedback="false">
            <n-select v-model:value="device.fangwuchu" :options="ToggleSel" />
            <template #label>
              <n-tooltip trigger="hover" :delay="200">
                <template #trigger>
                  <n-text underline>
                    触底防双击
                  </n-text>
                </template>
                <template #default>
                  防止轴体触底震动导致双击，影响底部死区，推荐开启
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
    }
  }
}
</style>

<style lang="scss">
// 这个不能scoped
.device-config-key-config .n-form-item-blank {
  max-width: 250px;
  gap: 1px;
}

.device-config-card-content {
  padding: 0px !important;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
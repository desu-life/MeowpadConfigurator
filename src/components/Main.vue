<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { FormInst } from 'naive-ui'
import { useStore } from '@/store'
import { Keyboard24Regular, Lightbulb24Regular } from '@vicons/fluent'
import { FormValidationStatus } from 'naive-ui/es/form/src/interface';
import FirmwareUpdate from '@/components/FirmwareUpdate.vue'
import ColorSetting from '@/components/ColorSetting.vue'
import DeviceSetting from '@/components/DeviceSetting.vue'
import Debug from '@/components/Debug.vue'
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const store = useStore()
const formRef = ref<FormInst | null>(null)
const message = useMessage()
const configType = ref(0)

const input_status = ref<FormValidationStatus | undefined>(undefined)

async function check_raw_config(value: string): Promise<void> {
  const res: boolean = await invoke("check_raw_config", { config: value })
  if (res) {
    store.can_sync = true
    input_status.value = undefined
  } else {
    store.can_sync = false
    input_status.value = "error"
  }
}


async function handleValidateClick(e: MouseEvent) {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      message.success('Valid')
    } else {
      console.log(errors)
      message.error('Invalid')
    }
  })
}

function switchConfig() {
  if (configType.value === 1) {
    configType.value = 0
  } else {
    configType.value = 1
  }
}


function formatms(value: number | null) {
  if (value === null) return ''
  return `${value} ms`
}

function parsems(value: string) {
  if (value === '') return null
  return parseInt(value)
}

</script>

<template>
  <n-spin :show="store.loading" style="max-width: 640px;">
    <div v-if="store.developer_mode" class="debug">
      <n-alert :title="$t('warning')" type="warning" style="margin-bottom: 10px;">
        {{ $t('developer_warning_2') }} </n-alert>
      <div v-if="store.connected">
        <div v-if="store.debug_mode">
          <Debug></Debug>
        </div>
        <div v-else-if="store.raw_config != undefined">
          <n-input type="textarea" v-model:value="store.raw_config" :on-input="check_raw_config" :status="input_status"
            :autosize="{
              minRows: 3,
              maxRows: 15
            }" />
        </div>
      </div>
    </div>
    <n-form ref="formRef" :label-width="80" label-placement="top" :model="store.config" size="medium"
      style="margin-bottom: 40px;" v-else-if="store.config != undefined && store.connected">
      <div>
        <transition mode="out-in" name="tab-transition">
          <div v-if="configType !== 1">
            <DeviceSetting></DeviceSetting>
          </div>
          <div v-else>
            <ColorSetting></ColorSetting>
          </div>
        </transition>
        <div class="switch-btn">
          <n-tooltip trigger="hover" :delay="400" :duration="200">
            <template #trigger>
              <n-button round type="warning" @click="switchConfig">
                <template #icon>
                  <n-icon>
                    <transition mode="out-in" enter-active-class="animate__animated animate__fadeIn animate__slower"
                      leave-active-class="animate__animated animate__fadeOut" style="animation-duration: 0.15s;">
                      <Keyboard24Regular v-if="configType === 1" />
                      <Lightbulb24Regular v-else />
                    </transition>
                  </n-icon>
                </template>
                <transition mode="out-in" enter-active-class="animate__animated animate__fadeIn animate__slower"
                  leave-active-class="animate__animated animate__fadeOut" style="animation-duration: 0.15s;">
                  <span v-if="configType === 1">{{ $t('back') }}</span>
                  <span v-else>{{ $t('light_config') }}</span>
                </transition>
              </n-button>
            </template>
            {{ $t('switch_tab') }} </n-tooltip>
        </div>
      </div>
    </n-form>
    <div v-else-if="store.need_update_firmware">
      <FirmwareUpdate></FirmwareUpdate>
    </div>
    <n-empty :description="t('no_device')" size="huge" v-else>
    </n-empty>
  </n-spin>
</template>

<style lang="scss">
.switch-btn {
  position: fixed;
  z-index: 10;
  bottom: 40px;
  right: calc(40px);
}

.debug {
  width: 100%;
  height: 100%;
  min-height: 80vh;
}

.tab-transition-leave-active,
.tab-transition-enter-active {
  transition:
    color .3s var(--n-bezier),
    background-color .3s var(--n-bezier),
    transform .2s var(--n-bezier),
    opacity .2s var(--n-bezier);
}

.tab-transition-enter-from,
.tab-transition-leave-to {
  // transform: translateX(32px);
  opacity: 0;
}

// .tab-transition-leave-to,
// .tab-transition-enter-from {
//   transform: translateX(32px);
//   opacity: 0;
// }

.tab-transition-leave-from,
.tab-transition-enter-to {
  // transform: translateX(0);
  opacity: 1;
}
</style>

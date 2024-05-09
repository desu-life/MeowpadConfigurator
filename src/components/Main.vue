<script setup lang="ts">
import { ref } from 'vue'
import { FormInst, UploadCustomRequestOptions } from 'naive-ui'
import { useStore } from '@/store'
import { emit, listen } from '@tauri-apps/api/event'
import { Keyboard24Regular, Lightbulb24Regular } from '@vicons/fluent'
import { ArchiveOutline as ArchiveIcon } from '@vicons/ionicons5'
import { FormValidationStatus } from 'naive-ui/es/form/src/interface';
import FirmwareUpdate from '@/components/FirmwareUpdate.vue'
import ColorSetting from '@/components/meowpad4k/ColorSetting.vue'
import DeviceSetting from '@/components/meowpad4k/DeviceSetting.vue'
import Debug from '@/components/Debug.vue'
import { useI18n } from "vue-i18n";
import * as api from '@/apis/api'
import * as api4k from '@/apis/meowpad4k/api'
import type { UploadFileInfo } from 'naive-ui'
import { IError } from '@/apis'

const { t } = useI18n();
const store = useStore()
const formRef = ref<FormInst | null>(null)
const message = useMessage()
const configType = ref(0)
const downloading = ref(false)
const file_list = ref<UploadFileInfo[]>([])

const input_status = ref<FormValidationStatus | undefined>(undefined)

async function check_raw_config(value: string): Promise<void> {
  const res: boolean = await api4k.check_raw_config(value)
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

const uploadFirmware = async ({
      file,
      data,
      headers,
      withCredentials,
      action,
      onFinish,
      onError,
      onProgress
    }: UploadCustomRequestOptions) => {

      function calc_process(total, current, state) {
        var s1 = 0;
        if (state == 1)
          s1 = current / (total * 1.125)
        else if (state == 2)
          s1 = ((current / 8) + total) / (total * 1.125)
        return Math.ceil(s1 * 100);
      }
      
      try {
        if (file.file) {
          downloading.value = true;
          
          if (file.file.size > 131072 || file.file.size < 32768) {
            message.error(t('firmware-file-error'))
            onError()
            store.iap_connected = false
            store.status = undefined
            store.status_str = t("device_disconnected")
            file_list.value = []
            return
          }

          var buffer = await file.file.arrayBuffer()
          var arr = Array.from(new Uint8Array(buffer))
          var file_length = await api.iap_start(arr)
          const unlisten = await listen('iap_process', (event: any) => {
            var process = event.payload[0];
            var state = event.payload[1];
            
            onProgress({ percent: calc_process(file_length, process, state) })
          });
          await api.iap_flush()
          onFinish()
          file_list.value = []
          message.info(t('upload_firmware_success'))
          store.iap_connected = false
          store.status = undefined
          store.status_str = t("device_disconnected")
        } else {
          onError()
        }
      } catch (e) {
        const es = e as IError
        console.log(es)
        onError()
        store.iap_connected = false
        store.status = undefined
        store.status_str = t("device_disconnected")
        file_list.value = []
      } finally {
        downloading.value = false;
      }
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
      <div v-else-if="store.iap_connected">
        <n-upload
          v-model:file-list="file_list"
          directory-dnd
          :custom-request="uploadFirmware"
          accept=".bin"
          :show-cancel-button="false"
          :show-remove-button="false"
          :max="1"
        >
          <n-upload-dragger v-if="!downloading">
            <div style="margin-bottom: 12px">
              <n-icon size="48" :depth="3">
                <archive-icon />
              </n-icon>
            </div>
            <n-text style="font-size: 16px">
              {{ $t('drag_file') }}
            </n-text>
          </n-upload-dragger>
        </n-upload>
      </div>
    </div>
    <n-form ref="formRef" :label-width="80" label-placement="top" size="medium"
       v-else-if="store.key_config != undefined && store.light_config != undefined && store.connected">
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
@/apis/api@/apis/interface
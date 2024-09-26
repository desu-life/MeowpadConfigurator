<script setup lang="ts">
import { ref } from 'vue'
import { UploadCustomRequestOptions } from 'naive-ui'
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { listen } from '@tauri-apps/api/event'
import { ArchiveOutline as ArchiveIcon } from '@vicons/ionicons5'
import { useI18n } from "vue-i18n";
import * as api from '@/apis/api'
import type { UploadFileInfo } from 'naive-ui'
import { IError } from '@/apis'
import emitter from "@/mitt";

const { t } = useI18n();
const store = useStore()
const message = useMessage()
const downloading = ref(false)
const file_list = ref<UploadFileInfo[]>([])

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

      // if (file.file.size > 131072 || file.file.size < 32768) {
      //   message.error(t('firmware-file-error'))
      //   onError()
      //   store.iap_connected = false
      //   store.status = undefined
      //   store.status_str = t("device_disconnected")
      //   file_list.value = []
      //   return
      // }

      var buffer = await file.file.arrayBuffer()
      var arr = Array.from(new Uint8Array(buffer))
      var file_length = await api.iap_start(arr)
      if (file_length <= 512) {
        message.error(t('firmware-file-error'))
        onError()
        // store.iap_connected = false
        // store.status = undefined
        // store.status_str = t("device_disconnected")
        file_list.value = []
        return
      }
      const unlisten = await listen('iap_process', (event: any) => {
        var process = event.payload[0];
        var state = event.payload[1];

        onProgress({ percent: calc_process(file_length, process, state) })
      });
      await api.iap_flush()
      onFinish()
      file_list.value = []
      message.info(t('upload_firmware_success'))
      emitter.emit('connection-broke', { e: null })
      emitter.emit('header-msg-update', { status: "default", str: t('device_disconnected') })
    } else {
      onError()
    }
  } catch (e) {
    onError()
    emitter.emit('connection-broke', {e: e as IError})
    file_list.value = []
  } finally {
    downloading.value = false;
  }
}

</script>

<template>
    <n-upload v-model:file-list="file_list" directory-dnd :custom-request="uploadFirmware" accept=".bin"
        :show-cancel-button="false" :show-remove-button="false" :max="1">
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
</template>

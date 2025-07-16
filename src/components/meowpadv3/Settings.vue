<script lang="ts">
  export default {
    name: 'SettingsV3',
  }
</script>

<script setup lang="ts">
import { ref } from 'vue'
import { FormInst } from 'naive-ui'
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { Keyboard24Regular, Lightbulb24Regular } from '@vicons/fluent'
import DeviceSetting from './DeviceSetting.vue'
import { useI18n } from "vue-i18n";
import emitter from "@/mitt";
import * as apiv3 from '@/apis/meowpadv3/api'
import { IKeyboard as IKBV3 } from "@/apis/meowpadv3/config";
import { IError } from '@/apis';
import { storeToRefs } from 'pinia';
import { getErrorMsg } from '@/utils';

const { t } = useI18n();
const device = useDeviceStore()
const store = useStore()
const formRef = ref<FormInst | null>(null)
const message = useMessage()
const configType = ref(0)


emitter.on('calibration-key', async () => {
  emitter.emit('loading')
  if (device.is_v3()) {
    try {
      await apiv3.calibration_key(null)
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
    }
  }
  emitter.emit('loaded')
})


emitter.on('get-default-config', async () => {
  emitter.emit('loading')
  if (device.is_v3()) {
    try {
      device.device_config = await apiv3.get_default_key_config()
      device.extract_key_config_v3()
      emitter.emit('header-msg-update', { status: "success", str: t('reset_success') })
      emitter.emit('sync-btn-highlight', { status: true })
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
    }
  }
  emitter.emit('loaded')
})

emitter.on('save-config', async () => {
  emitter.emit('loading')
  if (device.is_v3()) {
    try {
      await apiv3.save_key_config()
      emitter.emit('header-msg-update', { status: "success", str: t('sync_success') })
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
    }
  }
  emitter.emit('loaded')
})


emitter.on('sync-config', async () => {
  emitter.emit('header-loading', { str: t('syncing_config') })
  if (device.is_v3()) {
    const { device_config } = storeToRefs(device)
    const cfg = device_config as Ref<IKBV3>;

    try {
      device.store_key_config_v3()
      await apiv3.set_key_config(cfg.value!)

      for (let i = 0; i < cfg.value!.keys.length; i++) {
        if (cfg.value!.keys[i].dead_zone < 5) {
          store.need_check = true
        }
        if (cfg.value!.keys[i].press_percentage < 3) {
          store.need_check = true
        }
        if (cfg.value!.keys[i].release_percentage < 3) {
          store.need_check = true
        }
      }

      device.extract_key_config_v3()

      if (store.need_check) {
        emitter.emit('header-msg-update', { status: "warning", str: t('applied_config') })
        message.warning(t('check_config_msg'))
      } else {
        emitter.emit('save-config')
      }
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
      emitter.emit('header-msg-update', { status: "error", str: t('sync_error', { e: getErrorMsg(t, e as IError) }) })
    }
  }
  emitter.emit('sync-btn-highlight', { status: false })
  emitter.emit('loaded')
})


</script>

<template>
   <n-form ref="formRef" :label-width="80" label-placement="top" size="medium">
      <div v-if="device.device_config != undefined">
        <DeviceSetting></DeviceSetting>
      </div>
    </n-form>
</template>

<style lang="scss" scoped>
.switch-btn {
  position: fixed;
  z-index: 10;
  bottom: 40px;
  right: calc(40px);
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
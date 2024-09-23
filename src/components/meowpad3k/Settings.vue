<script lang="ts">
  export default {
    name: 'Settings3k',
  }
</script>

<script setup lang="ts">
import { ref } from 'vue'
import { FormInst } from 'naive-ui'
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { Keyboard24Regular, Lightbulb24Regular } from '@vicons/fluent'
import { FormValidationStatus } from 'naive-ui/es/form/src/interface';
import DeviceSetting from './DeviceSetting.vue'
import { useI18n } from "vue-i18n";
import emitter from "@/mitt";
import * as api3k from '@/apis/meowpad3k/api'
import { IKeyboard as IKB3K, ILighting as ILT3K } from "@/apis/meowpad3k/config";
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
  if (device.is_3k()) {
    try {
      await api3k.calibration_key()
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
    }
  }
  emitter.emit('loaded')
})


emitter.on('get-default-config', async () => {
  emitter.emit('loading')
  if (device.is_4k()) {
    try {
      device.key_config = await api3k.get_default_key_config()
      device.extract_key_config_3k()
      device.light_config = await api3k.get_default_light_config()
      device.extract_light_config_3k()
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
  if (device.is_3k()) {
    try {
      await api3k.save_key_config()
      await api3k.save_light_config()
      emitter.emit('header-msg-update', { status: "success", str: t('sync_success') })
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
    }
  }
  emitter.emit('loaded')
})


emitter.on('sync-config', async () => {
  emitter.emit('header-loading', { str: t('syncing_config') })
  if (device.is_3k()) {
    const { key_config } = storeToRefs(device)
    const cfg = key_config;

    try {
      device.store_key_config_3k()
      await api3k.set_key_config(device.key_config! as IKB3K)
      device.store_light_config_3k()
      await api3k.set_light_config(device.light_config! as ILT3K)

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

      device.extract_key_config_3k()
      device.extract_light_config_3k()

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
      <div v-if="device.key_config != undefined && device.light_config != undefined">
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
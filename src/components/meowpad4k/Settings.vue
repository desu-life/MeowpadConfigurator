<script lang="ts">
  export default {
    name: 'Settings4k',
  }
</script>


<script setup lang="ts">
import { ref } from 'vue'
import { FormInst } from 'naive-ui'
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { Keyboard24Regular, Lightbulb24Regular } from '@vicons/fluent'
import { FormValidationStatus } from 'naive-ui/es/form/src/interface';
import ColorSetting from './ColorSetting.vue'
import DeviceSetting from './DeviceSetting.vue'
import { useI18n } from "vue-i18n";
import emitter from "@/mitt";
import * as api4k from '@/apis/meowpad4k/api'
import { IKeyboard as IKB4K, ILighting as ILT4K } from "@/apis/meowpad4k/config";
import { IError } from '@/apis';
import { storeToRefs } from 'pinia';
import { getErrorMsg } from '@/utils';
import { stat } from 'fs';

const { t } = useI18n();
const device = useDeviceStore()
const store = useStore()
const formRef = ref<FormInst | null>(null)
const message = useMessage()
const configType = ref(0)

function switchConfig() {
  if (configType.value === 1) {
    configType.value = 0
  } else {
    configType.value = 1
  }
}

emitter.on('calibration-key', async () => {
  emitter.emit('loading')
  if (device.is_4k()) {
    try {
      await api4k.calibration_key()
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
      device.key_config = await api4k.get_default_key_config()
      device.extract_key_config_4k()
      device.light_config = await api4k.get_default_light_config()
      device.extract_light_config_4k()
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
  if (device.is_4k()) {
    try {
      await api4k.save_key_config()
      await api4k.save_light_config()
      emitter.emit('header-msg-update', { status: "success", str: t('sync_success') })
    } catch (e) {
      emitter.emit('connection-broke', {e: e as IError})
    }
  }
  emitter.emit('loaded')
})


emitter.on('sync-config', async () => {
  emitter.emit('header-loading', { str: t('syncing_config') })
  if (device.is_4k()) {
    const { key_config } = storeToRefs(device)
    const cfg = key_config;

    try {
      device.store_key_config_4k()
      await api4k.set_key_config(device.key_config! as IKB4K)
      device.store_light_config_4k()
      await api4k.set_light_config(device.light_config! as ILT4K)

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

      device.extract_key_config_4k()
      device.extract_light_config_4k()

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
</style>@/store/store
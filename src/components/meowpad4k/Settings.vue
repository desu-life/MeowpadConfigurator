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

const { t } = useI18n();
const device = useDeviceStore()
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

<style lang="scss">
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
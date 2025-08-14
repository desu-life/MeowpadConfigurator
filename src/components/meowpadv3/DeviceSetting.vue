<script setup lang="ts">
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import Keyboard from './Keyboard.vue'
import { h } from 'vue'
import { useI18n } from "vue-i18n";
import { IKeymap, Toggle } from '@/interface';
import { BulbOutline, KeypadOutline } from '@vicons/ionicons5';

import meowpad from '@/meowpad7k.json'
const keymap: IKeymap[][] = meowpad;

import { IKeyboard, LightingMode } from "@/apis/meowpadv3/config";
import { storeToRefs } from 'pinia';

const { t } = useI18n();
const message = useMessage()
const dialog = useDialog()
const store = useDeviceStore()
const { device_config } = storeToRefs(store)
const device_cfg = device_config as Ref<IKeyboard>;


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

  
const LighingModeSel = [
  {
    value: LightingMode.Off,
    label: t('off')
  },
  {
    value: LightingMode.Solid,
    label: t('solid')
  },
  {
    value: LightingMode.RainbowMode,
    label: t('rainbow_gradient_switch')
  },
  {
    value: LightingMode.RainbowFlowMode,
    label: t('rainbow_flow')
  },
  {
    value: LightingMode.PressRadianceMode,
    label: t('ya-gan-mo-shi')
  },
  {
    value: LightingMode.BreatheGlowMode,
    label: t('rainbow_breath_sync')
  },
  {
    value: LightingMode.BreatheGlowAsyncMode,
    label: t('rainbow_breath_switch')
  },
  {
    value: LightingMode.RainDropMode,
    label: t('rain_drop')
  },
  {
    value: LightingMode.TapToGlowMode,
    label: t('press_and_light')
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

function GetToggleSel() {
  return ToggleSel
}
</script>

<template>
    <div class="keyboard-container">
      <div class="key-config">
        <Keyboard :keymap="keymap"></Keyboard>
      </div>
    </div>

  
    <div class="settings-container">
      <!-- 按键设置面板 -->
      <div class="settings-panel key-settings">
        <div class="panel-header">
          <n-icon size="24" class="panel-icon">
            <keypad-outline />
          </n-icon>
          <h3>{{ $t('key_setting') }}</h3>
        </div>
        <div class="panel-content">
          <n-grid :cols="1" :x-gap="18">
            <n-gi>
              <n-form-item :label="$t('keyboard_jitters_elimination_time')" path="jitters_elimination_time">
                <n-input-number v-model:value="device_cfg.jitters_elimination_time" :min="0" :max="50" :step="0.5">
                  <template #suffix>
                    ms
                  </template>
                </n-input-number>
              </n-form-item>
            </n-gi>
            <n-gi>
              <n-form-item :label="$t('key_proof')" path="continuous_report">
                <n-select v-model:value="store.key_proof" :options="GetToggleSel()" />
              </n-form-item>
            </n-gi>
            <n-gi>
              <n-form-item :label="$t('filter_level')" path="continuous_report">
                <n-select v-model:value="store.hall_filter" :options="HallFilterSel" />
              </n-form-item>
            </n-gi>
            <!-- 可以添加更多按键相关的配置在这里 -->
          </n-grid>
        </div>
      </div>

      <!-- 灯光设置面板 -->
      <div class="settings-panel light-settings">
        <div class="panel-header">
          <n-icon size="24" class="panel-icon">
            <bulb-outline />
          </n-icon>
          <h3>{{ $t('light_setting') }}</h3>
        </div>
        <div class="panel-content">
          <n-grid :cols="1" :x-gap="14">
            <n-gi>
              <div class="color-settings">
                <div class="color-pickers">
                  <n-form-item :label="$t('led_color')" path="led_color">
                    <n-color-picker v-model:value="store.led_colors![0]" :show-alpha="false" :modes="['hex']" />
                  </n-form-item>
                </div>
              </div>
            </n-gi>
            <n-gi>
              <n-grid :cols="2" :x-gap="14">
                <n-gi>
                  <n-form-item :label="$t('device_sleep_idle_time')" path="sleep_time">
                    <n-input-number v-model:value="device_cfg!.sleep_timeout" :min="0" :max="65535">
                      <template #suffix>
                        {{ $t('sec') }}
                      </template>
                    </n-input-number>
                  </n-form-item>
                </n-gi>
                <n-gi>
                  <n-form-item :label="$t('maximum_brightness')" path="max_brightness">
                    <n-input-number v-model:value="store.max_brightness" :min="1" :max="60">
                      <template #suffix>
                        {{ '%' }}
                      </template>
                    </n-input-number>
                  </n-form-item>
                </n-gi>
                <n-gi>
                  <n-form-item :label="$t('lighting_mode')" path="lighting_mode">
                    <n-select v-model:value="device_cfg!.led_mode" :options="LighingModeSel" />
                  </n-form-item>
                </n-gi>
              </n-grid>
            </n-gi>
            

          </n-grid>
        </div>
      </div>
    </div>
</template>

<style scoped>
/* .hs_key_setting > .n-form-item-feedback-wrapper {
  display: block;
} */

.key-config {
  width: 100%;
  height: 100%;
}

.keyboard-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  width: 80vw;
  margin-bottom: 40px;
  margin-top: 10px;
}

.settings-container {
  display: flex;
  justify-content: space-between;
  width: 80vw;
  gap: 30px;
}

/* 按键设置面板占三份，灯光设置面板占七份 */
.key-settings {
  flex: 3;
  background: var(--color-background-soft);
  border-radius: 2px;
  border: 1px solid var(--color-border);
  padding: 20px 20px 0px 20px;
  /* box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1); */
  position: relative;
  overflow: hidden;
}

.light-settings {
  flex: 7;
  background: var(--color-background-soft);
  border-radius: 2px;
  border: 1px solid var(--color-border);
  padding: 20px 20px 0px 20px;
  /* box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1); */
  position: relative;
  overflow: hidden;
}


.panel-header {
  display: flex;
  align-items: center;
  margin-bottom: 5px;
  gap: 10px;
  border-bottom: 1px solid var(--n-border-color);
}

.panel-icon {
  color: var(--n-primary-color);
}

.panel-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--n-title-text-color);
}

.panel-content {
  padding: 10px 0;
}

.color-settings {
  margin-top: 5px;
}

.color-settings h4 {
  margin: 0 0 10px 0;
  font-size: 16px;
  font-weight: 500;
  color: var(--n-title-text-color);
}

.color-pickers {
  display: flex;
  flex-wrap: wrap;
  gap: 15px;
}

.color-pickers .n-form-item {
  margin-bottom: 0;
  width: calc(33.33% - 10px);
}

</style>
<script setup lang="ts">
import { IDeviceInfo, IDeviceStatus, IError, IHidDeviceInfo } from "../apis";
import emitter from "@/mitt";
import { useI18n } from "vue-i18n";
import { useStore } from '@/store/main';
import { useDeviceStore } from '@/store/device';
import { useDialog } from 'naive-ui'
import { ArrowForward } from '@vicons/ionicons5'
import { EllipsisHorizontal } from '@vicons/ionicons5'
import * as api from '@/apis/api'
import * as api4k from '@/apis/meowpad4k/api'
import * as api3k from '@/apis/meowpad3k/api'

const { t } = useI18n();
const store = useStore()
const device = useDeviceStore()
const dialog = useDialog()


function check_firmware_version(d: IHidDeviceInfo) {
    return d.firmware_version == store.firmware_versions.get(d.device_name)
}

function fv_tag_type(d: IHidDeviceInfo) {
    if (d.firmware_version == "IAP") {
        return "error"
    }
    if (check_firmware_version(d)) {
        return "info"
    } else {
        return "warning"
    }
}


function connect(d: IHidDeviceInfo) {
    emitter.emit('connect', { device: d })
}

function developer_mode(d: IHidDeviceInfo) {
    dialog.warning({
        title: t('warning'),
        content: t('developer_warning'),
        positiveText: t('confirm'),
        negativeText: t('unconfirm'),
        maskClosable: false,
        onPositiveClick: () => {
            store.developer_mode = true
            emitter.emit('connect', { device: d })
        },
    })
}

async function continue_device_upgrade(d: IHidDeviceInfo) {
    if (d.device_name != "Meowpad") {
        emitter.emit('header-msg-update', { status: "error", str: t('device_not_support') })
        return
    }

    emitter.emit('header-loading', { str: t('connecting') })
    try {
        if (!await api.connect_device(d)) {
            emitter.emit('header-msg-update', { status: "error", str: t('connection_broke', { e: t('device_not_found') }) })
            return
        }

        setTimeout(async () => {
            store.developer_mode = true
            store.iap_connected = true
            emitter.emit('header-msg-update', { status: "warning", str: t('iap_connected') })
        }, 300);
    } catch (e) {
        emitter.emit('connection-broke', { e: e as IError })
    }
}



async function device_update(d: IHidDeviceInfo) {
    if (d.device_name != "Meowpad") {
        emitter.emit('header-msg-update', { status: "error", str: t('device_not_support') })
        return
    }

    emitter.emit('header-loading', { str: t('connecting') })
    try {
        if (!await api.connect_device(d)) {
            emitter.emit('header-msg-update', { status: "error", str: t('connection_broke', { e: t('device_not_found') }) })
            return
        }
        
        dialog.warning({
            title: t('warning'),
            content: t('device_update_warn'),
            positiveText: t('yes'),
            negativeText: t('no'),
            maskClosable: false,
            onPositiveClick: async () => {
                await api4k.erase_firmware()
                setTimeout(async () => {
                    try {
                        await api.connect_iap()
                        // 不管怎么样总之是连上了
                        store.developer_mode = true
                        store.iap_connected = true
                        emitter.emit('header-msg-update', { status: "warning", str: t('iap_connected') })
                    } catch (e) {
                        emitter.emit('connection-broke', { e: e as IError })
                    }
                }, 1000);
            },
            onNegativeClick: () => {
                emitter.emit('header-msg-update', { status: "default", str: t('device_disconnected') })
            },
        })
    } catch (e) {
        emitter.emit('connection-broke', { e: e as IError })
    }
}



</script>

<template>
    <!-- <n-float-button :right="0" :top="0">
        {{ t("developer_mode") }}
    </n-float-button> -->

    <n-card class="device-list-card" content-class="device-list-card-content">
        <template #header>
            {{ $t('select-device') }}
        </template>
        <n-scrollbar style="max-height: 360px">
            <n-list hoverable :show-divider="false" class="device-list">
                <n-list-item v-for="(device, index) in store.device_list" :key="device.device_name">
                    <n-thing :title="t(device.device_name)">
                        <template #description>
                            <n-space size="small" style="margin-top: 4px">
                                <n-tag :bordered="false" :type="fv_tag_type(device)" size="small">
                                    <template v-if="device.firmware_version == 'IAP'">
                                        IAP    
                                    </template>
                                    <template v-else>
                                        v{{ device.firmware_version }}
                                    </template>
                                </n-tag>
                                <n-tag :bordered="false" type="info" size="small"
                                    v-if="device.serial_number != undefined">
                                    {{ device.serial_number }}
                                </n-tag>
                            </n-space>
                        </template>
                        <!-- 这里是主体部分，还不知道写啥 -->
                    </n-thing>
                    <template #suffix>
                        <div v-if="check_firmware_version(device)">
                            <n-button-group>
                                <n-button strong secondary round :disabled="store.loading" @click="developer_mode(device)">
                                    <template #icon>
                                        <n-icon><EllipsisHorizontal /></n-icon>
                                    </template>
                                </n-button>
                                <n-button strong secondary round :disabled="store.loading" @click="connect(device)">
                                    <template #icon>
                                        <n-icon>
                                            <ArrowForward />
                                        </n-icon>
                                    </template>
                                </n-button>
                            </n-button-group>
                        </div>
                        <div v-else>
                            <template v-if="device.firmware_version == 'IAP'">
                                <n-button strong round secondary type="warning" :disabled="store.loading"
                                    @click="continue_device_upgrade(device)">
                                    {{ t("device_continue_upgrade") }}
                                </n-button>
                            </template>
                            <template v-else-if="device.device_name == 'Meowpad'">
                                <n-button strong round secondary type="warning" :disabled="store.loading"
                                    @click="device_update(device)">
                                    {{ t("device_update") }}
                                </n-button>
                            </template>
                            <template v-else>
                                <n-button strong round secondary type="warning" :disabled="store.loading"
                                    @click="developer_mode(device)">
                                    {{ t("developer_mode") }}
                                </n-button>
                            </template>
                        </div>
                    </template>
                </n-list-item>
            </n-list>
        </n-scrollbar>
    </n-card>


</template>

<style scoped lang="scss">
.device-list {
    --n-border-radius: 10px !important;
    
    border-radius: var(--n-border-radius);
    border-color: var(--color-border);
    background-color: var(--color-background-soft)
}

.device-list-card {
    border-radius: 10px;
    border-color: var(--color-border);
}

</style>
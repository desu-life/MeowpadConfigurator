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
import * as apiv2 from '@/apis/meowpadv2/api'
import * as apiv2se from '@/apis/meowpadv2se/api'
import { type } from '@tauri-apps/plugin-os';
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";
import { message } from '@tauri-apps/plugin-dialog';
import { getErrorMsg } from "@/utils";
import { isTauri } from '@/wasm/environment';
import { request_device as wasmRequestDevice } from '@/wasm/pure64';

const { t } = useI18n();
const store = useStore()
const device = useDeviceStore()
const dialog = useDialog()
const router = useRouter()
const isWebMode = ref(!isTauri());


function check_firmware_version(d: IHidDeviceInfo) {
    return store.firmware_versions.get(d.device_name)?.includes(d.firmware_version)
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
    const ostype = type();

    if (d.firmware_version == "IAP") {
        if (d.device_name == "Meowpad") {
            emitter.emit('header-loading', { str: t('connecting') })
            try {
                if (!await api.connect_device(d)) {
                    emitter.emit('header-msg-update', { status: "error", str: t('connection_broke', { e: t('device_not_found') }) })
                    return
                }

                setTimeout(async () => {
                    store.developer_mode = true
                    store.iap_connected = true
                    router.push("/developer-settings")
                    emitter.emit('header-msg-update', { status: "warning", str: t('iap_connected') })
                }, 300);
            } catch (e) {
                emitter.emit('connection-broke', { e: e as IError })
            }
        } else if (d.product_id == 0xFA00) {
            if (store.iap_connected === true) {
                return
            }

            emitter.emit('header-loading', { str: t('connecting') })

            if (!await api.connect_device(d)) {
                emitter.emit('header-msg-update', { status: "error", str: t('connection_broke', { e: t('device_not_found') }) })
                return
            }

            store.iap_connected = true
            emitter.emit('header-msg-update', { status: "warning", str: t('iap_connected') })

            api.update_firmware_call().then(async (status: boolean) => {
                emitter.emit('header-msg-update', { status: "default", str: t('device_disconnected') })
                if (status) {
                    await message(t('firmware_update_success'), { kind: 'info' });
                } else {
                    await message(t('firmware_update_cancel'), { kind: 'error' });
                }

            }).catch((e: IError) => {
                emitter.emit('connection-broke', {e: e as IError})
                emitter.emit('header-msg-update', { status: "error", str: t('update_firmware_error', { e: getErrorMsg(t, e as IError) }) })
            }).finally(() => {
                emit("progress-close")
                store.iap_connected = false
            })
        }
    } else {
        emitter.emit('header-msg-update', { status: "error", str: t('device_not_support') })
    }
}



async function requestWebHIDDevice() {
    if (!navigator.hid) {
        emitter.emit('header-msg-update', { status: "error", str: t('webhid_not_supported') })
        return;
    }
    
    try {
        emitter.emit('header-loading', { str: t('selecting_device') })
        
        // Request device - this will show browser's device picker
        const devices = await navigator.hid.requestDevice({
            filters: [
                { vendorId: 0x5D3E, productId: 0xFB01 }, // Pure64
                { vendorId: 0x5D3E, productId: 0xFB02 }, // Meowpad V3
                { vendorId: 0x5D3E, productId: 0xFB03 }, // Meowpad V2
                { vendorId: 0x5D3E, productId: 0xFB04 }, // Meowpad V2 SE
            ]
        });
        
        if (devices && devices.length > 0) {
            // Refresh device list to show the newly authorized device
            emitter.emit('refresh-device-list');
            emitter.emit('header-msg-update', { status: "success", str: t('device_authorized') })
        } else {
            emitter.emit('header-msg-update', { status: "default", str: t('no_device_selected') })
        }
    } catch (err: any) {
        if (err.name === 'NotFoundError') {
            emitter.emit('header-msg-update', { status: "default", str: t('no_device_selected') })
        } else {
            console.error('[WebHID] Request device failed:', err);
            emitter.emit('header-msg-update', { status: "error", str: t('device_request_failed') })
        }
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
                await apiv2.erase_firmware()
                setTimeout(async () => {
                    try {
                        await api.connect_iap()
                        // 不管怎么样总之是连上了
                        store.developer_mode = true
                        store.iap_connected = true
                        router.push("/developer-settings")
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
    <div v-if="store.device_list.length > 0">
        <n-card class="device-list-card" header-class="device-list-card-header"
            content-class="device-list-card-content">
            <template #header>
                {{ $t('select-device') }}
            </template>
            <n-scrollbar style="max-height: 360px">
                <n-list :show-divider="false" class="device-list">
                    <n-list-item v-for="(device, index) in store.device_list" :key="device.device_name"
                        class="device-list-item">
                        <n-thing :title="device.device_name">
                            <template #description>
                                <n-space size="small" style="margin-top: 4px">
                                    <n-tag :bordered="true" :type="fv_tag_type(device)" size="small">
                                        <template v-if="device.firmware_version == 'IAP'">
                                            IAP
                                        </template>
                                        <template v-else>
                                            v{{ device.firmware_version }}
                                        </template>
                                    </n-tag>
                                    <n-tag :bordered="true" type="info" size="small"
                                        v-if="device.serial_number != undefined">
                                        {{ device.serial_number }}
                                    </n-tag>
                                </n-space>
                            </template>
                        </n-thing>
                        <template #suffix>
                            <div v-if="check_firmware_version(device)">
                                <n-button-group>
                                    <n-button strong secondary :disabled="store.loading"
                                        @click="developer_mode(device)">
                                        <template #icon>
                                            <n-icon>
                                                <EllipsisHorizontal />
                                            </n-icon>
                                        </template>
                                    </n-button>
                                    <n-button strong secondary :disabled="store.loading" @click="connect(device)">
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
                                    <n-button strong secondary type="warning" :disabled="store.loading"
                                        @click="continue_device_upgrade(device)">
                                        {{ t("device_continue_upgrade") }}
                                    </n-button>
                                </template>
                                <template v-else-if="device.device_name == 'Meowpad'">
                                    <n-button strong secondary type="warning" :disabled="store.loading"
                                        @click="device_update(device)">
                                        {{ t("device_update") }}
                                    </n-button>
                                </template>
                                <template v-else>
                                    <n-button strong secondary type="warning" :disabled="store.loading"
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
    </div>

    <div v-else>
        <n-empty :description="t('no_device')" size="huge">
            <template #extra v-if="isWebMode">
                <n-button @click="requestWebHIDDevice" type="primary" size="large">
                    {{ t('request_device') }}
                </n-button>
                <n-text depth="3" style="margin-top: 12px; display: block; font-size: 14px;">
                    {{ t('webhid_permission_hint') }}
                </n-text>
            </template>
        </n-empty>
    </div>
</template>

<style lang="scss"></style>

<style scoped lang="scss">
.device-list-card {
    border-radius: var(--n-border-radius);
    border-color: var(--color-border);
}

.device-list {
    border-radius: var(--n-border-radius);
    background-color: var(--color-background-soft);
}

.device-list-item {
    padding: 12px 18px;
}
</style>
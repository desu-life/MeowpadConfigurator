<script setup lang="ts">
import { useStore } from '@/store'
const store = useStore()
</script>

<template>
    <n-result status="warning" title="警告" size="large">
        <template #footer>
            设备固件版本不匹配，请更新设备固件后再试
            <br>
            你可以前往
            <n-a :href="store.version_info!.latest_firmware_download_url" target="_blank"
                :title="store.version_info!.latest_firmware_download_url">
                这里
            </n-a>
            下载最新的固件，在
            <n-a href="https://info.desu.life/?p=338" target="_blank" title="https://info.desu.life/?p=338">
                这里
            </n-a>
            查看固件更新教程
        </template>
    </n-result>
</template>

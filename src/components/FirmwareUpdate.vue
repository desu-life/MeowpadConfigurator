<script setup lang="ts">
import { useStore } from '@/store'
const store = useStore()
</script>

<template>
    <n-result status="warning" :title="$t('warning')" size="large">
        <template #footer>
            {{ $t('need_update_warning_1') }} <br>
            {{ $t('need_update_warning_2') }} <n-a :href="store.version_info!.latest_firmware_download_url" target="_blank"
                :title="store.version_info!.latest_firmware_download_url">
                {{ $t('here') }} </n-a>
            {{ $t('need_update_warning_3') }} <n-a href="https://info.desu.life/?p=338" target="_blank"
                title="https://info.desu.life/?p=338">
                {{ $t('here') }} </n-a>
            {{ $t('need_update_warning_4') }} </template>
    </n-result>
</template>

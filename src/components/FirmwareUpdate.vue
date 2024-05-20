<script setup lang="ts">
import { useStore } from '@/store/main';
const store = useStore()
</script>

<template>
    <n-result status="warning" :title="$t('warning')" size="large">
        <template #footer>
            {{ $t('need_update_warning_1') }} <br>
            {{ $t('need_update_warning_2') }} <n-a :href="store.latest_firmware_download_url" target="_blank"
                :title="store.latest_firmware_download_url">
                {{ $t('here') }} </n-a>
            {{ $t('need_update_warning_3') }} <n-a href="https://desu.life/#meowpad" target="_blank"
                title="https://desu.life/#meowpad">
                {{ $t('here') }} </n-a>
            {{ $t('need_update_warning_4') }} </template>
    </n-result>
</template>
lk@/store/store
<script setup lang="ts">
import { useDatasetStore } from '@/stores/dataset.store'
import { useSettingStore } from '@/stores/setting.store'
import { wait, waitUntilReady } from '@/utils/wait.util'
import { error, info } from '@tauri-apps/plugin-log'
import { storeToRefs } from 'pinia'
import { onMounted, shallowRef } from 'vue'
import { useRouter } from 'vue-router'
import { locale as tauriLocale } from '@tauri-apps/plugin-os'

const router = useRouter()

const datasetStore = useDatasetStore()
const { syncing } = storeToRefs(datasetStore)

const settingStore = useSettingStore()
const { isReady, locale, onboardingDone } = storeToRefs(settingStore)

const errorMsg = shallowRef<string>()

function handleRetry(): void {
  window.location.reload()
}

onMounted(async () => {
  await waitUntilReady(isReady)
  await info('Initializing app...')

  // Redirect to onboarding if not completed yet
  if (!onboardingDone.value) {
    await info('Onboarding not completed, redirecting to /onboarding')

    const systemLocale = await tauriLocale().then(l => l?.split('-')[0])
    locale.value = systemLocale as any
    await info(`Detected system locale: "${systemLocale}"`)

    router.replace('/onboarding')
    return
  }

  // Sync dataset from GitHub
  await info('Syncing dataset from GitHub...')
  const [err] = await datasetStore.sync()
  if (err) {
    error(err)
    errorMsg.value = err
    return
  }
  await info('Dataset synced successfully')

  // Redirect to home after initialization
  await wait(200)
  await info('Redirecting to /home')
  router.replace('/home')

  await error('Initialization failed:', {})
})
</script>

<template>
  <div class="screen screen--startup flex items-center justify-center h-dvh bg-base-300" data-theme="dark">
    <div class="text-center">
      <!-- Logo -->
      <div class="mb-8">
        <img src="/icon.svg" alt="Kolect Logo" class="w-24 h-24 mx-auto mb-4" />
      </div>

      <!-- Title -->
      <h1 class="text-3xl font-bold mb-4 text-center w-full">{{ $t('screen.startup.title') }}</h1>

      <!-- Loading indicator -->
      <div v-if="syncing" class="space-y-4">
        <div class="loading loading-spinner loading-lg"></div>
        <p class="text-sm opacity-70">{{ $t('screen.startup.initializing') }}</p>
      </div>

      <!-- Error display -->
      <div v-else-if="errorMsg" class="space-y-4">
        <p class="text-error text-sm">{{ errorMsg }}</p>
        <div class="flex gap-2 justify-center">
          <button class="btn btn-sm btn-primary" @click="handleRetry">
            {{ $t('screen.startup.action.retry') }}
          </button>
          <button class="btn btn-sm btn-primary" @click="router.replace('/home')">
            {{ $t('screen.home.title') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

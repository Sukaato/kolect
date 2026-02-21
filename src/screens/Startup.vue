<script setup lang="ts">
import { onMounted, shallowRef } from 'vue'
import { useRouter } from 'vue-router'
import { useDataset } from '@/composables/shared/use-dataset'
import { useDb } from '@/composables/shared/use-db'
import { useI18n } from '@/composables/shared/use-i18n'
import { useLogger } from '@/composables/shared/use-logger'
import { wait } from '@/utils/wait'

const router = useRouter()
const { initDb } = useDb()
const { sync: syncDataset, error: syncError } = useDataset()
const { t } = useI18n()
const { info, error: logError } = useLogger('Startup')

const loading = shallowRef(true)
const error = shallowRef<string | null>(null)

function handleRetry(): void {
  window.location.reload()
}

onMounted(async () => {
  info('Initializing app...')
  try {
    info('Initializing database...')
    // Initialize database
    await initDb()
    info('Database initialized successfully')

    info('Syncing dataset from GitHub...')
    // Sync dataset from GitHub
    await syncDataset()
    if (syncError.value) {
      throw new Error(syncError.value)
    }
    info('Dataset synced successfully')

    info('Waiting 1 second before redirect...')
    // Redirect to home after initialization
    await wait(2000)
    info('Redirecting to /home')
    router.replace('/home') // So user can't go back to Startup screen
  } catch (err) {
    const errorMsg = err instanceof Error ? err.message : String(err)
    logError('Initialization failed:', errorMsg)
    error.value = errorMsg
    loading.value = false
  }

})
</script>

<template>
  <div class="flex items-center justify-center h-dvh bg-base-300" data-theme="dark">
    <div class="text-center">
      <!-- Logo -->
      <div class="mb-8">
        <img src="/icon.svg" alt="Kolect Logo" class="w-24 h-24 mx-auto mb-4" />
      </div>

      <!-- Title -->
      <h1 class="text-3xl font-bold mb-4">{{ t('startup.title') }}</h1>

      <!-- Loading indicator -->
      <div v-if="loading" class="space-y-4">
        <div class="loading loading-spinner loading-lg"></div>
        <p class="text-sm opacity-70">{{ t('startup.initializing') }}</p>
      </div>

      <!-- Error display -->
      <div v-else-if="error" class="space-y-4">
        <p class="text-error text-sm">{{ error }}</p>
        <button class="btn btn-sm btn-primary" @click="handleRetry">
          {{ t('startup.retry') }}
        </button>
        <button class="btn btn-sm btn-primary" @click="router.replace('/home')">
          {{ 'Home' }}
        </button>
      </div>
    </div>
  </div>
</template>

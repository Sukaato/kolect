<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useDataset } from '@/composables/shared/use-dataset'
import { useDb } from '@/composables/shared/use-db'
import { useI18n } from '@/composables/shared/use-i18n'
import { wait } from '@/utils/wait'

const router = useRouter()
const { sync: syncDataset } = useDataset()
const { initDb } = useDb()
const { t } = useI18n()

const loading = ref(true)
const error = ref<string | null>(null)

const handleRetry = () => {
  window.location.reload()
}

onMounted(async () => {
  console.log('[Startup] Initializing app...')
  try {
    console.log('[Startup] Initializing database...')
    // Initialize database
    await initDb()
    console.log('[Startup] Database initialized successfully')

    console.log('[Startup] Syncing dataset from GitHub...')
    // Sync dataset from GitHub
    await syncDataset()
    console.log('[Startup] Dataset synced successfully')

    console.log('[Startup] Waiting 1 second before redirect...')
    // Redirect to home after initialization
    await wait(1000)
  } catch (err) {
    const errorMsg = err instanceof Error ? err.message : String(err)
    console.error('[Startup] Initialization failed:', errorMsg)
    error.value = errorMsg
    loading.value = false
  }
  console.log('[Startup] Redirecting to /home')
  router.push('/home')
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
      </div>
    </div>
  </div>
</template>

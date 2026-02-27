<script setup lang="ts">
import { useDatasetStore } from '@/stores/dataset.store'
import { wait } from '@/utils/wait.util'
import { info, error } from '@tauri-apps/plugin-log'
import { storeToRefs } from 'pinia'
import { onMounted, shallowRef } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const datasetStore = useDatasetStore()
const { error: datasetSyncError } = storeToRefs(datasetStore)

const loading = shallowRef(true)
const errorMsg = shallowRef<string | null>(null)

function handleRetry(): void {
  window.location.reload()
}

onMounted(async () => {
  info('Initializing app...')
  try {

    // Sync dataset from GitHub
    await info('Syncing dataset from GitHub...')
    await datasetStore.sync()
    if (datasetSyncError.value) {
      throw new Error(datasetSyncError.value)
    }
    await info('Dataset synced successfully')

    // Redirect to home after initialization
    await wait(2000)
    await info('Redirecting to /home')
    router.replace('/home') // So user can't go back to Startup screen
  } catch (err) {
    const e = err instanceof Error ? err.message : String(err)
    await error('Initialization failed:', {})
    errorMsg.value = e
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
      <h1 class="text-3xl font-bold mb-4">{{ $t('startup.title') }}</h1>

      <!-- Loading indicator -->
      <div v-if="loading" class="space-y-4">
        <div class="loading loading-spinner loading-lg"></div>
        <p class="text-sm opacity-70">{{ $t('startup.initializing') }}</p>
      </div>

      <!-- Error display -->
      <div v-else-if="errorMsg" class="space-y-4">
        <p class="text-error text-sm">{{ errorMsg }}</p>
        <button class="btn btn-sm btn-primary" @click="handleRetry">
          {{ $t('startup.retry') }}
        </button>
        <button class="btn btn-sm btn-primary" @click="router.replace('/home')">
          {{ 'Home' }}
        </button>
      </div>
    </div>
  </div>
</template>

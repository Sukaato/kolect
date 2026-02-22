<script setup lang="ts">
import { shallowRef } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useLogger } from '@/composables/use-logger';

const logger = useLogger('Settings');
const syncing = shallowRef(false);

async function syncDataset() {
  if (syncing.value) return;

  syncing.value = true;
  try {
    logger.info('Starting dataset sync');
    const updated = await invoke<boolean>('sync_dataset');
    logger.info('Sync completed', { updated });
  } catch (e) {
    const err = e instanceof Error ? e.message : String(e);
    logger.error('Sync failed', err);
  } finally {
    syncing.value = false;
  }
}
</script>

<template>
  <div class="p-4 space-y-4">
    <h1 class="text-2xl font-bold">Settings</h1>

    <div class="card bg-base-200 shadow-sm">
      <div class="card-body">
        <h2 class="card-title">Dataset</h2>
        <p>Synchronize the local dataset with the remote server to get the latest K-pop groups and lightsticks.</p>
        <div class="card-actions justify-end">
          <button class="btn btn-primary" :disabled="syncing" @click="syncDataset">
            <span v-if="syncing" class="loading loading-spinner"></span>
            {{ syncing ? 'Syncing...' : 'Sync Dataset' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

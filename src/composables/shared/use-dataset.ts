import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'

export function useDataset() {
  const syncing = shallowRef(false)
  const updated = shallowRef(false)
  const error = shallowRef<string | null>(null)

  async function sync() {
    syncing.value = true
    error.value = null
    console.log('[useDataset] Starting sync...')
    try {
      const wasUpdated = await invoke<boolean>('sync_dataset')
      console.log('[useDataset] Sync completed, updated:', wasUpdated)
      updated.value = wasUpdated
      return wasUpdated
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : String(e)
      console.error('[useDataset] Sync failed:', errorMsg)
      error.value = errorMsg
      return false
    } finally {
      syncing.value = false
    }
  }

  return { syncing, updated, error, sync }
}

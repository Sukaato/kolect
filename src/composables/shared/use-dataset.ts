import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'

export function useDataset() {
  const syncing = shallowRef(false)
  const updated = shallowRef(false)
  const error = shallowRef<string | null>(null)

  async function sync() {
    syncing.value = true
    error.value = null
    try {
      const wasUpdated = await invoke<boolean>('sync_dataset')
      updated.value = wasUpdated
      return wasUpdated
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : String(e)
      error.value = errorMsg
      console.error('Failed to sync dataset:', errorMsg)
      return false
    } finally {
      syncing.value = false
    }
  }

  return { syncing, updated, error, sync }
}

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
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      syncing.value = false
    }
  }

  return { syncing, updated, error, sync }
}

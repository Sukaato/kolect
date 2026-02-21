import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'
import { useLogger } from './use-logger'

export function useDataset() {
  const syncing = shallowRef(false)
  const updated = shallowRef(false)
  const error = shallowRef<string | null>(null)
  const { info, error: logError } = useLogger('useDataset')

  async function sync() {
    syncing.value = true
    error.value = null
    info('Starting sync...')
    try {
      const wasUpdated = await invoke<boolean>('sync_dataset')
      info('Sync completed, updated:', wasUpdated)
      updated.value = wasUpdated
      return wasUpdated
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : String(e)
      logError('Sync failed:', errorMsg)
      error.value = errorMsg
      return false
    } finally {
      syncing.value = false
    }
  }

  return { syncing, updated, error, sync }
}

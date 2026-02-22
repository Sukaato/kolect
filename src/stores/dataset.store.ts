import { defineStore } from 'pinia'
import { useInvoke } from '@/composables/use-invoke'

export const useDatasetStore = defineStore('dataset', () => {
  const { result, error, loading, invoke: fetch } = useInvoke('get_dataset')
  const { error: syncError, loading: syncing, invoke: sync } = useInvoke<boolean>('sync_dataset')

  return {
    dataset: result,
    loading,
    error,
    syncError,
    syncing,
    sync,
    fetch,
  }
})

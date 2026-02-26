import { defineStore } from 'pinia'
import { useInvoke } from '@/composables/use-invoke'

export const useDatasetStore = defineStore('dataset', () => {
  const { result, error, loading, invoke: fetch } = useInvoke('dataset_get')
  const { error: syncError, loading: syncing, invoke: sync } = useInvoke<boolean>('dataset_sync')

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

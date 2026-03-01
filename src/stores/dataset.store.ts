import { acceptHMRUpdate, defineStore } from 'pinia'
import { shallowRef } from 'vue'
import { useInvoke } from '@/composables/use-invoke'
import type { Dataset } from '@/types/models'

export const useDatasetStore = defineStore('dataset', () => {
  const {
    error: syncError,
    loading: syncing,
    invoke: syncDataset,
  } = useInvoke<boolean>('dataset_sync')
  const { result, error, loading, invoke: getDataset } = useInvoke<Dataset>('dataset_get')

  const fetchedAt = shallowRef<Date>()

  async function sync() {
    const dataset = await syncDataset()
    await getDataset()

    fetchedAt.value = new Date()
    return dataset
  }

  return {
    dataset: result,
    loading,
    error,
    syncError,
    syncing,
    fetchedAt,
    sync,
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDatasetStore, import.meta.hot))
}

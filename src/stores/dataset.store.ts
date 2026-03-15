import { acceptHMRUpdate, defineStore } from 'pinia'
import { readonly, shallowRef } from 'vue'
import { useInvoke } from '@/composables/use-invoke'
import { useToast } from '@/composables/use-toast'

export const useDatasetStore = defineStore('dataset', () => {
  const toast = useToast()

  const { error, loading: syncing, invoke: syncDataset } = useInvoke<boolean>('dataset_sync')

  const lastFetchedAt = shallowRef<Date>()

  async function sync(force: boolean = false) {
    const dataset = await syncDataset({ force })
    toast.success('Dataset synced successfully')

    lastFetchedAt.value = new Date()
    return dataset
  }

  return {
    error: readonly(error),
    syncing: readonly(syncing),
    lastFetchedAt: readonly(lastFetchedAt),
    sync,
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDatasetStore, import.meta.hot))
}

import { acceptHMRUpdate, defineStore } from 'pinia'
import { computed, shallowRef } from 'vue'
import { useInvoke } from '@/composables/use-invoke'
import { useToast } from '@/composables/use-toast'
import { CollectibleKind, type Dataset } from '@/types/dataset'

export const useDatasetStore = defineStore('dataset', () => {
  const toast = useToast()

  const {
    error: syncError,
    loading: syncing,
    invoke: syncDataset,
  } = useInvoke<boolean>('dataset_sync')
  const { result, error, loading, invoke: getDataset } = useInvoke<Dataset>('dataset_get')
  const groups = computed(() => result.value?.groups ?? [])
  const albums = computed(
    () => result.value?.collectibles?.filter(c => c.kind === CollectibleKind.ALBUM) ?? [],
  )
  const lightsticks = computed(
    () => result.value?.collectibles?.filter(c => c.kind === CollectibleKind.LIGHTSTICK) ?? [],
  )

  const fetchedAt = shallowRef<Date>()

  async function sync(force: boolean = false) {
    const dataset = await syncDataset({ force })
    toast.success('Dataset synced successfully')
    await getDataset()

    fetchedAt.value = new Date()
    return dataset
  }

  return {
    dataset: result,
    groups,
    albums,
    lightsticks,
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

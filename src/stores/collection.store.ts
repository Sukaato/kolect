import { defineStore } from 'pinia'
import { useInvoke } from '@/composables/use-invoke'
import type { Collectible } from '@/types/dataset'

export const useCollectionStore = defineStore('collection', () => {
  const { result, error, loading, invoke } = useInvoke<Collectible[]>('collection_get', {
    defaults: [],
  })

  async function load(path?: string) {
    return invoke({ path: path ?? null })
  }

  return {
    collection: result,
    loading,
    error,
    load,
  }
})

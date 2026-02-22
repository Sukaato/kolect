import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { useInvoke } from '@/composables/use-invoke'
import type { CollectionItem, ProductType } from '@/types/collection'

export const useCollectionStore = defineStore('collection', () => {
    const { result, error, loading, invoke } = useInvoke<CollectionItem[]>('get_collection')

  async function load(path?: string) {
    return invoke({ path: path ?? null })
  }

  async function add(productId: string, productType: ProductType, path?: string) {
    return tauriInvoke<string>('add_to_collection', { path: path ?? null, productId, productType }).then((id: string) => {
      load(path)
      return id
    })
  }

  async function remove(id: string, path?: string) {
    return tauriInvoke<boolean>('remove_from_collection', { path: path ?? null, id }).then((success: boolean) => {
      if (success) {
        load(path)
      }
      return success
    })
  }

  return {
    collection: result,
    loading,
    error,
    load,
    add,
    remove,
  }
})

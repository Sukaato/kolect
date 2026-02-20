import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'
import type { CollectionItem } from '@/types'

export function useCollection() {
  const items = shallowRef<Array<CollectionItem>>([])
  const loading = shallowRef(false)

  async function load(path?: string) {
    loading.value = true
    try {
      const res = await invoke<CollectionItem[]>('get_collection', { path: path ?? null })
      items.value = res;
      return items.value
    } finally {
      loading.value = false
    }
  }

  async function add(productId: string, productType: 'ALBUM' | 'LIGHTSTICK', path?: string) {
    return invoke<string>('add_to_collection', { path: path ?? null, productId, productType })
  }

  async function remove(id: string, path?: string) {
    return invoke<boolean>('remove_from_collection', { path: path ?? null, id })
  }

  return { items, loading, load, add, remove }
}

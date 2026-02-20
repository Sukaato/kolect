import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'

export function useDb() {
  const dbPath = shallowRef<string | null>(null)
  const status = shallowRef('idle')

  async function initDb(path?: string) {
    status.value = 'initializing'
    try {
      const p = await invoke('init_db', { path: path ?? null })
      dbPath.value = p as string
      status.value = 'ready'
      return p as string
    } catch (e) {
      status.value = 'error'
      throw e
    }
  }

  return { dbPath, status, initDb }
}

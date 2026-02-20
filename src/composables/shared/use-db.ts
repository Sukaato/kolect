import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'

export function useDb() {
  const dbPath = shallowRef<string | null>(null)
  const status = shallowRef('idle')

  async function initDb(path?: string) {
    console.log('[useDb] Initializing database...')
    status.value = 'initializing'
    try {
      const p = await invoke('init_db', { path: path ?? null })
      console.log('[useDb] Database initialized at:', p)
      dbPath.value = p as string
      status.value = 'ready'
      return p as string
    } catch (e) {
      console.error('[useDb] Database initialization failed:', e)
      status.value = 'error'
      throw e
    }
  }

  return { dbPath, status, initDb }
}

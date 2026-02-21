import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'
import type { DbStatus } from '@/types/db'

export function useDb() {
  const dbPath = shallowRef<string | null>(null)
  const status = shallowRef<DbStatus>('idle')

  async function initDb(path?: string) {
    console.log('[useDb] Initializing database...')
    status.value = 'initializing'
    try {
      const p = await invoke<string>('init_db', { path: path ?? null })
      console.log('[useDb] Database initialized at:', p)
      dbPath.value = p
      status.value = 'ready'
      return p
    } catch (e) {
      console.error('[useDb] Database initialization failed:', e)
      status.value = 'error'
      throw e
    }
  }

  return { dbPath, status, initDb }
}

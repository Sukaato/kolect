import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'
import { DbStatus } from '@/types/db'

export function useDb() {
  const dbPath = shallowRef<string | null>(null)
  const status = shallowRef<DbStatus>(DbStatus.Idle)

  async function initDb(path?: string) {
    console.log('[useDb] Initializing database...')
    status.value = DbStatus.Initializing
    try {
      const p = await invoke('init_db', { path: path ?? null })
      console.log('[useDb] Database initialized at:', p)
      dbPath.value = p as string
      status.value = DbStatus.Ready
      return p as string
    } catch (e) {
      console.error('[useDb] Database initialization failed:', e)
      status.value = DbStatus.Error
      throw e
    }
  }

  return { dbPath, status, initDb }
}

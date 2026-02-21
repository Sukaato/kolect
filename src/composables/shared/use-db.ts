import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'
import type { DbStatus } from '@/types/db'
import { useLogger } from './use-logger'

export function useDb() {
  const dbPath = shallowRef<string | null>(null)
  const status = shallowRef<DbStatus>('idle')
  const { info, error: logError } = useLogger('useDb')

  async function initDb(path?: string) {
    info('Initializing database...')
    status.value = 'initializing'
    try {
      const p = await invoke<string>('init_db', { path: path ?? null })
      info('Database initialized at:', p)
      dbPath.value = p
      status.value = 'ready'
      return p
    } catch (e) {
      logError('Database initialization failed:', e instanceof Error ? e.message : String(e))
      status.value = 'error'
      throw e
    }
  }

  return { dbPath, status, initDb }
}

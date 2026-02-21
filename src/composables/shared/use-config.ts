import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'

export interface LogConfig {
  level: string
}

export interface DatasetConfig {
  url: string
}

export interface Config {
  log: LogConfig
  dataset: DatasetConfig
}

export function useConfig() {
  const config = shallowRef<Config | null>(null)
  const loading = shallowRef(false)
  const error = shallowRef<string | null>(null)

  async function loadConfig() {
    loading.value = true
    error.value = null
    try {
      config.value = await invoke<Config>('get_config')
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : String(e)
      error.value = errorMsg
    } finally {
      loading.value = false
    }
  }

  return {
    config,
    loading,
    error,
    loadConfig,
  }
}

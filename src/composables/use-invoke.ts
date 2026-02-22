import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'

export function useInvoke<T = unknown, E = string>(command: string) {
  const result = shallowRef<T | null>(null)
  const error = shallowRef<E | null>(null)
  const loading = shallowRef(false)

  async function invoke(args?: Record<string, unknown>) {
    loading.value = true
    error.value = null
    result.value = null

    return tauriInvoke<T>(command, args)
      .then(res => {
        result.value = res
        return [null, res] as [null, T]
      })
      .catch(err => {
        const errorMsg = err instanceof Error ? err.message : String(err)
        error.value = errorMsg as E
        return [errorMsg, null] as [E, null]
      })
      .finally(() => {
        loading.value = false
      })
  }

  return {
    result,
    error,
    loading,
    invoke,
  }
}

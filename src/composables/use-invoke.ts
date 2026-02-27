import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import { debug } from '@tauri-apps/plugin-log'
import { shallowRef } from 'vue'
import { wait } from '@/utils/wait.util'
import { useToast } from './use-toast'

interface UseInvokeOptions {
  showErrorToast?: boolean
}

export function useInvoke<T = unknown, E = string>(command: string, options?: UseInvokeOptions) {
  const toast = useToast()

  const result = shallowRef<T | null>(null)
  const error = shallowRef<E | null>(null)
  const loading = shallowRef(false)

  async function invoke(args?: Record<string, unknown>) {
    loading.value = true
    error.value = null
    result.value = null

    await debug(`call ${command} command`)
    return Promise.all([
      wait(2000), // wait at least 2 seconds
      tauriInvoke<T>(command, args)
        .then(res => {
          result.value = res
          return [null, res] as [null, T]
        })
        .catch(err => {
          const errorMsg = err instanceof Error ? err.message : String(err)
          error.value = errorMsg as E

          if (options?.showErrorToast !== false) {
            toast.error(errorMsg)
          }
          return [errorMsg, null] as [E, null]
        }),
    ]).finally(() => {
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

import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import { debug, error as logError } from '@tauri-apps/plugin-log'
import { shallowRef } from 'vue'
import { wait } from '@/utils/wait.util'
import { useToast } from './use-toast'

interface UseInvokeOptions<T = unknown> {
  showErrorToast?: boolean
  defaults: T
}

export function useInvoke<T = unknown, E = string>(command: string, options?: UseInvokeOptions) {
  const toast = useToast()

  const result = shallowRef<T | null>((options?.defaults as T) ?? null)
  const error = shallowRef<E>()
  const loading = shallowRef(false)

  async function invoke(args?: Record<string, unknown>) {
    loading.value = true
    error.value = void 0
    result.value = void 0

    const stringArgs = Object.fromEntries(
      Object.entries(args ?? {}).map(([key, value]) => [key, JSON.stringify(value)]),
    )

    console.debug(`call "${command}" command`, {
      keyValues: stringArgs,
    })
    await debug(`call "${command}" command`, {
      keyValues: stringArgs,
    })
    return Promise.all([
      wait(2000), // wait at least 2 seconds
      tauriInvoke<T>(command, args)
        .then(res => {
          result.value = res
          return [null, res] as [null, T]
        })
        .catch(async err => {
          const errorMsg = err instanceof Error ? err.message : String(err)
          error.value = errorMsg as E

          if (options?.showErrorToast !== false) {
            toast.error(errorMsg)
          }
          console.error(`Error while calling command "${command}":`, err)
          await logError(`Error while calling command "${command}": ${err}`)
          return [errorMsg, null] as [E, null]
        }),
    ])
      .then(([_, data]) => data)
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

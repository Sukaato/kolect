import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import { debug, error as logError } from '@tauri-apps/plugin-log'
import { shallowRef } from 'vue'
import { wait } from '@/utils/wait.util'
import { useToast } from './use-toast'

type UseInvokeOptions<T = unknown> = {
  showErrorToast?: boolean
  defaults?: T
} & InvokeOptions

type InvokeOptions = {
  resetBeforeInvoke?: boolean
  resetOnError?: boolean
}

export function useInvoke<T = unknown, E = string>(command: string, useOptions?: UseInvokeOptions) {
  const toast = useToast()

  const result = shallowRef<T | null>((useOptions?.defaults as T) ?? null)
  const error = shallowRef<E>()
  const loading = shallowRef(false)

  async function invoke(args?: Record<string, unknown>, options?: InvokeOptions) {
    const mergedOptions = {
      ...useOptions,
      ...options,
    }

    loading.value = true
    error.value = void 0
    console.log(mergedOptions)
    if (mergedOptions?.resetBeforeInvoke !== false) {
      console.log('reset data')
      result.value = mergedOptions?.defaults ?? void 0
    }

    console.debug(
      args
        ? `call "${command}" command with args: ${JSON.stringify(args)}`
        : `call "${command}" command`,
    )
    await debug(
      args
        ? `call "${command}" command with args: ${JSON.stringify(args)}`
        : `call "${command}" command`,
    )
    return Promise.all([
      wait(200), // wait at least 2 seconds
      tauriInvoke<T>(command, args)
        .then(res => {
          result.value = res
          return [null, res] as [null, T]
        })
        .catch(async err => {
          const errorMsg = err instanceof Error ? err.message : String(err)
          error.value = errorMsg as E

          if (mergedOptions?.showErrorToast !== false) {
            toast.error(errorMsg)
          }
          if (mergedOptions?.resetOnError) {
            result.value = mergedOptions.defaults ?? void 0
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
    data: result,
    error,
    loading,
    invoke,
  }
}

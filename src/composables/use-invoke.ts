import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import { debug, error as logError } from '@tauri-apps/plugin-log'
import { readonly, shallowRef } from 'vue'

type Commands = TauriInvokeCommands
type CommandName = keyof Commands

type CommandParams<T extends CommandName> = Commands[T]['params']

type CommandResult<T extends CommandName> = Commands[T]['result']
type CommandError<T extends CommandName> = Commands[T]['error']

type UseInvokeOptions<T = unknown, E = unknown> = {
  showErrorToast?: boolean
  onError?: (cause: E) => void
  onSuccess?: (data: T) => void
  defaults?: T
} & InvokeOptions

type InvokeOptions = {
  resetBeforeInvoke?: boolean
  resetOnError?: boolean
}

export function useInvoke<TName extends CommandName>(
  command: TName,
  useOptions?: UseInvokeOptions<CommandResult<TName>, CommandError<TName>>,
) {
  const data = shallowRef<CommandResult<TName> | null>(
    (useOptions?.defaults as CommandResult<TName>) ?? null,
  )
  const error = shallowRef<CommandError<TName> | null>(null)
  const loading = shallowRef(false)

  async function invoke(args: CommandParams<TName>, options?: InvokeOptions) {
    loading.value = true
    error.value = void 0

    const mergedOptions = {
      ...useOptions,
      ...options,
    }

    if (mergedOptions?.resetBeforeInvoke !== false) {
      data.value = mergedOptions?.defaults ?? void 0
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

    return tauriInvoke<CommandResult<TName>>(command, args as Record<string, unknown>)
      .then(result => {
        mergedOptions.onSuccess?.(result)

        data.value = result

        return [null, result] as const
      })
      .catch(async cause => {
        console.error(`Error while calling command "${command}":`, cause)
        await logError(`Error while calling command "${command}": ${cause}`)

        error.value = cause
        mergedOptions.onError?.(cause)

        if (mergedOptions?.resetOnError) {
          data.value = mergedOptions.defaults ?? void 0
        }

        return [cause as string, null] as const
      })
      .finally(() => {
        loading.value = false
      })
  }

  function reset() {
    data.value = useOptions?.defaults ?? null
    error.value = null
    loading.value = false
  }

  return {
    // State
    data,
    error: readonly(error),
    loading: readonly(loading),

    // Actions
    invoke,
    reset,
  }
}

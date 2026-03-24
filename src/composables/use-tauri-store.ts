// use-tauri-store.ts

import { LazyStore } from '@tauri-apps/plugin-store'
import {
  computed,
  type MaybeRefOrGetter,
  onMounted,
  onUnmounted,
  readonly,
  ref,
  type ShallowRef,
  shallowRef,
  toValue,
  type WritableComputedRef,
} from 'vue'

// ----------------------
// Types
// ----------------------

type ValidateFn<T> = (newValue: T, oldValue: T | undefined) => boolean | Promise<boolean>

interface UseTauriStoreOptions<T> {
  validate?: ValidateFn<T>
}

// ----------------------
// Caches (scalabilité)
// ----------------------

const storeCache = new Map<string, LazyStore>()
const keyCache = new Map<string, unknown>()

function getStore(storeName: string): LazyStore {
  if (!storeCache.has(storeName)) {
    storeCache.set(storeName, new LazyStore(storeName))
  }

  // biome-ignore lint/style/noNonNullAssertion: Typescript issue
  return storeCache.get(storeName)!
}

function getCacheKey(storeName: string, key: string) {
  return `${storeName}:${key}`
}

// ----------------------
// Composable
// ----------------------

export function useTauriStore<T>(
  storeName: string,
  key: string,
  defaultValue: T,
  options?: UseTauriStoreOptions<T>,
): {
  state: WritableComputedRef<T>
  isReady: ShallowRef<boolean>
} {
  const cacheKey = getCacheKey(storeName, key)

  // ✅ évite duplication (reactivity + listeners)
  if (keyCache.has(cacheKey)) {
    return keyCache.get(cacheKey) as {
      state: WritableComputedRef<T>
      isReady: ShallowRef<boolean>
    }
  }

  const store = getStore(storeName)

  const internal = ref<T>(defaultValue)
  const isReady = shallowRef(false)

  let unlisten: (() => void) | null = null

  // ----------------------
  // Init (lazy load)
  // ----------------------

  async function init() {
    const stored = await store.get<T>(key)

    if (stored !== null && stored !== undefined) {
      internal.value = stored
    }

    isReady.value = true

    // sync store -> vue
    unlisten = await store.onKeyChange<T>(key, value => {
      internal.value = value
    })
  }

  // ----------------------
  // Setter (vue -> store)
  // ----------------------

  async function set(value: T) {
    const oldValue = internal.value

    // ✅ validation
    if (options?.validate) {
      const valid = await options.validate(value, oldValue)
      if (!valid) return
    }

    internal.value = value
    await store.set(key, value)
  }

  // ----------------------
  // Lifecycle
  // ----------------------

  onMounted(init)

  onUnmounted(() => {
    if (unlisten) unlisten()
  })

  // ----------------------
  // Computed exposé
  // ----------------------

  const state = computed({
    get: () => internal.value,
    set,
  })

  const api = {
    state,
    isReady: readonly(isReady),
  }

  keyCache.set(cacheKey, api)

  return api
}

export function allReady(...states: MaybeRefOrGetter<boolean>[]) {
  return computed(() => states.every(toValue))
}

import { info } from '@tauri-apps/plugin-log'
import { LazyStore } from '@tauri-apps/plugin-store'
import { ref, shallowRef, watch } from 'vue'

export function useTauriStore<Store extends object>(file: string) {
  const store = new LazyStore(file)
  const ready = shallowRef(false)

  async function init() {
    ready.value = true
    info(`Sync '${file}' store`)
  }

  function persistentRef<K extends keyof Store>(key: K, defaultValue: Store[K]) {
    const state = ref<Store[K]>(defaultValue)

    ;(async () => {
      const stored = await store.get<Store[K]>(key as string)
      if (stored !== null && stored !== undefined) {
        state.value = stored
      }
    })()

    watch(state, async val => {
      if (!ready.value) return
      await store.set(key as string, val)
      await store.save()
    })

    return state
  }

  return {
    init,
    persistentRef,
  }
}

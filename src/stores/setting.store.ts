import { acceptHMRUpdate, defineStore } from 'pinia'
import { useTauriStore } from '@/composables/use-tauri-store'

export interface Setting {
  theme: 'system' | 'light' | 'dark'
  locale: 'en' | 'fr'
  includePhotocardInCount: boolean
}
export const useSettingStore = defineStore('setting', () => {
  const { init, persistentRef } = useTauriStore<Setting>('settings.json')

  const theme = persistentRef('theme', 'light')
  const locale = persistentRef('locale', 'fr')
  const includePhotocardCount = persistentRef('includePhotocardInCount', true)

  return {
    // State
    theme,
    locale,
    includePhotocardCount,

    // Actions
    init,
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSettingStore, import.meta.hot))
}

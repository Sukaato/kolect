import { acceptHMRUpdate, defineStore } from 'pinia'
import { useTauriStore } from '@/composables/use-tauri-store'

export interface Setting {
  theme: 'system' | 'light' | 'dark'
  locale: 'en' | 'fr'
  includePhotocardInCount: boolean
  includeExclusiveItems: boolean
  onboardingDone: boolean
}

export const useSettingStore = defineStore('setting', () => {
  const { init, persistentRef } = useTauriStore<Setting>('settings.json')

  const theme = persistentRef('theme', 'system')
  const locale = persistentRef('locale', 'fr')
  const includePhotocardCount = persistentRef('includePhotocardInCount', true)
  const includeExclusiveItems = persistentRef('includeExclusiveItems', false)
  const onboardingDone = persistentRef('onboardingDone', false)

  return {
    // State
    theme,
    locale,
    includePhotocardCount,
    includeExclusiveItems,
    onboardingDone,

    // Actions
    init,
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSettingStore, import.meta.hot))
}

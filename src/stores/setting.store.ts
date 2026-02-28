import { acceptHMRUpdate, defineStore } from 'pinia'
import { useTauriStore } from '@/composables/use-tauri-store'

export interface Setting {
  theme: 'system' | 'light' | 'dark'
  lang: 'en' | 'fr'
}
export const useSettingStore = defineStore('setting', () => {
  const { init, persistentRef } = useTauriStore<Setting>('settings.json')
  const theme = persistentRef('theme', 'light')
  const lang = persistentRef('lang', 'fr')

  return {
    init,
    theme,
    lang,
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSettingStore, import.meta.hot))
}

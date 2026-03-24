import { acceptHMRUpdate, defineStore } from 'pinia'
import { watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { allReady, useTauriStore } from '@/composables/use-tauri-store'

export interface Setting {
  theme: 'system' | 'light' | 'dark'
  locale: 'en' | 'fr'
  includePhotocardInCount: boolean
  includeExclusiveItems: boolean
  onboardingDone: boolean
}

export const useSettingStore = defineStore('setting', () => {
  const i18n = useI18n()

  const { state: theme, isReady: themeReady } = useTauriStore<'system' | 'light' | 'dark'>(
    'settings.json',
    'theme',
    'system',
  )
  const { state: locale, isReady: localeReady } = useTauriStore<'fr' | 'en'>(
    'settings.json',
    'locale',
    'en',
    {
      validate(newValue) {
        return i18n.availableLocales.includes(newValue)
      },
    },
  )
  const { state: includePhotocardInCount, isReady: includePhotocardInCountReady } =
    useTauriStore<boolean>('settings.json', 'includePhotocardInCount', false)
  const { state: includeExclusiveItems, isReady: includeExclusiveItemsReady } =
    useTauriStore<boolean>('settings.json', 'includeExclusiveItems', false)
  const { state: onboardingDone, isReady: onboardingDoneReady } = useTauriStore<boolean>(
    'settings.json',
    'onboardingDone',
    false,
  )

  watch(locale, value => {
    i18n.locale.value = value || 'en'
  })

  const isReady = allReady(
    themeReady,
    localeReady,
    includePhotocardInCountReady,
    includeExclusiveItemsReady,
    onboardingDoneReady,
  )

  return {
    theme,
    locale,
    includePhotocardInCount,
    includeExclusiveItems,
    onboardingDone,
    // Store internal state
    isReady,
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSettingStore, import.meta.hot))
}

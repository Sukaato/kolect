import { shallowRef } from 'vue'
import { useI18n as useVueI18n } from 'vue-i18n'

export function useI18n() {
  const i18n = useVueI18n()
  const locale = shallowRef<'en' | 'fr'>(i18n.locale.value as 'en' | 'fr')

  const setLocale = (newLocale: 'en' | 'fr') => {
    i18n.locale.value = newLocale
    locale.value = newLocale
    // Save preference to localStorage
    localStorage.setItem('locale', newLocale)
  }

  const toggleLocale = () => {
    const newLocale = locale.value === 'en' ? 'fr' : 'en'
    setLocale(newLocale)
  }

  // Load saved preference on mount
  const saved = localStorage.getItem('locale') as 'en' | 'fr' | null
  if (saved && (saved === 'en' || saved === 'fr')) {
    setLocale(saved)
  }

  return {
    locale,
    setLocale,
    toggleLocale,
    t: i18n.t,
  }
}

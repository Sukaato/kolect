import { defineStore } from 'pinia'
import { shallowRef } from 'vue'

type Theme = 'dark' | 'light'
type ThemeSystem = Theme | 'system'

export const useThemeStore = defineStore('theme', () => {
  const theme = shallowRef<ThemeSystem>('system')

  function setTheme(newTheme: ThemeSystem) {
    theme.value = newTheme
    document.documentElement.setAttribute(
      'data-theme',
      newTheme === 'system' ? getSystemTheme() : newTheme,
    )
  }

  function getSystemTheme(): Theme {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  }

  // Initialize theme on load
  setTheme(theme.value)

  return {
    theme,
    setTheme,
  }
})

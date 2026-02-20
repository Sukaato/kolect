import { createI18n } from 'vue-i18n'
import en from '@/locales/en.json'
import fr from '@/locales/fr.json'

type MessageSchema = typeof en

const i18n = createI18n<{ message: MessageSchema }, 'en' | 'fr'>({
  legacy: false,
  locale: 'en',
  fallbackLocale: 'en',
  messages: {
    en,
    fr,
  },
})

export default i18n
export type { MessageSchema }

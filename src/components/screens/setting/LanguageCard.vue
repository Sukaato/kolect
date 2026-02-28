<script setup lang="ts">
import { Setting, useSettingStore } from '@/stores/setting.store'
import { storeToRefs } from 'pinia'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

const languages = {
  en: 'English',
  fr: 'Français'
} as const

const settingStore = useSettingStore()
const { lang } = storeToRefs(settingStore)

const i18n = useI18n()
const selectedLocale = computed(() => languages[lang.value as any as keyof typeof languages] || languages.en)

function setLocale(locale: Setting['lang']) {
  i18n.locale.value = locale
  lang.value = locale
}
</script>

<template>
  <div class="card card-xs bg-base-200 shadow-sm">
    <div class="card-body flex flex-row justify-between">
      <div>
        <h3 class="card-title">{{ $t('screens.setting.sections.appearence.language.title') }}</h3>
        <p class="text-sm text-base-content/60">
          {{ $t('screens.setting.sections.appearence.language.description') }}
        </p>
      </div>

      <div class="content-center w-28 card-actions">
        <div class="w-full flex justify-end">
          <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn m-1 capitalize" aria-haspopup="true"
              :aria-label="`Current theme: ${selectedLocale}`">
              {{ selectedLocale }}
              <svg width="12px" height="12px" class="inline-block h-2 w-2 fill-current opacity-60"
                xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2048 2048">
                <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
              </svg>
            </div>

            <ul tabindex="-1" class="dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl">
              <li>
                <input type="radio" name="theme-dropdown"
                  class="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start" aria-label="English"
                  value="en" @click="setLocale('en')" />
              </li>
              <li>
                <input type="radio" name="theme-dropdown"
                  class="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start" aria-label="Français"
                  value="fr" @click="setLocale('fr')" />
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
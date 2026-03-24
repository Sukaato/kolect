<script setup lang="ts">
import { useSettingStore } from '@/stores/setting.store'
import { storeToRefs } from 'pinia'
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

defineEmits<{
  next: []
}>()

const { locale } = useI18n()

const settingStore = useSettingStore()
const { locale: settingLocale } = storeToRefs(settingStore)

// Detect device locale on mount and pre-select if supported
onMounted(function detectLocale(): void {
  const deviceLang = navigator.language?.split('-')[0]
  if (deviceLang === 'fr' || deviceLang === 'en') {
    applyLocale(deviceLang as 'fr' | 'en')
  }
})

function applyLocale(lang: 'fr' | 'en'): void {
  settingLocale.value = lang
  locale.value = lang
}

function handleSelect(lang: 'fr' | 'en'): void {
  applyLocale(lang)
}
</script>

<template>
  <div class="slide flex flex-col items-center justify-center flex-1 px-8 text-center gap-8">
    <!-- Text -->
    <div class="space-y-3">
      <h2 class="text-2xl font-bold">{{ $t('screen.onboarding.step.language.title') }}</h2>
      <p class="text-base-content/60 text-sm">
        {{ $t('screen.onboarding.step.language.description') }}
      </p>
    </div>

    <!-- Language buttons -->
    <div class="flex flex-col gap-3 w-full max-w-xs">
      <button class="btn btn-lg w-full" :class="settingLocale === 'fr' ? 'btn-primary' : 'btn-outline'"
        @click="handleSelect('fr')">
        Français
      </button>
      <button class="btn btn-lg w-full" :class="settingLocale === 'en' ? 'btn-primary' : 'btn-outline'"
        @click="handleSelect('en')">
        English
      </button>
    </div>

    <!-- Next -->
    <button class="btn btn-primary btn-wide" @click="$emit('next')">
      {{ $t('screen.onboarding.actions.next') }}
    </button>
  </div>
</template>
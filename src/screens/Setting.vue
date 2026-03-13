<script setup lang="ts">
import SettingRow from '@/components/screens/setting/SettingRow.vue';
import SettingSection from '@/components/screens/setting/SettingSection.vue';
import { useDatasetStore } from '@/stores/dataset.store';
import { Setting, useSettingStore } from '@/stores/setting.store';
import { app } from '@tauri-apps/api';
import { onLongPress } from '@vueuse/core';
import { ChevronLeftIcon, DatabaseIcon, GlobeIcon, MoonIcon, PackageIcon, RefreshCwIcon, SunIcon } from 'lucide-vue-next';
import { storeToRefs } from 'pinia';
import { computed, onMounted, shallowRef, useTemplateRef } from 'vue';
import { useI18n } from 'vue-i18n';

const themes: NoInfer<Setting['theme'][]> = ['light', 'dark']
const locales: Record<Setting['locale'], string> = {
  en: 'English',
  fr: 'Français',
}

const i18n = useI18n()

const settingStore = useSettingStore()
const { theme, locale } = storeToRefs(settingStore)

function setLocale(newLocale: Setting['locale']) {
  i18n.locale.value = newLocale
  locale.value = newLocale
}

const datasetStore = useDatasetStore()
const { syncing, fetchedAt } = storeToRefs(datasetStore)
const lastSync = computed(() => {
  return Intl.DateTimeFormat(locale.value, {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  }).format(fetchedAt.value) ?? 'Never'
})

const refreshButtonRef = useTemplateRef('refresh-btn')
onLongPress(refreshButtonRef.value, async () => {
  await datasetStore.sync(true)
})

const appVersion = shallowRef<string>()

onMounted(async () => {
  appVersion.value = await app.getVersion()
})
</script>

<template>
  <div class="screen--setting min-h-screen relative pb-10">

    <!-- Header -->
    <div class="sticky top-0 z-10 bg-base-100/80 backdrop-blur-md border-b border-base-300">
      <div class="flex items-center gap-3 px-4 py-4 max-w-lg mx-auto">
        <button @click="$emit('back')" class="btn btn-ghost btn-sm btn-circle">
          <ChevronLeftIcon class="w-5 h-5" />
        </button>
        <h1 class="text-xl font-bold tracking-tight">{{ $t('screens.setting.title') }}</h1>
      </div>
    </div>

    <div class="max-w-lg mx-auto px-4 pt-6 space-y-5">

      <!-- Appearance -->
      <SettingSection :title="$t('screens.setting.sections.appearence.title')">
        <SettingRow :label="$t('screens.setting.sections.appearence.theme.title')"
          :icon="theme === 'dark' ? MoonIcon : SunIcon" icon-color="text-primary">
          <select v-model="theme" class="select select-sm select-bordered w-36 text-sm">
            <option v-for="theme in themes" :key="theme" :value="theme">
              {{ $t(`screens.setting.sections.appearence.theme.values.${theme}`) }}
            </option>
          </select>
        </SettingRow>


        <SettingRow :label="$t('screens.setting.sections.appearence.language.title')" :icon="GlobeIcon"
          icon-color="text-secondary">
          <select v-model="locale" class="select select-sm select-bordered w-36 text-sm" @change="setLocale($event.target.value as Setting['locale'])">
            <option v-for="[locale, label] in Object.entries(locales)" :key="locale" :value="locale">
              {{ label }}
            </option>
          </select>
        </SettingRow>

      </SettingSection>

      <!-- Data -->
      <SettingSection :title="$t('screens.setting.sections.data.title')">
        <SettingRow :label="$t('screens.setting.sections.data.dataset.title')"
          :sublabel="lastSync" :icon="DatabaseIcon" icon-color="text-info">
          <button ref="refresh-btn" class="btn btn-sm btn-outline btn-warning gap-1.5" :disabled="syncing" @click="datasetStore.sync()">
            <RefreshCwIcon class="w-3.5 h-3.5" />
            {{ $t('screens.setting.sections.data.dataset.actions.refresh') }}
          </button>
        </SettingRow>
      </SettingSection>

      <!-- About -->
      <SettingSection :title="$t('screens.setting.sections.about.title')">
        <SettingRow :label="$t('screens.setting.sections.about.version.title')" :icon="PackageIcon"
          icon-color="text-base-content/40">
          <span class="text-sm font-mono text-primary">{{ appVersion }}</span>
        </SettingRow>
      </SettingSection>
    </div>
  </div>
</template>

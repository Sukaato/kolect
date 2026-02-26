<script setup lang="ts">
import DatasetCard from '@/components/screens/setting/DatasetCard.vue';
import LanguageCard from '@/components/screens/setting/LanguageCard.vue';
import ThemeCard from '@/components/screens/setting/ThemeCard.vue';
</script>

<template>
  <div class="screen--setting min-h-screen">
    <header class="navbar bg-base-100 shadow-sm sticky top-0 z-10">
      <div class="flex-1">
        <h1 class="text-left">{{ $t('screens.setting.title') }}</h1>
      </div>
    </header>

    <div class="p-4 space-y-4 min-h-screen ">
      <!-- Appearance Section -->
      <div class="space-y-1">
        <h2 class="text-lg font-semibold">{{ $t('screens.setting.sections.appearence.title') }}</h2>

        <ThemeCard />
        <LanguageCard />
      </div>

      <!-- Dangerous Section -->
      <div class="space-y-1">
        <h2 class="text-lg font-semibold">{{ $t('screens.setting.sections.data.title') }}</h2>

        <DatasetCard />
      </div>
    </div>
  </div>
</template>

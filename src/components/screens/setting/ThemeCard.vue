<script setup lang="ts">
import ThemeController from './ThemeController.vue';

</script>

<template>
  <div class="card card-xs bg-base-200 shadow-sm">
    <div class="card-body flex flex-row justify-between">
      <div>
        <h3 class="card-title">{{ $t('screens.setting.sections.appearence.theme.title') }}</h3>
        <p class="text-sm text-base-content/60">
          {{ $t('screens.setting.sections.appearence.theme.description') }}
        </p>
      </div>

      <div class="content-center w-28 card-actions">
        <div class="w-full flex justify-end">
          <ThemeController />
        </div>
      </div>
    </div>
  </div>
</template>
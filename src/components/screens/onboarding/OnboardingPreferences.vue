<script setup lang="ts">
import { useSettingStore } from '@/stores/setting.store'
import { storeToRefs } from 'pinia'

defineEmits<{
  next: []
}>()

const settingStore = useSettingStore()
const { includeExclusiveItems, includePhotocardInCount } = storeToRefs(settingStore)
</script>

<template>
  <div class="slide flex flex-col items-center justify-center flex-1 px-8 gap-8">
    <!-- Text -->
    <div class="space-y-3 text-center">
      <h2 class="text-2xl font-bold">{{ $t('screen.onboarding.step.preferences.title') }}</h2>
      <p class="text-base-content/60 text-sm">
        {{ $t('screen.onboarding.step.preferences.description') }}
      </p>
    </div>

    <!-- Toggles -->
    <div class="flex flex-col gap-4 w-full max-w-sm">

      <!-- Exclusive items -->
      <label class="flex items-start gap-4 cursor-pointer bg-base-200 rounded-2xl p-4">
        <input v-model="includeExclusiveItems" type="checkbox" class="toggle toggle-primary mt-0.5 shrink-0" />
        <div class="space-y-1">
          <p class="font-medium text-sm leading-snug">
            {{ $t('screen.onboarding.step.preferences.exclusive_items.label') }}
          </p>
          <p class="text-xs text-base-content/50 leading-relaxed">
            {{ $t('screen.onboarding.step.preferences.exclusive_items.hint') }}
          </p>
        </div>
      </label>

      <!-- Photocards -->
      <label class="flex items-start gap-4 cursor-pointer bg-base-200 rounded-2xl p-4">
        <input v-model="includePhotocardInCount" type="checkbox" class="toggle toggle-primary mt-0.5 shrink-0" />
        <div class="space-y-1">
          <p class="font-medium text-sm leading-snug">
            {{ $t('screen.onboarding.step.preferences.photocards.label') }}
          </p>
          <p class="text-xs text-base-content/50 leading-relaxed">
            {{ $t('screen.onboarding.step.preferences.photocards.hint') }}
          </p>
        </div>
      </label>

    </div>

    <!-- Next -->
    <button class="btn btn-primary btn-wide" @click="$emit('next')">
      {{ $t('screen.onboarding.actions.next') }}
    </button>
  </div>
</template>
<script setup lang="ts">
import { Sun, Moon, Monitor } from 'lucide-vue-next'
import { useSettingStore } from '@/stores/setting.store'
import { storeToRefs } from 'pinia'

defineEmits<{
  next: []
}>()

const settingStore = useSettingStore()
const { theme } = storeToRefs(settingStore)

function handleSelect(value: 'system' | 'light' | 'dark'): void {
  theme.value = value
}
</script>

<template>
  <div class="slide flex flex-col items-center justify-center flex-1 px-8 text-center gap-8">
    <!-- Text -->
    <div class="space-y-3">
      <h2 class="text-2xl font-bold">{{ $t('screen.onboarding.step.theme.title') }}</h2>
      <p class="text-base-content/60 text-sm">
        {{ $t('screen.onboarding.step.theme.description') }}
      </p>
    </div>

    <!-- Theme buttons -->
    <div class="flex flex-col gap-3 w-full max-w-xs">
      <button class="btn btn-lg w-full gap-3" :class="theme === 'system' ? 'btn-primary' : 'btn-outline'"
        @click="handleSelect('system')">
        <Monitor :size="20" />
        {{ $t('screen.onboarding.step.theme.system') }}
      </button>
      <button class="btn btn-lg w-full gap-3" :class="theme === 'light' ? 'btn-primary' : 'btn-outline'"
        @click="handleSelect('light')">
        <Sun :size="20" />
        {{ $t('screen.onboarding.step.theme.light') }}
      </button>
      <button class="btn btn-lg w-full gap-3" :class="theme === 'dark' ? 'btn-primary' : 'btn-outline'"
        @click="handleSelect('dark')">
        <Moon :size="20" />
        {{ $t('screen.onboarding.step.theme.dark') }}
      </button>
    </div>

    <!-- Next -->
    <button class="btn btn-primary btn-wide" @click="$emit('next')">
      {{ $t('screen.onboarding.actions.next') }}
    </button>
  </div>
</template>
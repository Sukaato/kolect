<script setup lang="ts">
defineEmits<{
  next: []
}>()
</script>

<template>
  <div class="slide flex flex-col items-center justify-center flex-1 px-8 text-center gap-6">
    <!-- Logo -->
    <img src="/icon.svg" alt="Kolect" class="w-24 h-24" />

    <!-- Text -->
    <div class="space-y-3">
      <h1 class="text-3xl font-bold">{{ $t('screen.onboarding.step.welcome.title') }}</h1>
      <p class="text-base-content/60 text-base leading-relaxed">
        {{ $t('screen.onboarding.step.welcome.description') }}
      </p>
    </div>

    <!-- CTA -->
    <button class="btn btn-primary btn-wide mt-4" @click="$emit('next')">
      {{ $t('screen.onboarding.actions.next') }}
    </button>
  </div>
</template>
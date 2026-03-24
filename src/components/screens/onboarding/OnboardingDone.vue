<script setup lang="ts">
defineEmits<{
  finish: []
}>()
</script>

<template>
  <div class="slide flex flex-col items-center justify-center flex-1 px-8 text-center gap-6">
    <!-- Illustration -->
    <div class="text-6xl select-none">🎉</div>

    <!-- Text -->
    <div class="space-y-3">
      <h2 class="text-2xl font-bold">{{ $t('screen.onboarding.step.done.title') }}</h2>
      <p class="text-base-content/60 text-base leading-relaxed">
        {{ $t('screen.onboarding.step.done.description') }}
      </p>
    </div>

    <!-- CTA -->
    <button class="btn btn-primary btn-wide mt-4" @click="$emit('finish')">
      {{ $t('screen.onboarding.actions.finish') }}
    </button>
  </div>
</template>
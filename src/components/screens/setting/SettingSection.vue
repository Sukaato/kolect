<script setup lang="ts">
defineProps<{
  title: string
}>()
</script>

<template>
  <div class="space-y-0">
    <p class="text-xs font-semibold uppercase tracking-widest text-base-content/40 px-1 mb-2">
      {{ title }}
    </p>
    <div class="bg-base-100 rounded-2xl overflow-hidden divide-y divide-base-200 shadow-sm">
      <slot></slot>
    </div>
  </div>
</template>
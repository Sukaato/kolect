<script setup lang="ts">
defineProps<{
  region: string
}>()
</script>

<template>
  <span v-if="region.toLocaleUpperCase() !== 'GLOBAL'"
    class="inline-flex items-center px-1.5 py-0.5 rounded text-[9px] font-bold uppercase tracking-wide bg-secondary/20 text-secondary">
    {{ region }}
  </span>
</template>
<script setup lang="ts">
import { LucideIcon } from 'lucide-vue-next';

defineProps<{
  label: string
  sublabel?: string
  icon: LucideIcon
  iconColor: string
}>()
</script>

<template>
  <div class="flex items-center justify-between gap-3 px-4 py-3.5 min-h-14">
    <div class="flex items-center gap-3 flex-1 min-w-0">
      <component :is="icon" class="w-5 h-5 shrink-0" :class="iconColor" />
      <div class="flex-1 min-w-0">
        <span class="text-sm font-medium block">{{ label }}</span>
        <span v-if="sublabel" class="text-xs text-base-content/50 block">{{ sublabel }}</span>
      </div>
    </div>
    <div v-if="$slots.default" class="shrink-0">
      <slot></slot>
    </div>
  </div>
</template>
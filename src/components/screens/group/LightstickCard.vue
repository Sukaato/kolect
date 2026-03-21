<script setup lang="ts">
import type { LightstickItem } from '@/types/group.type';

defineProps<{
  lightstick: LightstickItem
}>()
</script>

<template>
  <div class="shrink-0 w-20 text-center">
    <div
      class="relative w-20 h-20 rounded-xl bg-base-100 border flex items-center justify-center text-2xl transition-colors"
      :class="lightstick.ownedCount > 0 ? 'border-success bg-success/10' : 'border-base-300'">
      <img v-if="lightstick.imageUrl" :src="lightstick.imageUrl" :alt="lightstick.name"
        class="w-full h-full object-cover rounded-xl" />
      <span v-else>🔦</span>
      <span v-if="lightstick.ownedCount > 1"
        class="absolute top-1 right-1 bg-primary text-white text-[9px] font-bold w-4 h-4 rounded-full flex items-center justify-center">
        {{ lightstick.ownedCount }}
      </span>
      <span v-else-if="lightstick.ownedCount === 1"
        class="absolute bottom-1 right-1 w-2.5 h-2.5 rounded-full bg-success border-2 border-base-100" />
    </div>
    <p class="text-[10px] text-base-content/60 mt-1.5">{{ lightstick.version }}</p>
  </div>
</template>
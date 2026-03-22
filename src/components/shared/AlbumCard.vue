<script setup lang="ts">
import type { AlbumSummary } from '@/types/group.type';

const { summary } = defineProps<{
  summary: AlbumSummary
}>()

const isComplete = summary.ownedCount === summary.totalCount && summary.totalCount > 0
</script>

<template>
  <div class="shrink-0 w-24 cursor-pointer group">
    <div
      class="relative w-24 h-24 rounded-xl bg-base-200 border border-base-300 flex items-center justify-center overflow-hidden group-active:opacity-70 transition-opacity">
      <img v-if="summary.imageUrl" :src="summary.imageUrl" :alt="summary.name" class="w-full h-full object-cover" />
      <span v-else class="text-3xl">🎵</span>
      <span class="absolute top-1.5 right-1.5 text-white text-[10px] font-semibold px-1.5 py-0.5 rounded-full"
        :class="isComplete ? 'bg-success' : 'bg-primary'">
        {{ summary.ownedCount }}/{{ summary.totalCount }}
      </span>
    </div>
    <p class="text-[11px] font-semibold mt-1.5 truncate">{{ summary.name }}</p>
  </div>
</template>
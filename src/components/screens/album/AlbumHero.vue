<script setup lang="ts">
import { computed } from 'vue'
import type { AlbumDetail } from '@/types/album.type';

const { detail } = defineProps<{
  detail: AlbumDetail
}>()

const progressPercent = computed(() =>
  detail.totalCount > 0
    ? Math.round((detail.ownedCount / detail.totalCount) * 100)
    : 0
)
</script>

<template>
  <div class="bg-base-100 border-b border-base-300 px-4 py-4">
    <div class="max-w-lg mx-auto flex gap-4 items-start">

      <div
        class="shrink-0 w-20 h-20 rounded-xl bg-base-200 border border-base-300 flex items-center justify-center text-3xl overflow-hidden">
        <img v-if="detail.imageUrl" :src="detail.imageUrl" :alt="detail.name" class="w-full h-full object-cover" />
        <span v-else>🎵</span>
      </div>

      <div class="flex-1 min-w-0">
        <p class="font-bold text-lg leading-tight truncate">{{ detail.name }}</p>
        <p class="text-sm text-base-content/50 mt-0.5">
          {{ new Date(detail.releaseDate).getFullYear() }}
        </p>

        <div class="mt-3 space-y-1">
          <div class="flex justify-between text-xs text-base-content/50">
            <span>{{ $t('screens.album.hero.progress') }}</span>
            <span class="text-primary font-semibold">
              {{ detail.ownedCount }} / {{ detail.totalCount }}
            </span>
          </div>
          <div class="h-1.5 bg-base-200 rounded-full overflow-hidden">
            <div class="h-full bg-primary rounded-full transition-all" :style="{ width: `${progressPercent}%` }" />
          </div>
        </div>
      </div>

    </div>
  </div>
</template>
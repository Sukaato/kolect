<script setup lang="ts">
import { useAlbumStore } from '@/stores/album.store'
import { storeToRefs } from 'pinia'

const albumStore = useAlbumStore()
const { detail, versionsProgress, digipacksProgress, photocardsProgress, showPhotocardsProgress } =
  storeToRefs(albumStore)
</script>

<template>
  <div class="album--hero bg-base-100 border-b border-base-300 px-4 py-4">
    <div class="max-w-lg mx-auto flex gap-4 items-start">

      <!-- Cover -->
      <div
        class="shrink-0 w-20 h-20 rounded-xl bg-base-200 border border-base-300 flex items-center justify-center text-3xl overflow-hidden">
        <img v-if="detail?.imageUrl" :src="detail.imageUrl" :alt="detail.name" class="w-full h-full object-cover" />
        <span v-else>🎵</span>
      </div>

      <!-- Info + progress -->
      <div class="flex-1 min-w-0">
        <p class="font-bold text-lg leading-tight truncate">{{ detail?.name }}</p>
        <p class="text-sm text-base-content/50 mt-0.5">
          {{ detail ? new Date(detail.releaseDate).getFullYear() : '' }}
        </p>

        <div class="mt-3 space-y-2">

          <!-- Versions -->
          <div v-if="versionsProgress.total > 0" class="space-y-0.5">
            <div class="flex justify-between text-xs text-base-content/50">
              <span>{{ $t('screen.album.hero.versions') }}</span>
              <span class="text-primary font-semibold">
                {{ versionsProgress.owned }} / {{ versionsProgress.total }}
              </span>
            </div>
            <div class="h-1.5 bg-base-200 rounded-full overflow-hidden">
              <div class="h-full bg-primary rounded-full transition-all"
                :style="{ width: `${versionsProgress.percent}%` }" />
            </div>
          </div>

          <!-- Digipacks -->
          <div v-if="digipacksProgress.total > 0" class="space-y-0.5">
            <div class="flex justify-between text-xs text-base-content/50">
              <span>{{ $t('screen.album.hero.digipacks') }}</span>
              <span class="text-secondary font-semibold">
                {{ digipacksProgress.owned }} / {{ digipacksProgress.total }}
              </span>
            </div>
            <div class="h-1.5 bg-base-200 rounded-full overflow-hidden">
              <div class="h-full bg-secondary rounded-full transition-all"
                :style="{ width: `${digipacksProgress.percent}%` }" />
            </div>
          </div>

          <!-- Photocards (setting-gated) -->
          <div v-if="showPhotocardsProgress" class="space-y-0.5">
            <div class="flex justify-between text-xs text-base-content/50">
              <span>{{ $t('screen.album.hero.photocards') }}</span>
              <span class="text-accent font-semibold">
                {{ photocardsProgress.owned }} / {{ photocardsProgress.total }}
              </span>
            </div>
            <div class="h-1.5 bg-base-200 rounded-full overflow-hidden">
              <div class="h-full bg-accent rounded-full transition-all"
                :style="{ width: `${photocardsProgress.percent}%` }" />
            </div>
          </div>

        </div>
      </div>

    </div>
  </div>
</template>
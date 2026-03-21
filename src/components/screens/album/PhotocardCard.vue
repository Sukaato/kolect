<script setup lang="ts">
import { computed } from 'vue'
import type { AlbumVersionItem, PhotocardItem } from '@/types/album.type';
import type { ArtistWithAliases } from '@/types/schema/artist.type';

const { members, photocard, versions } = defineProps<{
  photocard: PhotocardItem
  members: ArtistWithAliases[]
  versions: AlbumVersionItem[]
}>()

const memberName = computed(() => {
  if (!photocard.artistId) return null
  const match = members.find(m => m.artist.id === photocard.artistId)
  return match?.aliases.find(a => a.kind === 'group_stage' && a.isPrimary)?.name ?? null
})

const versionName = computed(() => {
  if (!photocard.albumVersionId) return null
  return versions.find(v => v.id === photocard.albumVersionId)?.name ?? null
})
</script>

<template>
  <div class="flex flex-col items-center gap-1">
    <div class="relative w-full aspect-2/3 rounded-lg border flex items-center justify-center text-xl transition-colors"
      :class="photocard.ownedCount > 0 ? 'border-success bg-success/10' : 'bg-base-200 border-base-300'">
      <img v-if="photocard.imageUrl" :src="photocard.imageUrl" class="w-full h-full object-cover rounded-lg" />
      <span v-else>🃏</span>

      <span v-if="photocard.ownedCount > 1"
        class="absolute top-1 right-1 bg-primary text-white text-[9px] font-bold w-4 h-4 rounded-full flex items-center justify-center">
        {{ photocard.ownedCount }}
      </span>
      <span v-else-if="photocard.ownedCount === 1"
        class="absolute bottom-1 right-1 w-2 h-2 rounded-full bg-success border border-base-100" />

      <span v-if="photocard.hasSigned" class="absolute top-1 left-1 text-[10px]" title="Signé">✍️</span>
    </div>
    <p v-if="memberName" class="text-[10px] font-medium text-center leading-tight truncate w-full">
      {{ memberName }}
    </p>
    <p v-if="versionName" class="text-[10px] text-base-content/40 text-center truncate w-full">
      {{ versionName }}
    </p>
  </div>
</template>
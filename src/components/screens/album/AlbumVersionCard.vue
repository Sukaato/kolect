<script setup lang="ts">
import RegionChip from '@/components/global/RegionChip.vue';
import { useAlbumStore } from '@/stores/album.store';
import { usePossessionStore } from '@/stores/possession.store';
import type { AlbumVersionItem } from '@/types/album.type';
import type { AlbumId } from '@/types/schema/album.type';

const { version, albumId } = defineProps<{
  version: AlbumVersionItem
  albumId: AlbumId
}>()

const possessionStore = usePossessionStore()
const albumStore = useAlbumStore()

function handleClick() {
  possessionStore.open({
    type: 'albumVersion',
    id: version.id,
    name: version.name,
    imageUrl: version.imageUrl,
    ownedCount: version.ownedCount,
    signedCount: version.hasSigned ? 1 : 0,
    hasSigned: true,
    onSaved: () => albumStore.load(albumId),
  })
}
</script>

<template>
  <div class="shrink-0 w-24 text-center cursor-pointer" @click="handleClick">
    <div
      class="relative w-24 h-24 rounded-xl bg-base-100 border flex items-center justify-center text-3xl transition-colors active:opacity-70"
      :class="version.ownedCount > 0 ? 'border-success bg-success/10' : 'border-base-300'">
      <img v-if="version.imageUrl" :src="version.imageUrl" :alt="version.name"
        class="w-full h-full object-cover rounded-xl" />
      <span v-else>💿</span>

      <span v-if="version.ownedCount > 1"
        class="absolute top-1 right-1 bg-primary text-primary-content text-[9px] font-bold w-4 h-4 rounded-full flex items-center justify-center">
        {{ version.ownedCount }}
      </span>
      <span v-else-if="version.ownedCount === 1"
        class="absolute bottom-1.5 right-1.5 w-2.5 h-2.5 rounded-full bg-success border-2 border-base-100" />

      <span v-if="version.hasSigned"
        class="absolute top-1 left-1 bg-warning text-warning-content text-[8px] font-bold px-1 py-0.5 rounded">
        ✍️
      </span>
    </div>
    <p class="text-[11px] font-semibold mt-1.5 truncate">{{ version.name }}</p>
    <RegionChip :region="version.region" />
  </div>
</template>
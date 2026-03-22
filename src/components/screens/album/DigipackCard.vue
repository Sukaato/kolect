<script setup lang="ts">
import RegionChip from '@/components/global/RegionChip.vue'
import { useAlbumStore } from '@/stores/album.store'
import { usePossessionStore } from '@/stores/possession.store'
import type { DigipackItem } from '@/types/album.type'
import type { AlbumId } from '@/types/schema/album.type'
import { storeToRefs } from 'pinia'
import { computed } from 'vue'

const { digipack, albumId } = defineProps<{
  digipack: DigipackItem
  albumId: AlbumId
}>()

const possessionStore = usePossessionStore()
const albumStore = useAlbumStore()
const { members } = storeToRefs(albumStore)

const memberName = computed(() => {
  if (!digipack.artistId) return null
  const match = members.value.find(m => m.artist.id === digipack.artistId)
  return match?.aliases.find(a => a.kind === 'group_stage' && a.isPrimary)?.name ?? null
})

function handleClick() {
  possessionStore.open({
    type: 'digipack',
    id: digipack.id,
    name: digipack.name,
    imageUrl: digipack.imageUrl,
    ownedCount: digipack.ownedCount,
    signedCount: digipack.hasSigned ? 1 : 0,
    hasSigned: true,
    onSaved: () => albumStore.load(albumId),
  })
}
</script>

<template>
  <div class="shrink-0 w-24 text-center cursor-pointer" @click="handleClick">
    <div
      class="relative w-24 h-24 rounded-xl bg-base-100 border flex items-center justify-center text-3xl transition-colors active:opacity-70"
      :class="digipack.ownedCount > 0 ? 'border-success bg-success/10' : 'border-base-300'">
      <img v-if="digipack.imageUrl" :src="digipack.imageUrl" :alt="digipack.name"
        class="w-full h-full object-cover rounded-xl" />
      <span v-else>📦</span>

      <span v-if="digipack.ownedCount > 1"
        class="absolute top-1 right-1 bg-primary text-primary-content text-[9px] font-bold w-4 h-4 rounded-full flex items-center justify-center">
        {{ digipack.ownedCount }}
      </span>
      <span v-else-if="digipack.ownedCount === 1"
        class="absolute bottom-1.5 right-1.5 w-2.5 h-2.5 rounded-full bg-success border-2 border-base-100" />

      <span v-if="digipack.hasSigned"
        class="absolute top-1 left-1 bg-warning text-warning-content text-[8px] font-bold px-1 py-0.5 rounded">
        ✍️
      </span>
    </div>
    <p class="text-[11px] font-semibold mt-1.5 truncate">{{ digipack.name }}</p>
    <p v-if="memberName" class="text-[10px] text-base-content/50 truncate">{{ memberName }}</p>
    <RegionChip :region="digipack.region" />
  </div>
</template>
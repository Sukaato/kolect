<script setup lang="ts">
import EntityLayout from '@/components/layout/EntityLayout.vue'
import { useArtistStore } from '@/stores/artist.store'
import { useFavoriteStore } from '@/stores/favorite.store'
import { RouteName } from '@/types/routes'
import type { ArtistId } from '@/types/schema/artist.type'
import { storeToRefs } from 'pinia'
import { onMounted, shallowRef } from 'vue'

const { id } = defineProps<{
  id: ArtistId
}>()

const artistStore = useArtistStore()
const {
  artist,
  displayName,
  loading,
  possessionFilter,
  filteredAlbums,
  filteredLightsticks,
  filteredFanclubKits,
} = storeToRefs(artistStore)

const favoriteStore = useFavoriteStore()
const isFavorite = shallowRef(false)

async function handleFavoriteToggle() {
  const result = await favoriteStore.toggleArtist(id)
  if (result !== null) isFavorite.value = result
}

onMounted(async () => {
  await artistStore.load(id)
  isFavorite.value = artist.value?.isFavorite ?? false
})
</script>

<template>
  <EntityLayout v-model:possession-filter="possessionFilter" :loading :is-favorite
    :favorite-loading="favoriteStore.loading" :albums="filteredAlbums" :lightsticks="filteredLightsticks"
    :fanclub-kits="filteredFanclubKits"
    :album-route="(albumId) => ({ name: RouteName.ARTIST_ALBUM, params: { id, albumId } })" class="screen--artist"
    @toggle-favorite="handleFavoriteToggle" @on-saved="artistStore.load(id, true)">
    <template #title>{{ displayName }}</template>

    <template #empty="{ possessionFilter }">
      <p class="text-sm">{{
        possessionFilter === 'owned' ? $t('screen.artist.list.empty_owned')
          : possessionFilter === 'missing' ? $t('screen.artist.list.empty_missing')
            : $t('screen.artist.list.empty')
      }}</p>
    </template>
  </EntityLayout>
</template>
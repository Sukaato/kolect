<script setup lang="ts">
import EntityLayout from '@/components/layout/EntityLayout.vue'
import MembersSheet from '@/components/screens/group/MembersSheet.vue'
import { useFavoriteStore } from '@/stores/favorite.store'
import { useGroupStore } from '@/stores/group.store'
import { RouteName } from '@/types/routes'
import type { GroupId } from '@/types/schema/group.type'
import { UsersIcon } from '@lucide/vue'
import { storeToRefs } from 'pinia'
import { onMounted, shallowRef } from 'vue'
import { useRouter } from 'vue-router'

const { id, mode } = defineProps<{
  id: GroupId
  mode: string
}>()

const router = useRouter()

const groupStore = useGroupStore()
const {
  group,
  members,
  loading,
  possessionFilter,
  filteredAlbums,
  filteredLightsticks,
  filteredFanclubKits,
} = storeToRefs(groupStore)

const favoriteStore = useFavoriteStore()
const isFavorite = shallowRef(false)
const isMembersSheetOpen = shallowRef(false)

async function handleFavoriteToggle() {
  const result = await favoriteStore.toggleGroup(id)
  if (result !== null) isFavorite.value = result
}

function navigateToSolo(artistId: string) {
  router.push({ name: RouteName.ARTIST, params: { id: artistId } })
}

onMounted(async () => {
  await groupStore.load(id)
  isFavorite.value = group.value?.isFavorite ?? false
})
</script>

<template>
  <EntityLayout v-model:possession-filter="possessionFilter" :loading :is-favorite
    :favorite-loading="favoriteStore.loading" :albums="filteredAlbums" :lightsticks="filteredLightsticks"
    :fanclub-kits="filteredFanclubKits"
    :album-route="(albumId) => ({ name: RouteName.GROUP_ALBUM, params: { id, mode, albumId } })" class="screen--group"
    @toggle-favorite="handleFavoriteToggle" @on-saved="groupStore.load(id, true)">
    <template #title>{{ group?.name ?? '...' }}</template>

    <template #header-actions>
      <button class="btn btn-ghost btn-sm btn-circle" @click="isMembersSheetOpen = true">
        <UsersIcon class="w-5 h-5" />
      </button>
    </template>

    <template #empty="{ possessionFilter }">
      <p class="text-sm">{{
        possessionFilter === 'owned' ? $t('screen.group.list.empty_owned')
          : possessionFilter === 'missing' ? $t('screen.group.list.empty_missing')
            : $t('screen.group.list.empty')
      }}</p>
    </template>

    <template #bottom>
      <MembersSheet :open="isMembersSheetOpen" :members @close="isMembersSheetOpen = false"
        @navigate-to-solo="navigateToSolo" />
    </template>
  </EntityLayout>
</template>
<script setup lang="ts">
import AlbumCard from '@/components/screens/group/AlbumCard.vue'
import FanclubKitCard from '@/components/screens/group/FanclubKitCard.vue'
import LightstickCard from '@/components/screens/group/LightstickCard.vue'
import { useArtistStore } from '@/stores/artist.store'
import { PossessionFilter } from '@/types/group.type'
import { RouteName } from '@/types/routes'
import { ChevronLeftIcon } from 'lucide-vue-next'
import { storeToRefs } from 'pinia'
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'

const { id } = defineProps<{
  id: string
}>()

const router = useRouter()

const artistStore = useArtistStore()
const {
  displayName,
  loading,
  possessionFilter,
  filteredAlbums,
  filteredLightsticks,
  filteredFanclubKits,
} = storeToRefs(artistStore)

const FILTERS: PossessionFilter[] = ['all', 'owned', 'missing']

onMounted(() => artistStore.load(id as any))
</script>

<template>
  <div class="screen--artist grow relative pb-10">

    <!-- Header -->
    <div class="sticky top-0 z-10 bg-base-100/80 backdrop-blur-md border-b border-base-300">
      <div class="px-4 py-3 max-w-lg mx-auto space-y-3">

        <div class="flex items-center gap-3">
          <button @click="router.back()" class="btn btn-ghost btn-sm btn-circle">
            <ChevronLeftIcon class="w-5 h-5" />
          </button>
          <h1 class="text-xl font-bold tracking-tight flex-1">
            {{ displayName }}
          </h1>
        </div>

        <div class="flex gap-2">
          <button v-for="filter in FILTERS" :key="filter" class="btn btn-xs rounded-full"
            :class="possessionFilter === filter ? 'btn-neutral' : 'btn-ghost'" @click="possessionFilter = filter">
            {{ $t(`screens.artist.filters.${filter}`) }}
          </button>
        </div>

      </div>
    </div>

    <!-- Skeleton -->
    <div v-if="loading" class="max-w-lg mx-auto px-4 pt-6 space-y-8">
      <div v-for="i in 3" :key="i" class="space-y-3">
        <div class="skeleton h-3 w-20 rounded" />
        <div class="flex gap-3">
          <div v-for="j in 4" :key="j" class="skeleton h-24 w-24 rounded-xl shrink-0" />
        </div>
      </div>
    </div>

    <!-- Contenu -->
    <div v-else class="max-w-lg mx-auto px-4 pt-6 space-y-8">

      <section v-if="filteredAlbums.length">
        <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
          {{ $t('screens.artist.sections.albums') }}
        </h2>
        <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
          <RouterLink v-for="summary in filteredAlbums" :key="summary.albumId"
            :to="{ name: RouteName.ARTIST_ALBUM, params: { id: id, albumId: summary.albumId } }">
            <AlbumCard :summary />
          </RouterLink>
        </div>
      </section>

      <section v-if="filteredLightsticks.length">
        <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
          {{ $t('screens.artist.sections.lightsticks') }}
        </h2>
        <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
          <LightstickCard v-for="ls in filteredLightsticks" :key="ls.id" :lightstick="ls" />
        </div>
      </section>

      <section v-if="filteredFanclubKits.length">
        <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
          {{ $t('screens.artist.sections.fanclub_kits') }}
        </h2>
        <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
          <FanclubKitCard v-for="fk in filteredFanclubKits" :key="fk.id" :fanclub-kit="fk" />
        </div>
      </section>

      <div v-if="!filteredAlbums.length && !filteredLightsticks.length && !filteredFanclubKits.length"
        class="text-center py-16 text-base-content/40 text-sm">
        {{ $t('screens.artist.empty') }}
      </div>

    </div>
  </div>
</template>

<style scoped>
.scrollbar-none::-webkit-scrollbar {
  display: none;
}

.scrollbar-none {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
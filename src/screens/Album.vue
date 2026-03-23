<script setup lang="ts">
import AlbumHero from '@/components/screens/album/AlbumHero.vue'
import DigipackCard from '@/components/screens/album/DigipackCard.vue'
import PhotocardSection from '@/components/screens/album/PhotocardSection.vue'
import AlbumVersionCard from '@/components/screens/album/AlbumVersionCard.vue'
import { useAlbumStore } from '@/stores/album.store'
import { PossessionFilter } from '@/types/group.type'
import { AlbumId } from '@/types/schema/album.type'
import { ChevronLeftIcon } from 'lucide-vue-next'
import { storeToRefs } from 'pinia'
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'

const { albumId } = defineProps<{
  albumId: AlbumId
}>()

const router = useRouter()

const albumStore = useAlbumStore()
const {
  detail,
  loading,
  possessionFilter,
  filteredVersions,
  filteredDigipacks,
} = storeToRefs(albumStore)

const FILTERS: PossessionFilter[] = ['all', 'owned', 'missing']

onMounted(() => albumStore.load(albumId))
</script>

<template>
  <div class="screen--album grow relative pb-10">

    <!-- Header -->
    <div class="sticky top-0 z-10 bg-base-100/80 backdrop-blur-md border-b border-base-300">
      <div class="px-4 py-3 max-w-lg mx-auto space-y-3">

        <div class="flex items-center gap-3">
          <button @click="router.back()" class="btn btn-ghost btn-sm btn-circle">
            <ChevronLeftIcon class="w-5 h-5" />
          </button>
          <h1 class="text-xl font-bold tracking-tight flex-1">
            {{ detail?.name ?? '...' }}
          </h1>
        </div>

        <div class="flex gap-2">
          <button v-for="filter in FILTERS" :key="filter" class="btn btn-xs rounded-full"
            :class="possessionFilter === filter ? 'btn-neutral' : 'btn-ghost'" @click="possessionFilter = filter">
            {{ $t(`screen.album.filters.${filter}`) }}
          </button>
        </div>

      </div>
    </div>

    <!-- Skeleton -->
    <div v-if="loading" class="max-w-lg mx-auto px-4 pt-4 space-y-8">
      <div class="flex gap-4">
        <div class="skeleton w-20 h-20 rounded-xl shrink-0" />
        <div class="flex-1 space-y-2 pt-1">
          <div class="skeleton h-4 w-32 rounded" />
          <div class="skeleton h-3 w-24 rounded" />
          <div class="skeleton h-2 w-full rounded mt-3" />
        </div>
      </div>
      <div v-for="i in 3" :key="i" class="space-y-3">
        <div class="skeleton h-3 w-20 rounded" />
        <div class="flex gap-3">
          <div v-for="j in 4" :key="j" class="skeleton h-24 w-24 rounded-xl shrink-0" />
        </div>
      </div>
    </div>

    <!-- Contenu -->
    <div v-else class="max-w-lg mx-auto">

      <AlbumHero v-if="detail" :detail="detail" />

      <div class="px-4 pt-6 space-y-8">

        <section v-if="filteredVersions.length">
          <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
            {{ $t('screen.album.sections.versions') }}
          </h2>
          <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
            <AlbumVersionCard v-for="version in filteredVersions" :key="version.id" :album-id="albumId"
              :version="version" />
          </div>
        </section>

        <section v-if="filteredDigipacks.length">
          <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
            {{ $t('screen.album.sections.digipacks') }}
          </h2>
          <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
            <DigipackCard v-for="digipack in filteredDigipacks" :key="digipack.id" :album-id="albumId"
              :digipack="digipack" />
          </div>
        </section>

        <PhotocardSection />

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
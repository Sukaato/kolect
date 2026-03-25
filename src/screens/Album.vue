<script setup lang="ts">
import AlbumHero from '@/components/screens/album/AlbumHero.vue'
import AlbumVersionCard from '@/components/screens/album/AlbumVersionCard.vue'
import DigipackCard from '@/components/screens/album/DigipackCard.vue'
import PhotocardSection from '@/components/screens/album/PhotocardSection.vue'
import { useAlbumStore } from '@/stores/album.store'
import type { PossessionFilter } from '@/types/group.type'
import type { AlbumId } from '@/types/schema/album.type'
import { ChevronLeftIcon } from '@lucide/vue'
import { storeToRefs } from 'pinia'
import { onMounted } from 'vue'

const { albumId } = defineProps<{
  albumId: AlbumId
}>()

const albumStore = useAlbumStore()
const {
  detail,
  loading,
  filteredVersions,
  filteredDigipacks,
} = storeToRefs(albumStore)

const possessionFilter = defineModel<PossessionFilter>('possessionFilter', { default: 'all' })

const FILTERS: PossessionFilter[] = ['all', 'owned', 'missing']

onMounted(() => albumStore.load(albumId))
</script>

<template>
  <div class="screen--album grow relative pb-10">
    <Transition name="fade">
      <div v-if="loading" class="grid place-items-center absolute inset-0 z-20 bg-neutral/60">
        <span class="loading loading-spinner loading-md"></span>
      </div>
    </Transition>

    <!-- Header -->
    <div class="sticky top-0 z-10 bg-base-100/80 backdrop-blur-md border-b border-base-300">
      <div class="px-4 py-3 max-w-lg mx-auto space-y-3">

        <div class="flex items-center gap-3">
          <button class="btn btn-ghost btn-sm btn-circle" @click="$router.back()">
            <ChevronLeftIcon class="w-5 h-5" />
          </button>
          <h1 class="text-xl font-bold tracking-tight flex-1">
            {{ detail?.name ?? '...' }}
          </h1>
        </div>

        <!-- Possession filter chips -->
        <div class="flex gap-2">
          <button v-for="filter in FILTERS" :key="filter" class="btn btn-xs rounded-full"
            :class="possessionFilter === filter ? 'btn-neutral' : 'btn-ghost'" @click="possessionFilter = filter">
            {{ $t(`common.filter.${filter}`) }}
          </button>
        </div>

      </div>
    </div>

    <!-- Content -->
    <div class="max-w-lg mx-auto">

      <AlbumHero v-if="detail" />

      <div class="px-4 pt-6 space-y-8">

        <!-- Versions -->
        <section v-if="filteredVersions.length">
          <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
            {{ $t('screen.album.section.versions') }}
          </h2>
          <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
            <AlbumVersionCard v-for="version in filteredVersions" :key="version.id" :album-id="albumId"
              :version="version" />
          </div>
        </section>

        <!-- Digipacks -->
        <section v-if="filteredDigipacks.length">
          <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
            {{ $t('screen.album.section.digipacks') }}
          </h2>
          <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
            <DigipackCard v-for="digipack in filteredDigipacks" :key="digipack.id" :album-id="albumId"
              :digipack="digipack" />
          </div>
        </section>

        <!-- Photocards -->
        <PhotocardSection />

        <!-- Empty state -->
        <div v-if="!filteredVersions.length && !filteredDigipacks.length"
          class="text-center py-16 text-base-content/40 text-sm">
          {{
            possessionFilter === 'owned'
              ? $t('screen.album.list.empty_owned')
              : possessionFilter === 'missing'
                ? $t('screen.album.list.empty_missing')
                : $t('screen.album.list.empty')
          }}
        </div>

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
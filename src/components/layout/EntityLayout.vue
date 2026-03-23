<script setup lang="ts">
import AlbumCard from '@/components/shared/AlbumCard.vue'
import FanclubKitCard from '@/components/shared/FanclubKitCard.vue'
import LightstickCard from '@/components/shared/LightstickCard.vue'
import type { AlbumSummary, FanclubKitItem, LightstickItem, PossessionFilter } from '@/types/group.type'
import { ChevronLeftIcon, StarIcon } from 'lucide-vue-next'
import type { RouteLocationRaw } from 'vue-router'

// ─── Props & emits ───────────────────────────────────────────────────────────

const {
  loading,
  isFavorite,
  favoriteLoading = false,
  albums,
  lightsticks,
  fanclubKits,
  albumRoute,
} = defineProps<{
  loading: boolean
  isFavorite: boolean
  favoriteLoading?: boolean
  albums: AlbumSummary[]
  lightsticks: LightstickItem[]
  fanclubKits: FanclubKitItem[]
  /** Builds the route for a given albumId */
  albumRoute: (albumId: string) => RouteLocationRaw
}>()

const possessionFilter = defineModel<PossessionFilter>('possessionFilter', { default: 'all' })

const emit = defineEmits<{
  'toggle-favorite': []
  'on-saved': []
}>()

// ─── Filters ─────────────────────────────────────────────────────────────────

const FILTERS: PossessionFilter[] = ['all', 'owned', 'missing']
</script>

<template>
  <div class="layout layout--entity grow relative pb-10">

    <!-- Header -->
    <header class="sticky top-0 z-10 bg-base-100/80 backdrop-blur-md border-b border-base-300">
      <div class="px-4 py-3 max-w-lg mx-auto space-y-3">

        <div class="flex items-center gap-3">
          <button class="btn btn-ghost btn-sm btn-circle" @click="$router.back()">
            <ChevronLeftIcon class="w-5 h-5" />
          </button>
          <h1 class="text-xl font-bold tracking-tight flex-1">
            <slot name="title"></slot>
          </h1>
          <button class="btn btn-ghost btn-sm btn-circle" :disabled="favoriteLoading" @click="$emit('toggle-favorite')">
            <StarIcon class="w-5 h-5 transition-colors"
              :class="isFavorite ? 'fill-yellow-400 text-yellow-400' : 'text-base-content/40'" />
          </button>
          <!-- Slot for extra header actions (e.g. members button on Group) -->
          <slot name="header-actions"></slot>
        </div>

        <!-- Possession filter chips -->
        <div class="flex gap-2">
          <button v-for="filter in FILTERS" :key="filter" class="btn btn-xs rounded-full"
            :class="possessionFilter === filter ? 'btn-neutral' : 'btn-ghost'" @click="possessionFilter = filter">
            {{ $t(`common.filter.${filter}`) }}
          </button>
        </div>

      </div>
    </header>

    <!-- Loading skeleton -->
    <div v-if="loading" class="max-w-lg mx-auto px-4 pt-6 space-y-8">
      <div v-for="i in 3" :key="i" class="space-y-3">
        <div class="skeleton h-3 w-20 rounded" />
        <div class="flex gap-3">
          <div v-for="j in 4" :key="j" class="skeleton h-24 w-24 rounded-xl shrink-0" />
        </div>
      </div>
    </div>

    <!-- Content -->
    <div v-else class="max-w-lg mx-auto px-4 pt-6 space-y-8">

      <!-- Albums -->
      <section v-if="albums.length">
        <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
          {{ $t('common.section.album.title') }}
        </h2>
        <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
          <RouterLink v-for="summary in albums" :key="summary.albumId" :to="albumRoute(summary.albumId)">
            <AlbumCard :summary />
          </RouterLink>
        </div>
      </section>

      <!-- Lightsticks -->
      <section v-if="lightsticks.length">
        <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
          {{ $t('common.section.lightstick.title') }}
        </h2>
        <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
          <LightstickCard v-for="ls in lightsticks" :key="ls.id" :lightstick="ls"
            :after-save="() => $emit('on-saved')" />
        </div>
      </section>

      <!-- Fanclub kits -->
      <section v-if="fanclubKits.length">
        <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
          {{ $t('common.section.fanclub_kit.title') }}
        </h2>
        <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-none">
          <FanclubKitCard v-for="fk in fanclubKits" :key="fk.id" :fanclub-kit="fk"
            :after-save="() => $emit('on-saved')" />
        </div>
      </section>

      <!-- Empty state -->
      <div v-if="!albums.length && !lightsticks.length && !fanclubKits.length"
        class="text-center py-16 text-base-content/40 text-sm">
        <slot name="empty" :possession-filter="possessionFilter"></slot>
      </div>
    </div>


    <!-- Slot for bottom sheets (e.g. MembersSheet on Group) -->
    <slot name="bottom"></slot>

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
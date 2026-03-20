<script setup lang="ts">
import { useCollectionStore } from '@/stores/collection.store'
import type { CollectionSortBy } from '@/stores/collection.store'
import { storeToRefs } from 'pinia'
import { onMounted, shallowRef, useTemplateRef } from 'vue'
import { PlusIcon, StarIcon } from 'lucide-vue-next'
import { useInfiniteScroll } from '@vueuse/core'

const collectionStore = useCollectionStore()
const { loading, items, hasMorePages, isEmpty } = storeToRefs(collectionStore)

// ─── Filtres ──────────────────────────────────────────────────────────────────

const sortOptions: { value: CollectionSortBy; label: string }[] = [
  { value: 'name', label: 'screens.collection.sort.name' },
  { value: 'agency', label: 'screens.collection.sort.agency' },
]

// Tracks quel filtre UI est actif : 'chips' | 'select' | 'toggle'
const filterVariant = shallowRef<'chips' | 'select' | 'toggle'>('chips')

onMounted(async () => {
  await collectionStore.fetch()
})

// ─── Scroll infini ────────────────────────────────────────────────────────────

const scrollContainerRef = useTemplateRef('infite-scroll')
useInfiniteScroll(scrollContainerRef, async () => {
  await collectionStore.loadNextPage()
}, {
  canLoadMore() {
    return hasMorePages.value
  },

})
</script>

<template>
  <div class="screen--collection grow relative pb-10">

    <!-- FAB -->
    <div class="fab absolute bottom-6 right-4 z-20">
      <RouterLink to="/collection/add" tabindex="0" role="button" class="btn btn-lg btn-circle btn-primary shadow-lg">
        <PlusIcon />
      </RouterLink>
    </div>

    <!-- Header -->
    <div class="sticky top-0 z-10 bg-base-100/80 backdrop-blur-md border-b border-base-300">
      <div class="flex items-center gap-3 px-4 py-4 max-w-lg mx-auto">
        <h1 class="text-xl font-bold tracking-tight flex-1">
          {{ $t('screens.collection.title') }}
        </h1>
      </div>

      <!-- Filtres -->
      <div class="px-4 pb-3 max-w-lg mx-auto space-y-2">

        <!-- Variante 1 : Chips horizontales scrollables -->
        <div v-if="filterVariant === 'chips'" class="flex gap-2 overflow-x-auto pb-1 no-scrollbar">
          <button v-for="opt in sortOptions" :key="opt.value" class="btn btn-sm shrink-0"
            :class="collectionStore.params.sortBy === opt.value ? 'btn-primary' : 'btn-ghost border border-base-300'"
            @click="collectionStore.setSortBy(opt.value)">
            {{ $t(opt.label) }}
          </button>
        </div>

        <!-- Variante 2 : Select/dropdown -->
        <select v-if="filterVariant === 'select'" class="select select-sm select-bordered w-full max-w-xs"
          :value="collectionStore.params.sortBy"
          @change="collectionStore.setSortBy(($event.target as HTMLSelectElement).value as CollectionSortBy)">
          <option v-for="opt in sortOptions" :key="opt.value" :value="opt.value">
            {{ $t(opt.label) }}
          </option>
        </select>

        <!-- Variante 3 : Boutons toggle -->
        <div v-if="filterVariant === 'toggle'" class="join">
          <button v-for="opt in sortOptions" :key="opt.value" class="btn btn-sm join-item"
            :class="collectionStore.params.sortBy === opt.value ? 'btn-primary' : 'btn-ghost'"
            @click="collectionStore.setSortBy(opt.value)">
            {{ $t(opt.label) }}
          </button>
        </div>
      </div>
    </div>

    <div class="max-w-lg mx-auto px-4 pt-4">

      <!-- Loading initial -->
      <div v-if="loading && items.length === 0" class="flex justify-center items-center h-40">
        <span class="loading loading-spinner loading-lg"></span>
      </div>

      <!-- Empty state -->
      <div v-else-if="isEmpty" class="flex flex-col items-center justify-center h-40 gap-2 text-base-content/50">
        <span class="text-4xl">📭</span>
        <p class="text-sm">{{ $t('screens.collection.empty') }}</p>
      </div>

      <!-- Grille 2 colonnes -->
      <div v-else ref="infite-scroll" class="grid grid-cols-2 gap-3">
        <div v-for="item in items" :key="item.id"
          class="card bg-base-200 shadow-sm overflow-hidden cursor-pointer hover:shadow-md transition-shadow">
          <!-- Image -->
          <figure class="relative aspect-square bg-base-300">
            <img v-if="item.imageUrl" :src="item.imageUrl" :alt="item.name" class="w-full h-full object-cover" />
            <div v-else class="w-full h-full flex items-center justify-center text-4xl text-base-content/20">
              {{ item.kind === 'group' ? '🎤' : '🎧' }}
            </div>

            <!-- Badge favori -->
            <div v-if="item.isFavorite" class="absolute top-2 right-2">
              <StarIcon class="w-4 h-4 fill-yellow-400 text-yellow-400 drop-shadow" />
            </div>

            <!-- Badge kind -->
            <div class="absolute top-2 left-2">
              <span class="badge badge-xs badge-ghost bg-base-100/80 backdrop-blur-sm capitalize">
                {{ $t(`common.kind.${item.kind}`) }}
              </span>
            </div>
          </figure>

          <!-- Footer -->
          <div class="p-2 space-y-0.5">
            <p class="font-semibold text-sm truncate">{{ item.name }}</p>
            <p class="text-xs text-base-content/50 truncate">{{ item.agencyName }}</p>
            <p class="text-xs text-primary font-medium pt-0.5">
              {{ item.ownedCount }} / {{ item.totalCount }}
            </p>
          </div>
        </div>
      </div>

      <!-- scroll infini -->
      <div v-if="hasMorePages" class="flex justify-center py-6">
        <span v-if="loading" class="loading loading-spinner loading-md" />
      </div>

      <!-- Fin de liste -->
      <div v-if="!hasMorePages && items.length > 0" class="text-center py-6 text-xs text-base-content/30">
        {{ $t('screens.collection.endOfList') }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}

.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
<script setup lang="ts">
import type { CollectionListStore } from '@/types/collection-list.type'
import type { CollectionSortBy, CollectionSummaryItem } from '@/stores/collection.store'
import { RouteName } from '@/types/routes'
import { useInfiniteScroll } from '@vueuse/core'
import { StarIcon } from 'lucide-vue-next'
import { computed, useTemplateRef } from 'vue'
import { useRouter } from 'vue-router'

// ─── Props ───────────────────────────────────────────────────────────────────

const { store } = defineProps<{
  store: CollectionListStore
  screenClass: string
}>()

// ─── Store ───────────────────────────────────────────────────────────────────

const loading = computed(() => store.loading)
const items = computed(() => store.items)
const hasMorePages = computed(() => store.hasMorePages)
const isEmpty = computed(() => store.isEmpty)

// ─── Filters ─────────────────────────────────────────────────────────────────

const sortOptions: { value: CollectionSortBy; label: string }[] = [
  { value: 'name', label: 'screens.collection.sort.name' },
  { value: 'agency', label: 'screens.collection.sort.agency' },
]

// ─── Navigation ──────────────────────────────────────────────────────────────

const router = useRouter()

function navigateToItem(item: CollectionSummaryItem) {
  if (item.kind === 'group') {
    router.push({
      name: RouteName.GROUP,
      params: { id: item.id, mode: 'collection' },
    })
  } else {
    router.push({
      name: RouteName.ARTIST,
      params: { id: item.id },
    })
  }
}

// ─── Infinite scroll ─────────────────────────────────────────────────────────

const scrollContainerRef = useTemplateRef('infinite-scroll')
useInfiniteScroll(
  scrollContainerRef,
  async () => {
    await store.loadNextPage()
  },
  {
    canLoadMore() {
      return hasMorePages.value
    },
  },
)
</script>

<template>
  <div class="grow relative pb-10" :class="screenClass">

    <!-- FAB slot -->
    <slot name="fab"></slot>

    <!-- Header -->
    <div class="sticky top-0 z-10 bg-base-100/80 backdrop-blur-md border-b border-base-300">
      <div class="flex items-center gap-3 px-4 py-4 max-w-lg mx-auto">
        <h1 class="text-xl font-bold tracking-tight flex-1">
          <slot name="title"></slot>
        </h1>
        <!-- Slot for additional header actions (e.g. sync button) -->
        <slot name="header-actions"></slot>
      </div>

      <!-- Filters -->
      <div class="px-4 pb-3 max-w-lg mx-auto">
        <div class="flex gap-2 overflow-x-auto pb-1 no-scrollbar">
          <button v-for="opt in sortOptions" :key="opt.value" class="btn btn-sm shrink-0"
            :class="store.params.sortBy === opt.value ? 'btn-primary' : 'btn-ghost border border-base-300'"
            @click="store.setSortBy(opt.value)">
            {{ $t(opt.label) }}
          </button>
        </div>
      </div>
    </div>

    <div class="max-w-lg mx-auto px-4 pt-4">

      <!-- Initial loading -->
      <div v-if="loading && items.length === 0" class="flex justify-center items-center h-40">
        <span class="loading loading-spinner loading-lg" />
      </div>

      <!-- Empty state -->
      <div v-else-if="isEmpty" class="flex flex-col items-center justify-center h-40 gap-2 text-base-content/50">
        <span class="text-4xl">📭</span>
        <p class="text-sm">{{ $t('screens.collection.empty') }}</p>
      </div>

      <!-- 2-column grid -->
      <div v-else ref="infinite-scroll" class="grid grid-cols-2 gap-3">
        <div v-for="item in items" :key="item.id"
          class="card bg-base-200 shadow-sm overflow-hidden cursor-pointer hover:shadow-md transition-shadow active:opacity-70"
          @click="navigateToItem(item)">
          <!-- Image -->
          <figure class="relative aspect-square bg-base-300">
            <img v-if="item.imageUrl" :src="item.imageUrl" :alt="item.name" class="w-full h-full object-cover" />
            <div v-else class="w-full h-full flex items-center justify-center text-4xl text-base-content/20">
              {{ item.kind === 'group' ? '🎤' : '🎧' }}
            </div>

            <!-- Favorite badge -->
            <div v-if="item.isFavorite" class="absolute top-2 right-2">
              <StarIcon class="w-4 h-4 fill-yellow-400 text-yellow-400 drop-shadow" />
            </div>

            <!-- Kind badge -->
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

      <!-- Infinite scroll spinner -->
      <div v-if="hasMorePages" class="flex justify-center py-6">
        <span v-if="loading" class="loading loading-spinner loading-md" />
      </div>

      <!-- End of list -->
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
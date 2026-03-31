<script setup lang="ts">
import { CommandName, useInvoke } from '@/composables/use-invoke'
import type { CollectionSummaryItem } from '@/stores/collection.store'
import type { CollectionListStore } from '@/types/collection-list.type'
import { RouteName } from '@/types/routes'
import { AgencyId } from '@/types/schema/agency.type'
import { useDebounceFn, useInfiniteScroll } from '@vueuse/core'
import { SearchIcon, StarIcon } from '@lucide/vue'
import { computed, onMounted, shallowRef, useTemplateRef, watch } from 'vue'
import { useRouter } from 'vue-router'

// ─── Props ───────────────────────────────────────────────────────────────────

const {
  store,
  screenClass,
  agenciesCommand,
  searchDebounce = 500,
} = defineProps<{
  store: CollectionListStore
  screenClass: string
  /** Tauri command name to load the agency list (differs between Home and Collection) */
  agenciesCommand: Extract<CommandName, 'dataset_get_agencies' | 'collection_get_agencies'>
  /** Debounce delay in ms for the search input (default: 500) */
  searchDebounce?: number
}>()

// ─── Store ───────────────────────────────────────────────────────────────────

const loading = computed(() => store.loading)
const items = computed(() => store.items)
const hasMorePages = computed(() => store.hasMorePages)
const isEmpty = computed(() => store.isEmpty)
const hasActiveFilters = computed(
  () => !!(store.params.search || store.params.agencyId),
)

// ─── Agencies ─────────────────────────────────────────────────────────────────

const { data: agencies, invoke: fetchAgencies } = useInvoke(agenciesCommand, {
  defaults: [],
})

onMounted(async () => {
  await fetchAgencies()
})

// ─── Filters ─────────────────────────────────────────────────────────────────

const searchInput = shallowRef<string>('')
const selectedAgencyId = shallowRef<string | null>(null)

const debouncedSearch = useDebounceFn(async (value: string) => {
  await store.setSearch(value.trim() || null)
}, searchDebounce)

watch(searchInput, value => debouncedSearch(value))

async function onAgencyChange(agencyId: string | null) {
  selectedAgencyId.value = agencyId
  await store.setAgency(agencyId as AgencyId)
}

// ─── Navigation ──────────────────────────────────────────────────────────────

const router = useRouter()

function navigateToItem(item: CollectionSummaryItem) {
  if (item.kind === 'group') {
    router.push({
      name: RouteName.GROUP,
      params: { id: item.id },
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
      <div class="px-4 pb-3 max-w-lg mx-auto flex gap-2">

        <!-- Search input -->
        <label class="input input-sm flex items-center gap-2 flex-1 border border-base-300">
          <SearchIcon class="h-3.5 w-3.5 shrink-0 text-base-content/40" />
          <input v-model="searchInput" type="search" class="grow bg-transparent outline-none text-sm"
            :placeholder="$t('common.filter.search.placeholder')" />
        </label>

        <!-- Agency dropdown -->
        <select class="select select-sm select-bordered border-base-300 max-w-35" :value="selectedAgencyId ?? ''"
          @change="onAgencyChange($event.target.value || null)">
          <option value="">{{ $t('common.filter.agency.all') }}</option>
          <option v-for="agency in agencies" :key="agency.id" :value="agency.id">
            {{ agency.name }}
          </option>
        </select>

      </div>
    </div>

    <div class="max-w-lg mx-auto px-4 pt-4">

      <!-- Initial loading -->
      <div v-if="loading && items.length === 0" class="flex justify-center items-center h-40">
        <span class="loading loading-spinner loading-lg"></span>
      </div>

      <!-- Empty state -->
      <div v-else-if="isEmpty" class="flex flex-col items-center justify-center h-40 gap-2 text-base-content/50">
        <span class="text-4xl">📭</span>
        <slot name="empty" :has-active-filters="hasActiveFilters"></slot>
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
        <slot name="end-of-list"></slot>
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
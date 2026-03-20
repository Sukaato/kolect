import { defineStore } from 'pinia'
import { computed, readonly, ref, shallowRef, watch } from 'vue'
import { useInvoke } from '@/composables/use-invoke'
import type { PageMeta, PaginatedResult } from '@/types/pagination'
import { useSettingStore } from './setting.store'

export interface CollectionSummaryItem {
  id: string
  kind: 'group' | 'solo'
  name: string
  agencyName: string
  imageUrl: string | null
  isFavorite: boolean
  ownedCount: number
  totalCount: number
}

export type CollectionSortBy = 'name' | 'agency'

export interface CollectionSummaryParams {
  page: number
  perPage: number
  sortBy: CollectionSortBy
  includePhotocards: boolean
}

const defaultMeta: PageMeta = {
  perPage: 6,
  currentPage: 1,
  isFirst: true,
  isLast: true,
  isEmpty: true,
  total: 0,
  hasTotal: true,
  lastPage: 1,
  hasMorePages: false,
  hasPages: false,
}

const defaultSummary: PaginatedResult<CollectionSummaryItem> = {
  data: [],
  meta: defaultMeta,
}

export const useCollectionStore = defineStore('collection', () => {
  const settingStore = useSettingStore()
  watch(
    () => settingStore.includePhotocardCount,
    include => {
      params.value.includePhotocards = include
      reset()
    },
  )

  // ─── State ─────────────────────────────────────────────────────────────────

  const params = ref<CollectionSummaryParams>({
    page: defaultMeta.currentPage,
    perPage: defaultMeta.perPage,
    sortBy: 'name',
    includePhotocards: settingStore.includePhotocardCount,
  })

  // Pages accumulées : Map<pageNumber, items[]>
  const pages = ref<Map<number, CollectionSummaryItem[]>>(new Map())

  // Meta de la dernière page chargée
  const meta = shallowRef<PageMeta>(defaultMeta)

  // ─── Invoke ────────────────────────────────────────────────────────────────

  const { result, loading, error, invoke } = useInvoke<PaginatedResult<CollectionSummaryItem>>(
    'collection_get_summary',
    {
      defaults: defaultSummary,
    },
  )

  // ─── Computed ──────────────────────────────────────────────────────────────

  // Liste aplatie dans l'ordre des pages pour le scroll infini
  const items = computed<CollectionSummaryItem[]>(() => {
    const sorted = pages.value
      .entries()
      .toArray()
      .toSorted(([a], [b]) => a - b)
    return sorted.flatMap(([, data]) => data)
  })

  const hasMorePages = computed(() => meta.value.hasMorePages)

  const isEmpty = computed(() => meta.value.isEmpty)

  // ─── Helpers privés ────────────────────────────────────────────────────────

  function reset() {
    pages.value = new Map()
    meta.value = defaultMeta
    params.value = { ...params.value, page: 1 }
  }

  // ─── Actions ───────────────────────────────────────────────────────────────

  async function fetch(overrides?: Partial<CollectionSummaryParams>) {
    if (overrides) {
      params.value = { ...params.value, ...overrides }
    }

    await invoke({ params: params.value })

    if (result.value) {
      pages.value = new Map(pages.value).set(params.value.page, result.value.data)
      meta.value = result.value.meta
    }
  }

  async function loadNextPage() {
    if (!meta.value.hasMorePages) return
    await fetch({ page: params.value.page + 1 })
  }

  async function setSortBy(sortBy: CollectionSortBy) {
    reset()
    await fetch({ sortBy })
  }

  async function setIncludePhotocards(include: boolean) {
    reset()
    await fetch({ includePhotocards: include })
  }

  async function refresh() {
    reset()
    await fetch()
  }

  return {
    // State
    params: readonly(params),
    meta: readonly(meta),

    // Computed
    items,
    hasMorePages,
    isEmpty,

    // Invoke state
    loading: readonly(loading),
    error: readonly(error),

    // Actions
    fetch,
    loadNextPage,
    setSortBy,
    setIncludePhotocards,
    refresh,
  }
})

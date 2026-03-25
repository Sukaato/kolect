import { defineStore } from 'pinia'
import { computed, readonly, ref, shallowRef, watch } from 'vue'
import { useInvoke } from '@/composables/use-invoke'
import type { PageMeta, PaginatedResult } from '@/types/pagination.type'
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

export interface CollectionSummaryParams {
  page: number
  perPage: number
  includePhotocards: boolean
  includeExclusiveItems: boolean
  search: string | null
  agencyId: string | null
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
  // ─── Composables ───────────────────────────────────────────────────────────

  const settingStore = useSettingStore()
  watch(
    () => settingStore.includePhotocardInCount,
    include => {
      params.value.includePhotocards = include
      reset()
    },
  )
  watch(
    () => settingStore.includeExclusiveItems,
    include => {
      params.value.includeExclusiveItems = include
      reset()
    },
  )

  // ─── State ─────────────────────────────────────────────────────────────────

  const params = ref<CollectionSummaryParams>({
    page: defaultMeta.currentPage,
    perPage: defaultMeta.perPage,
    includePhotocards: settingStore.includePhotocardInCount,
    includeExclusiveItems: settingStore.includeExclusiveItems,
    search: null,
    agencyId: null,
  })

  // Accumulated pages: Map<pageNumber, items[]>
  const pages = ref<Map<number, CollectionSummaryItem[]>>(new Map())

  // Meta of the last loaded page
  const meta = shallowRef<PageMeta>(defaultMeta)

  // ─── Invoke ────────────────────────────────────────────────────────────────

  const { data, loading, error, invoke } = useInvoke<PaginatedResult<CollectionSummaryItem>>(
    'collection_get_summary',
    { defaults: defaultSummary },
  )

  // ─── Computed ──────────────────────────────────────────────────────────────

  // Flattened list in page order for infinite scroll
  const items = computed<CollectionSummaryItem[]>(() => {
    const sorted = pages.value
      .entries()
      .toArray()
      .toSorted(([a], [b]) => a - b)
    return sorted.flatMap(([, data]) => data)
  })

  const hasMorePages = computed(() => meta.value.hasMorePages)

  const isEmpty = computed(() => meta.value.isEmpty)

  // ─── Private helpers ───────────────────────────────────────────────────────

  function reset() {
    pages.value = new Map()
    meta.value = defaultMeta
    params.value = { ...params.value, page: 1 }
  }

  // ─── Actions ───────────────────────────────────────────────────────────────

  async function fetch(overrides?: Partial<CollectionSummaryParams>, refresh = false) {
    if (overrides) {
      params.value = { ...params.value, ...overrides }
    }

    await invoke({ params: params.value }, { resetBeforeInvoke: !refresh })

    if (data.value) {
      pages.value = new Map(pages.value).set(params.value.page, data.value.data)
      meta.value = data.value.meta
    }
  }

  async function loadNextPage() {
    if (!meta.value.hasMorePages) return
    await fetch({ page: params.value.page + 1 })
  }

  async function setSearch(search: string | null) {
    reset()
    await fetch({ search })
  }

  async function setAgency(agencyId: string | null) {
    reset()
    await fetch({ agencyId })
  }

  async function refresh() {
    reset()
    await fetch({}, true)
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
    setSearch,
    setAgency,
    refresh,
  }
})

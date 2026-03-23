import { acceptHMRUpdate, defineStore } from 'pinia'
import { computed, readonly, ref, shallowRef, watch } from 'vue'
import { useInvoke } from '@/composables/use-invoke'
import { useToast } from '@/composables/use-toast'
import type { PageMeta, PaginatedResult } from '@/types/pagination.type'
import type { CollectionSummaryItem, CollectionSummaryParams } from './collection.store'
import { useSettingStore } from './setting.store'

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

export const useDatasetStore = defineStore('dataset', () => {
  // ─── Composables ───────────────────────────────────────────────────────────
  const toast = useToast()
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
    includePhotocards: settingStore.includePhotocardCount,
    search: null,
    agencyId: null,
  })

  // Accumulated pages: Map<pageNumber, items[]>
  const pages = ref<Map<number, CollectionSummaryItem[]>>(new Map())

  // Meta of the last loaded page
  const meta = shallowRef<PageMeta>(defaultMeta)

  // Last time dataset has been synced
  const lastFetchedAt = shallowRef<Date>()

  // ─── Invoke ────────────────────────────────────────────────────────────────

  const {
    data: syncSuccess,
    loading: syncing,
    error: syncError,
    invoke: syncDataset,
  } = useInvoke<boolean>('dataset_sync')

  const {
    data: collection,
    loading,
    error,
    invoke,
  } = useInvoke<PaginatedResult<CollectionSummaryItem>>('dataset_get_summary', {
    defaults: defaultSummary,
  })

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

  async function sync(force: boolean = false) {
    await syncDataset({ force })
    if (syncSuccess) {
      toast.success('Dataset synced successfully')
    } else {
      toast.error(syncError.value ?? '', { title: 'Dataset failed to sync' })
    }
    lastFetchedAt.value = new Date()
  }

  async function fetch(overrides?: Partial<CollectionSummaryParams>, refresh = false) {
    if (overrides) {
      params.value = { ...params.value, ...overrides }
    }

    await invoke({ params: params.value }, { resetBeforeInvoke: !refresh })

    if (collection.value) {
      pages.value = new Map(pages.value).set(params.value.page, collection.value.data)
      meta.value = collection.value.meta
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
    lastFetchedAt: readonly(lastFetchedAt),

    // Computed
    items,
    hasMorePages,
    isEmpty,

    // Invoke state
    syncing: readonly(syncing),
    syncError: readonly(syncError),
    loading: readonly(loading),
    error: readonly(error),

    // Actions
    sync,
    fetch,
    loadNextPage,
    setSearch,
    setAgency,
    refresh,
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useDatasetStore, import.meta.hot))
}

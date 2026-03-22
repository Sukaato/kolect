import type { CollectionSortBy, CollectionSummaryItem } from '@/stores/collection.store'

/**
 * Common interface exposed by useCollectionStore and useDatasetStore
 * to feed the CollectionGrid component.
 */
export interface CollectionListStore {
  // State
  params: { readonly sortBy: CollectionSortBy; [key: string]: unknown }

  // Computed
  items: CollectionSummaryItem[]
  hasMorePages: boolean
  isEmpty: boolean

  // Invoke state
  loading: boolean
  error: string | undefined

  // Actions
  fetch: (overrides?: {
    page?: number
    sortBy?: CollectionSortBy
    includePhotocards?: boolean
  }) => Promise<void>
  loadNextPage: () => Promise<void>
  setSortBy: (sortBy: CollectionSortBy) => Promise<void>
}

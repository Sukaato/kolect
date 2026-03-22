import type { CollectionSummaryItem } from '@/stores/collection.store'

/**
 * Common interface exposed by useCollectionStore and useDatasetStore
 * to feed the CollectionGrid component.
 */
export interface CollectionListStore {
  // State
  params: {
    readonly search: string | null
    readonly agencyId: string | null
    [key: string]: unknown
  }

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
    search?: string | null
    agencyId?: string | null
    includePhotocards?: boolean
  }) => Promise<void>
  loadNextPage: () => Promise<void>
  setSearch: (search: string | null) => Promise<void>
  setAgency: (agencyId: string | null) => Promise<void>
}

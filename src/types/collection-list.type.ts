import type { CollectionSummaryItem } from '@/stores/collection.store'
import type { AgencyId } from './schema/agency.type'

/**
 * Common interface exposed by useCollectionStore and useDatasetStore
 * to feed the CollectionGrid component.
 */
export interface CollectionListStore {
  // State
  params: {
    readonly search: string | null
    readonly agencyId: AgencyId | null
    [key: string]: unknown
  }

  // Computed
  items: CollectionSummaryItem[]
  hasMorePages: boolean
  isEmpty: boolean

  // Invoke state
  loading: boolean
  error: string | null

  // Actions
  fetch: (overrides?: {
    page?: number
    search?: string | null
    agencyId?: AgencyId | null
    includePhotocards?: boolean
  }) => Promise<void>
  loadNextPage: () => Promise<void>
  setSearch: (search: string | null) => Promise<void>
  setAgency: (agencyId: AgencyId | null) => Promise<void>
}

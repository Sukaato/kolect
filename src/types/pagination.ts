export interface PaginatedResult<T> {
  data: T[]
  meta: PageMeta
}

export interface PageMeta {
  perPage: number
  currentPage: number
  isFirst: boolean
  isLast: boolean
  isEmpty: boolean
  total: number
  hasTotal: boolean
  lastPage: number
  hasMorePages: boolean
  hasPages: boolean
}

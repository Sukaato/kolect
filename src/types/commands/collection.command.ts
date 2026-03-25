import type { CollectionSummaryItem, CollectionSummaryParams } from '@/stores/collection.store'
import type { PaginatedResult } from '../pagination.type'
import type { Command, InferCommand } from './command.type'

type SummaryParam = {
  params: CollectionSummaryParams
}

type CollectionCommand = {
  collection_get_summary: Command<SummaryParam, PaginatedResult<CollectionSummaryItem>>
}

declare global {
  interface TauriInvokeCommands extends InferCommand<CollectionCommand> {}
}

import type { CollectionSummaryItem, CollectionSummaryParams } from '@/stores/collection.store'
import type { PaginatedResult } from '../pagination.type'
import type { Command, InferCommand } from './command.type'

type SummaryParam = {
  params: CollectionSummaryParams
}

type SyncParam = {
  force: boolean
}

type DatasetCommand = {
  dataset_get_summary: Command<SummaryParam, PaginatedResult<CollectionSummaryItem>>
  dataset_sync: Command<SyncParam, boolean>
}

declare global {
  interface TauriInvokeCommands extends InferCommand<DatasetCommand> {}
}

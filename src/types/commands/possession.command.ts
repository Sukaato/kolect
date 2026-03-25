import type { CollectibleType } from '@/stores/possession.store'
import type { Command, InferCommand } from './command.type'

type SyncParam = {
  item_type: CollectibleType
  item_id: string
  owned_count: number
  signed_count: number
}

type PossessionCommand = {
  collection_update_item: Command<SyncParam, void>
}

declare global {
  interface TauriInvokeCommands extends InferCommand<PossessionCommand> {}
}

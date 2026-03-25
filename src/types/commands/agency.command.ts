import type { Agency } from '../schema/agency.type'
import type { Command, InferCommand } from './command.type'

type AgencyCommand = {
  dataset_get_agencies: Command<void, Agency[]>
  collection_get_agencies: Command<void, Agency[]>
}

declare global {
  interface TauriInvokeCommands extends InferCommand<AgencyCommand> {}
}

import type { ArtistId } from '../schema/artist.type'
import type { GroupId } from '../schema/group.type'
import type { Command, InferCommand } from './command.type'

type SummaryParam = {
  group_id: GroupId
}
type SyncParam = {
  artist_id: ArtistId
}

type FavoriteCommand = {
  favorite_toggle_group: Command<SummaryParam, boolean>
  favorite_toggle_artist: Command<SyncParam, boolean>
}

declare global {
  interface TauriInvokeCommands extends InferCommand<FavoriteCommand> {}
}

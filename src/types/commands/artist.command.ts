import type { ArtistDetail } from '../artist.type'
import type { AlbumSummary, FanclubKitItem, LightstickItem } from '../group.type'
import type { ArtistId } from '../schema/artist.type'
import type { Command, InferCommand } from './command.type'

type DetailParam = {
  artist_id: ArtistId
}

type ArtistParams = {
  artist_id: ArtistId
  include_exclusive_items: boolean
}

type ArtistCommand = {
  artist_get_detail: Command<DetailParam, ArtistDetail>
  artist_get_album_summaries: Command<ArtistParams, AlbumSummary[]>
  artist_get_lightsticks: Command<ArtistParams, LightstickItem[]>
  artist_get_fanclub_kits: Command<ArtistParams, FanclubKitItem[]>
}

declare global {
  interface TauriInvokeCommands extends InferCommand<ArtistCommand> {}
}

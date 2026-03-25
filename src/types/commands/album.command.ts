import type { AlbumVersionItem, DigipackItem, PhotocardItem } from '../album.type'
import type { AlbumId } from '../schema/album.type'
import type { ArtistId } from '../schema/artist.type'
import type { GroupId } from '../schema/group.type'
import type { Command, InferCommand } from './command.type'

type AlbumParams = {
  album_id: AlbumId
  include_exclusive_items: boolean
}

interface AlbumDetail {
  albumId: AlbumId
  name: string
  releaseDate: string
  imageUrl: string | null
  groupId: GroupId | null
  artistId: ArtistId | null
  // Versions progress
  versionsOwnedCount: number
  versionsTotalCount: number
  // Digipacks progress
  digipacksOwnedCount: number
  digipacksTotalCount: number
  // Photocards progress
  photocardsOwnedCount: number
  photocardsTotalCount: number
}

type AlbumCommand = {
  album_get_detail: Command<AlbumParams, AlbumDetail>
  album_get_versions: Command<AlbumParams, AlbumVersionItem[]>
  album_get_digipacks: Command<AlbumParams, DigipackItem[]>
  album_get_photocards: Command<AlbumParams, PhotocardItem[]>
}

declare global {
  interface TauriInvokeCommands extends InferCommand<AlbumCommand> {}
}

import type { AlbumId, AlbumVersionId, DigipackId } from './album.type'
import type { ArtistId } from './artist.type'
import type { Brand } from '../brand.type'

export type PhotocardId = Brand<string, 'photocard_id'>
export type PhotocardKind = 'album_random' | 'album_version' | 'digipack'

export type Photocard = {
  id: PhotocardId
  artist_id: ArtistId | null
  album_id: AlbumId | null
  album_version_id: AlbumVersionId | null
  digipack_id: DigipackId | null
  release_date: string
  region: string
  image_url: string | null
  kind: PhotocardKind
}

import type { Brand } from '../brand.type'
import type { AlbumId, AlbumVersionId, DigipackId } from './album.type'
import type { ArtistId } from './artist.type'

export type PhotocardId = Brand<string, 'photocard_id'>
export type PhotocardKind = 'album_random' | 'album_version' | 'digipack'

export type Photocard = {
  id: PhotocardId
  artistId: ArtistId | null
  albumId: AlbumId | null
  albumVersionId: AlbumVersionId | null
  digipackId: DigipackId | null
  releaseDate: string
  region: string
  imageUrl: string | null
  kind: PhotocardKind
}

import type { AlbumId, AlbumVersionId, DigipackId } from './schema/album.type'
import type { ArtistId } from './schema/artist.type'
import type { GroupId } from './schema/group.type'

export type PhotocardFilterMode = 'chips' | 'dropdown'

export type AlbumDetail = {
  albumId: AlbumId
  name: string
  releaseDate: string
  imageUrl: string | null
  groupId: GroupId | null
  artistId: ArtistId | null
  ownedCount: number
  totalCount: number
}

export type AlbumVersionItem = {
  id: AlbumVersionId
  name: string
  format: string
  releaseDate: string
  imageUrl: string | null
  ownedCount: number
  hasSigned: boolean
}

export type DigipackItem = {
  id: DigipackId
  name: string
  artistId: ArtistId | null
  releaseDate: string
  imageUrl: string | null
  ownedCount: number
  hasSigned: boolean
}

export type PhotocardItem = {
  id: DigipackId
  artistId: ArtistId | null
  albumVersionId: AlbumVersionId | null
  digipackId: DigipackId | null
  imageUrl: string | null
  ownedCount: number
  hasSigned: boolean
}

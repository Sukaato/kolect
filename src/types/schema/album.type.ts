import type { Brand } from '../brand.type'
import type { ArtistId } from './artist.type'
import type { GroupId } from './group.type'

export type AlbumId = Brand<string, 'album_id'>

export type Album = {
  id: AlbumId
  name: string
  releaseDate: string
  region: string
  imageUrl: string | null
  groupId: GroupId | null
  artistId: ArtistId | null
}

export type AlbumVersionId = Brand<string, 'album_version_id'>
export type VersionFormat = 'cd' | 'ep' | 'album' | 'mini_album' | 'vinyl' | 'kit'

export type AlbumVersion = {
  id: AlbumVersionId
  albumId: AlbumId
  name: string
  format: VersionFormat
  releaseDate: string
  region: string
  imageUrl: string | null
}

export type DigipackId = Brand<string, 'digipack_id'>

export type Digipack = {
  id: DigipackId
  albumId: AlbumId
  artistId: ArtistId | null
  name: string
  releaseDate: string
  region: string
  imageUrl: string | null
}

export type AlbumFull = {
  album: Album
  versions: AlbumVersion[]
  digipacks: Digipack[]
}

import type { ArtistId } from './artist.type'
import type { Brand } from '../brand.type'
import type { GroupId } from './group.type'

export type AlbumId = Brand<string, 'album_id'>

export type Album = {
  id: AlbumId
  name: string
  release_date: string
  region: string
  image_url: string | null
  group_id: GroupId | null
  artist_id: ArtistId | null
}

export type AlbumVersionId = Brand<string, 'album_version_id'>
export type VersionFormat = 'cd' | 'ep' | 'album' | 'mini_album' | 'vinyl' | 'kit'

export type AlbumVersion = {
  id: AlbumVersionId
  album_id: AlbumId
  name: string
  format: VersionFormat
  release_date: string
  region: string
  image_url: string | null
}

export type DigipackId = Brand<string, 'digipack_id'>

export type Digipack = {
  id: DigipackId
  album_id: AlbumId
  artist_id: ArtistId | null
  name: string
  release_date: string
  region: string
  image_url: string | null
}

export type AlbumFull = {
  album: Album
  versions: AlbumVersion[]
  digipacks: Digipack[]
}

import type { AlbumId } from './schema/album.type'
import type { ArtistId, ArtistWithAliases } from './schema/artist.type'
import type { FanclubKitId } from './schema/fanclub-kit.type'
import type { Group, GroupId } from './schema/group.type'
import type { LightstickId } from './schema/lightstick.type'

export type PossessionFilter = 'all' | 'owned' | 'missing'

export type GroupDetail = {
  group: Group
  members: ArtistWithAliases[]
}

export interface AlbumSummary {
  albumId: AlbumId
  name: string
  releaseDate: string
  imageUrl: string | null
  versionsOwnedCount: number
  versionsTotalCount: number
  digipacksOwnedCount: number
  digipacksTotalCount: number
  photocardsOwnedCount: number
}

export type LightstickItem = {
  id: LightstickId
  groupId: GroupId | null
  artistId: ArtistId | null
  name: string
  version: string
  releaseDate: string
  region: string
  imageUrl: string | null
  ownedCount: number
}

export type FanclubKitItem = {
  id: FanclubKitId
  groupId: GroupId | null
  artistId: ArtistId | null
  name: string
  releaseDate: string
  region: string
  imageUrl: string | null
  ownedCount: number
}

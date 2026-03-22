import type { Brand } from '../brand.type'
import type { ArtistId } from './artist.type'
import type { GroupId } from './group.type'

export type FanclubKitId = Brand<string, 'fanclub_kit_id'>

export type FanclubKit = {
  id: FanclubKitId
  groupId: GroupId | null
  artistId: ArtistId | null
  name: string
  releaseDate: string
  region: string
  imageUrl: string | null
}

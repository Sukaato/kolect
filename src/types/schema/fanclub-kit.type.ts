import type { ArtistId } from './artist.type'
import type { Brand } from '../brand.type'
import type { GroupId } from './group.type'

export type FanclubKitId = Brand<string, 'fanclub_kit_id'>

export type FanclubKit = {
  id: FanclubKitId
  group_id: GroupId | null
  artist_id: ArtistId | null
  name: string
  release_date: string
  region: string
  image_url: string | null
}

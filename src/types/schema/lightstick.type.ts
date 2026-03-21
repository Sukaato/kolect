import type { ArtistId } from './artist.type'
import type { Brand } from '../brand.type'
import type { GroupId } from './group.type'

export type LightstickId = Brand<string, 'lightstick_id'>

export type Lightstick = {
  id: LightstickId
  group_id: GroupId | null
  artist_id: ArtistId | null
  name: string
  version: string
  release_date: string
  image_url: string | null
}

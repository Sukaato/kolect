import type { Brand } from '../brand.type'
import type { ArtistId } from './artist.type'
import type { GroupId } from './group.type'

export type LightstickId = Brand<string, 'lightstick_id'>

export type Lightstick = {
  id: LightstickId
  groupId: GroupId | null
  artistId: ArtistId | null
  name: string
  version: string
  releaseDate: string
  imageUrl: string | null
}

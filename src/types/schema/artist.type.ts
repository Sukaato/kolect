import type { Brand } from '../brand.type'
import type { AgencyId } from './agency.type'

export type ArtistId = Brand<string, 'artist_id'>

export type Artist = {
  id: ArtistId
  realName: string
  birthDate: string | null
  imageUrl: string | null
  soloDebutDate: string | null
  soloAgencyId: AgencyId | null
}

export type ArtistAliasId = Brand<string, 'artist_alias_id'>
export type AliasKind = 'group_stage' | 'solo_stage' | 'original'

export type ArtistAlias = {
  id: ArtistAliasId
  artistId: ArtistId
  name: string
  kind: AliasKind
  isPrimary: boolean
}

export type ArtistWithAliases = {
  artist: Artist
  aliases: ArtistAlias[]
}

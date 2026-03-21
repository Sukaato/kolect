import type { Artist, ArtistAlias } from './schema/artist.type'

export type ArtistDetail = {
  artist: Artist
  aliases: ArtistAlias[]
}

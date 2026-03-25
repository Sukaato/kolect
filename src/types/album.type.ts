import type { AlbumVersionId } from '@/types/schema/album.type'
import type { ArtistId } from '@/types/schema/artist.type'

export interface AlbumVersionItem {
  id: AlbumVersionId
  name: string
  format: string
  releaseDate: string
  region: string
  imageUrl: string | null
  ownedCount: number
  hasSigned: boolean
}

export interface DigipackItem {
  id: string
  name: string
  artistId: string | null
  releaseDate: string
  region: string
  imageUrl: string | null
  ownedCount: number
  hasSigned: boolean
}

export interface PhotocardItem {
  id: string
  artistId: ArtistId | null
  albumVersionId: AlbumVersionId | null
  digipackId: string | null
  region: string
  imageUrl: string | null
  ownedCount: number
  hasSigned: boolean
}

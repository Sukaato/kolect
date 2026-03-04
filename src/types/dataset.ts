import type { Brand } from './brand.type'
import type { Group, GroupId } from './group'

export enum CollectibleKind {
  ALBUM = 'album',
  LIGHTSTICK = 'lightstick',
  PHOTOCARD = 'photocard',
  MERCH = 'merch',
  FANCLUB_KIT = 'fanclub_kit',
  LUCKY_DRAW = 'lucky_draw',
}

export type CollectibleBase<Id, Kind extends CollectibleKind> = {
  id: Id
  groupId: GroupId
  kind: Kind
  name: string
  image: string | null
  releaseDate: string
  verified: boolean
}

export enum CollectibleRegion {
  KR = 'KR',
  JP = 'JP',
  US = 'US',
  CN = 'CN',
  TW = 'TW',
  GLOBAL = 'GLOBAL',
}

export type AlbumId = Brand<string, 'album_id'>
export enum AlbumType {
  EP = 'EP',
  ALBUM = 'ALBUM',
  SINGLE = 'SINGLE',
  MINI_ALBUM = 'MINI_ALBUM',
  SPECIAL = 'SPECIAL',
  REPACKAGE = 'REPACKAGE',
  OST = 'OST',
}
export enum AlbumFormat {
  CD = 'CD',
  LP = 'LP',
  KIT = 'KIT',
  PLATFORM = 'PLATFORM',
  DIGIPACK = 'DIGIPACK',
  JEWEL = 'JEWEL',
}
export type Album = {
  type: AlbumType

  // Version physique
  version: string | null // "Version A", "Digipack", "Limited"
  region: CollectibleRegion // KR, JP, US...
  format: AlbumFormat | null // CD, LP, Kit, Platform...

  barcode: string | null
} & CollectibleBase<AlbumId, CollectibleKind.ALBUM>

export type LightstickId = Brand<string, 'lightstick_id'>
export type Lightstick = {
  version: string | null
  barcode: string | null
} & CollectibleBase<LightstickId, CollectibleKind.LIGHTSTICK>

export type PhotocardId = Brand<string, 'photocard_id'>
export type Photocard = {
  memberName: string
  albumId: AlbumId | null
  region: CollectibleRegion // KR, JP, US...
} & CollectibleBase<PhotocardId, CollectibleKind.PHOTOCARD>

export type MerchId = Brand<string, 'merch_id'>
export type Merch = {
  category: 'clothing' | 'accessory' | 'figure' | 'other'
} & CollectibleBase<MerchId, CollectibleKind.MERCH>

export type FanclubKitId = Brand<string, 'fanclub_kit_id'>
export type FanclubKit = {
  membershipTerm: string | null
} & CollectibleBase<FanclubKitId, CollectibleKind.FANCLUB_KIT>

export type LuckyDrawId = Brand<string, 'lucky_draw_id'>

export type LuckyDraw = {
  eventName: string
  location: string | null
} & CollectibleBase<LuckyDrawId, CollectibleKind.LUCKY_DRAW>

export type Collectible = Album | Lightstick | Photocard | Merch | FanclubKit | LuckyDraw

export type Dataset = {
  datasetVersion: string
  generatedAt: string
  groups: Group[]
  collectibles: Collectible[]
}

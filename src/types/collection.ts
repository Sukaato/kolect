import type { Brand } from './brand.type'
import type { AlbumId, LightstickId } from './models'

export enum ProductType {
  ALBUM = 'ALBUM',
  LIGHTSTICK = 'LIGHTSTICK',
}

type BaseItem = {
  addedAt: number
}

type AlbumItemId = Brand<string, 'album_item_id'>
type AlbumItem = {
  id: AlbumItemId
  productId: AlbumId
  productType: ProductType.ALBUM
} & BaseItem

type LightstickItemId = Brand<string, 'lightstick_item_id'>
type LightstickItem = {
  id: LightstickItemId
  productId: LightstickId
  productType: ProductType.LIGHTSTICK
} & BaseItem

export type CollectionItem = AlbumItem | LightstickItem

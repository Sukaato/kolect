export enum ProductType {
  ALBUM = 'ALBUM',
  LIGHTSTICK = 'LIGHTSTICK',
}

export type CollectionItem = {
  id: string
  productId: string
  productType: ProductType
  addedAt: number
}

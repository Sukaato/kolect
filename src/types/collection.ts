export enum ProductType {
  Album = 'ALBUM',
  Lightstick = 'LIGHTSTICK',
}

export type CollectionItem = {
  id: string
  productId: string
  productType: ProductType
  addedAt: number
}

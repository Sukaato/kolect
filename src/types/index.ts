export type Group = {
  id: string
  name: string
  agency: string
  debutYear: number
  isActive: boolean
}

export type Album = {
  id: string
  groupId: string
  title: string
  type: 'EP' | 'Album' | 'Single'
  releaseDate: string
  coverImage: string
  barcode: string
  verified: boolean
}

export type Lightstick = {
  id: string
  groupId: string
  name: string
  version: string
  releaseYear: number
  image: string
  verified: boolean
}

export type CollectionItem = {
  id: string
  productId: string
  productType: 'ALBUM' | 'LIGHTSTICK'
  addedAt: number
}

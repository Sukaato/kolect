export enum AlbumType {
  EP = 'EP',
  Album = 'Album',
  Single = 'Single',
}

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
  type: AlbumType
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

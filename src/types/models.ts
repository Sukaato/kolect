export enum AlbumType {
  EP = 'EP',
  ALBUM = 'Album',
  SINGLE = 'Single',
}

type GroupId = string & { __brand: 'group_id' }
export type Group = {
  id: GroupId
  name: string
  agency: string
  debutYear: number
  isActive: boolean
}

type AlbumId = string & { __brand: 'album_id' }
export type Album = {
  id: AlbumId
  groupId: GroupId
  title: string
  type: AlbumType
  releaseDate: string
  coverImage: string
  barcode: string
  verified: boolean
}

type LightstickId = string & { __brand: 'lightstick_id' }
export type Lightstick = {
  id: LightstickId
  groupId: GroupId
  name: string
  version: string
  releaseYear: number
  image: string
  verified: boolean
}

export type Dataset = {
  datasetVersion: string
  generatedAt: string
  groups: Group[]
  albums: Album[]
  lightstick: Lightstick[]
}

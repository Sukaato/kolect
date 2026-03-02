import type { Brand } from './brand.type'

export enum AlbumType {
  EP = 'EP',
  ALBUM = 'Album',
  SINGLE = 'Single',
}

export type GroupId = Brand<string, 'group_id'>
export type Group = {
  id: GroupId
  name: string
  agency: string
  debutYear: number
  isActive: boolean
}

export type AlbumId = Brand<string, 'album_id'>
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

export type LightstickId = Brand<string, 'lightstick_id'>
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
  lightsticks: Lightstick[]
}

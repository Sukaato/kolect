import type { Brand } from '../brand.type'
import type { AgencyId } from './agency.type'

export type GroupId = Brand<string, 'group_id'>

export type Group = {
  id: GroupId
  name: string
  debutDate: string
  fandomName: string | null
  imageUrl: string | null
  agencyId: AgencyId
  isFavorite: boolean
}

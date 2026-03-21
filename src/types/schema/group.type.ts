import type { AgencyId } from './agency.type'
import type { Brand } from '../brand.type'

export type GroupId = Brand<string, 'group_id'>

export type Group = {
  id: GroupId
  name: string
  debut_date: string
  fandom_name: string | null
  image_url: string | null
  agency_id: AgencyId
}

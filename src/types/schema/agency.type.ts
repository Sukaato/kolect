import type { Brand } from '../brand.type'

export type AgencyId = Brand<string, 'agency_id'>

export type Agency = {
  id: AgencyId
  name: string
  country: string
  image_url: string | null
}

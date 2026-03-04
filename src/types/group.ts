import type { Brand } from './brand.type'

export type GroupId = Brand<string, 'group_id'>
export type Group = {
  id: GroupId
  name: string
  agency: string
  debutYear: number
  isActive: boolean
}

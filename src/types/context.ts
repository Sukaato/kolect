import type { Brand } from './brand.type'
import type { GroupId } from './group'

export enum ContextKind {
  TOUR = 'tour',
  ERA = 'era',
  EVENT = 'event',
}

export type ContextId = Brand<string, 'context_id'>
export type Context = {
  id: ContextId
  groupId: GroupId
  kind: ContextKind
  name: string
  startYear: number
  endYear: number
}

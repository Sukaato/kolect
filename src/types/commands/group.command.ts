import type { AlbumSummary, FanclubKitItem, GroupDetail, LightstickItem } from '../group.type'
import type { GroupId } from '../schema/group.type'
import type { Command, InferCommand } from './command.type'

type DetailParam = {
  group_id: GroupId
}
type GroupParams = {
  group_id: GroupId
  include_exclusive_items: boolean
}

type GroupCommand = {
  group_get_detail: Command<DetailParam, GroupDetail>
  group_get_album_summaries: Command<GroupParams, AlbumSummary[]>
  group_get_lightsticks: Command<GroupParams, LightstickItem[]>
  group_get_fanclub_kits: Command<GroupParams, FanclubKitItem[]>
}

declare global {
  interface TauriInvokeCommands extends InferCommand<GroupCommand> {}
}

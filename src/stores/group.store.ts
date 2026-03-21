import { defineStore } from 'pinia'
import { computed, readonly } from 'vue'
import { useEntityStore } from '@/composables/use-entity-store'
import { useInvoke } from '@/composables/use-invoke'
import type { GroupDetail } from '@/types/group.type'
import type { GroupId } from '@/types/schema/group.type'

export const useGroupStore = defineStore('group', () => {
  // ─── Invoke ────────────────────────────────────────────────────────────────

  const detailInvoke = useInvoke<GroupDetail>('group_get_detail', { defaults: null })

  // ─── Logique commune ───────────────────────────────────────────────────────

  const {
    possessionFilter,
    collectiblesLoading,
    albumsError,
    lightsticksError,
    fanclubKitsError,
    filteredAlbums,
    filteredLightsticks,
    filteredFanclubKits,
    loadCollectibles,
  } = useEntityStore({
    albumsCommand: 'group_get_album_summaries',
    lightsticksCommand: 'group_get_lightsticks',
    fanclubKitsCommand: 'group_get_fanclub_kits',
    idParam: 'groupId',
  })

  // ─── Computed ──────────────────────────────────────────────────────────────

  const group = computed(() => detailInvoke.result.value?.group ?? null)
  const members = computed(() => detailInvoke.result.value?.members ?? [])

  const loading = computed(() => detailInvoke.loading.value || collectiblesLoading.value)

  // ─── Actions ───────────────────────────────────────────────────────────────

  async function load(groupId: GroupId) {
    await Promise.all([detailInvoke.invoke({ groupId }), loadCollectibles(groupId)])
  }

  return {
    // State
    possessionFilter,

    // Computed
    group,
    members,
    filteredAlbums,
    filteredLightsticks,
    filteredFanclubKits,

    // Invoke state
    loading: readonly(loading),
    detailError: readonly(detailInvoke.error),
    albumsError,
    lightsticksError,
    fanclubKitsError,

    // Actions
    load,
  }
})

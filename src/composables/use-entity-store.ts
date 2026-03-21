import { computed, readonly, shallowRef } from 'vue'
import { useInvoke } from '@/composables/use-invoke'
import type {
  AlbumSummary,
  FanclubKitItem,
  LightstickItem,
  PossessionFilter,
} from '@/types/group.type'

export interface EntityStoreCommands {
  albumsCommand: string
  lightsticksCommand: string
  fanclubKitsCommand: string
  idParam: string
}

export function useEntityStore(commands: EntityStoreCommands) {
  // ─── Invoke ──────────────────────────────────────────────────────────────

  const albumsInvoke = useInvoke<AlbumSummary[]>(commands.albumsCommand, { defaults: [] })
  const lightsticksInvoke = useInvoke<LightstickItem[]>(commands.lightsticksCommand, {
    defaults: [],
  })
  const fanclubKitsInvoke = useInvoke<FanclubKitItem[]>(commands.fanclubKitsCommand, {
    defaults: [],
  })

  // ─── State ───────────────────────────────────────────────────────────────

  const possessionFilter = shallowRef<PossessionFilter>('all')

  // ─── Computed ────────────────────────────────────────────────────────────

  const albums = computed(() => albumsInvoke.result.value ?? [])
  const lightsticks = computed(() => lightsticksInvoke.result.value ?? [])
  const fanclubKits = computed(() => fanclubKitsInvoke.result.value ?? [])

  const collectiblesLoading = computed(
    () =>
      albumsInvoke.loading.value ||
      lightsticksInvoke.loading.value ||
      fanclubKitsInvoke.loading.value,
  )

  const filteredAlbums = computed(() => {
    if (possessionFilter.value === 'all') return albums.value
    return albums.value.filter(s =>
      possessionFilter.value === 'owned' ? s.ownedCount > 0 : s.ownedCount === 0,
    )
  })

  const filteredLightsticks = computed(() => {
    if (possessionFilter.value === 'all') return lightsticks.value
    return lightsticks.value.filter(ls =>
      possessionFilter.value === 'owned' ? ls.ownedCount > 0 : ls.ownedCount === 0,
    )
  })

  const filteredFanclubKits = computed(() => {
    if (possessionFilter.value === 'all') return fanclubKits.value
    return fanclubKits.value.filter(fk =>
      possessionFilter.value === 'owned' ? fk.ownedCount > 0 : fk.ownedCount === 0,
    )
  })

  // ─── Actions ─────────────────────────────────────────────────────────────

  async function loadCollectibles(id: string) {
    await Promise.all([
      albumsInvoke.invoke({ [commands.idParam]: id }),
      lightsticksInvoke.invoke({ [commands.idParam]: id }),
      fanclubKitsInvoke.invoke({ [commands.idParam]: id }),
    ])
  }

  return {
    // State
    possessionFilter,

    // Computed
    filteredAlbums,
    filteredLightsticks,
    filteredFanclubKits,

    // Invoke state
    collectiblesLoading: readonly(collectiblesLoading),
    albumsError: readonly(albumsInvoke.error),
    lightsticksError: readonly(lightsticksInvoke.error),
    fanclubKitsError: readonly(fanclubKitsInvoke.error),

    // Actions
    loadCollectibles,
  }
}

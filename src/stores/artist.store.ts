import { defineStore } from 'pinia'
import { computed, readonly } from 'vue'
import { useEntityStore } from '@/composables/use-entity-store'
import { useInvoke } from '@/composables/use-invoke'
import type { ArtistId } from '@/types/schema/artist.type'
import { useToast } from '@/composables/use-toast'

export const useArtistStore = defineStore('artist', () => {
  // ─── Composables ───────────────────────────────────────────────────────────

  const toast = useToast()

  // ─── Invoke ────────────────────────────────────────────────────────────────

  const detailInvoke = useInvoke('artist_get_detail', {
    onError(cause) {
      toast.error(cause)
    },
  })

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
    albumsCommand: 'artist_get_album_summaries',
    lightsticksCommand: 'artist_get_lightsticks',
    fanclubKitsCommand: 'artist_get_fanclub_kits',
    idParam: 'artistId',
  })

  // ─── Computed ──────────────────────────────────────────────────────────────

  const artist = computed(() => detailInvoke.data.value?.artist ?? null)
  const aliases = computed(() => detailInvoke.data.value?.aliases ?? [])

  const displayName = computed(() => {
    const a = aliases.value
    return (
      a.find(al => al.kind === 'solo_stage' && al.isPrimary)?.name ??
      a.find(al => al.kind === 'group_stage' && al.isPrimary)?.name ??
      artist.value?.realName ??
      '...'
    )
  })

  const loading = computed(() => detailInvoke.loading.value || collectiblesLoading.value)

  // ─── Actions ───────────────────────────────────────────────────────────────

  async function load(artistId: ArtistId, refresh = false) {
    await Promise.all([
      detailInvoke.invoke({ artistId }, { resetBeforeInvoke: !refresh }),
      loadCollectibles(artistId, refresh),
    ])
  }

  return {
    // State
    possessionFilter,

    // Computed
    artist,
    aliases,
    displayName,
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

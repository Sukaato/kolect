import { defineStore } from 'pinia'
import { computed, readonly, shallowRef } from 'vue'
import { useInvoke } from '@/composables/use-invoke'
import type { AlbumDetail, AlbumVersionItem, DigipackItem, PhotocardItem } from '@/types/album.type'
import type { ArtistDetail } from '@/types/artist.type'
import type { GroupDetail, PossessionFilter } from '@/types/group.type'
import type { AlbumId, AlbumVersionId } from '@/types/schema/album.type'
import type { ArtistId, ArtistWithAliases } from '@/types/schema/artist.type'

export const useAlbumStore = defineStore('album', () => {
  // ─── Invoke ────────────────────────────────────────────────────────────────

  const detailInvoke = useInvoke<AlbumDetail>('album_get_detail', { defaults: null })
  const versionsInvoke = useInvoke<AlbumVersionItem[]>('album_get_versions', { defaults: [] })
  const digipacksInvoke = useInvoke<DigipackItem[]>('album_get_digipacks', { defaults: [] })
  const photocardsInvoke = useInvoke<PhotocardItem[]>('album_get_photocards', { defaults: [] })
  const groupDetailInvoke = useInvoke<GroupDetail>('group_get_detail', { defaults: null })
  const artistDetailInvoke = useInvoke<ArtistDetail>('artist_get_detail', { defaults: null })

  // ─── State ─────────────────────────────────────────────────────────────────

  const possessionFilter = shallowRef<PossessionFilter>('all')
  const selectedMemberTab = shallowRef<ArtistId | 'all'>('all')
  const selectedVersionFilter = shallowRef<AlbumVersionId | 'all'>('all')

  // ─── Computed ──────────────────────────────────────────────────────────────

  const detail = computed(() => detailInvoke.result.value)
  const versions = computed(() => versionsInvoke.result.value ?? [])
  const digipacks = computed(() => digipacksInvoke.result.value ?? [])
  const photocards = computed(() => photocardsInvoke.result.value ?? [])

  const members = computed<ArtistWithAliases[]>(() => {
    if (groupDetailInvoke.result.value) {
      return groupDetailInvoke.result.value.members
    }
    if (artistDetailInvoke.result.value) {
      return [
        {
          artist: artistDetailInvoke.result.value.artist,
          aliases: artistDetailInvoke.result.value.aliases,
        },
      ]
    }
    return []
  })

  // true = album d'artiste solo → cacher les tabs membres dans PhotocardSection
  const isSolo = computed(() => !!detail.value?.artistId && !detail.value?.groupId)

  const loading = computed(
    () =>
      detailInvoke.loading.value ||
      versionsInvoke.loading.value ||
      digipacksInvoke.loading.value ||
      photocardsInvoke.loading.value ||
      groupDetailInvoke.loading.value ||
      artistDetailInvoke.loading.value,
  )

  const progressPercent = computed(() => {
    if (!detail.value || detail.value.totalCount === 0) return 0
    return Math.round((detail.value.ownedCount / detail.value.totalCount) * 100)
  })

  const filteredVersions = computed(() => {
    if (possessionFilter.value === 'all') return versions.value
    return versions.value.filter(v =>
      possessionFilter.value === 'owned' ? v.ownedCount > 0 : v.ownedCount === 0,
    )
  })

  const filteredDigipacks = computed(() => {
    if (possessionFilter.value === 'all') return digipacks.value
    return digipacks.value.filter(d =>
      possessionFilter.value === 'owned' ? d.ownedCount > 0 : d.ownedCount === 0,
    )
  })

  const filteredPhotocards = computed(() => {
    let result = photocards.value

    if (selectedMemberTab.value !== 'all') {
      result = result.filter(pc => pc.artistId === selectedMemberTab.value)
    }

    if (selectedVersionFilter.value !== 'all') {
      result = result.filter(pc => pc.albumVersionId === selectedVersionFilter.value)
    }

    if (possessionFilter.value !== 'all') {
      result = result.filter(pc =>
        possessionFilter.value === 'owned' ? pc.ownedCount > 0 : pc.ownedCount === 0,
      )
    }

    return result
  })

  const photocardCountByMember = computed(() => {
    const counts = new Map<ArtistId, number>()
    for (const pc of photocards.value) {
      if (!pc.artistId) continue
      counts.set(pc.artistId, (counts.get(pc.artistId) ?? 0) + 1)
    }
    return counts
  })

  // ─── Actions ───────────────────────────────────────────────────────────────

  async function load(albumId: AlbumId) {
    // Reset les membres immédiatement pour éviter d'afficher
    // les données du précédent album pendant le chargement
    groupDetailInvoke.result.value = null
    artistDetailInvoke.result.value = null
    resetFilters()

    await Promise.all([
      detailInvoke.invoke({ albumId }),
      versionsInvoke.invoke({ albumId }),
      digipacksInvoke.invoke({ albumId }),
      photocardsInvoke.invoke({ albumId }),
    ])

    const d = detailInvoke.result.value
    if (!d) return

    if (d.groupId) {
      await groupDetailInvoke.invoke({ groupId: d.groupId })
    } else if (d.artistId) {
      await artistDetailInvoke.invoke({ artistId: d.artistId })
    }
  }

  function resetFilters() {
    possessionFilter.value = 'all'
    selectedMemberTab.value = 'all'
    selectedVersionFilter.value = 'all'
  }

  return {
    // State
    possessionFilter,
    selectedMemberTab,
    selectedVersionFilter,

    // Computed
    detail,
    versions,
    digipacks,
    members,
    isSolo,
    progressPercent,
    filteredVersions,
    filteredDigipacks,
    filteredPhotocards,
    photocardCountByMember,

    // Invoke state
    loading: readonly(loading),
    detailError: readonly(detailInvoke.error),
    versionsError: readonly(versionsInvoke.error),
    digipacksError: readonly(digipacksInvoke.error),
    photocardsError: readonly(photocardsInvoke.error),

    // Actions
    load,
    resetFilters,
  }
})

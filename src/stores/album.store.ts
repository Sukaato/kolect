import { defineStore } from 'pinia'
import { computed, readonly, shallowRef } from 'vue'
import { useInvoke } from '@/composables/use-invoke'
import { useToast } from '@/composables/use-toast'
import { useSettingStore } from '@/stores/setting.store'
import type { PossessionFilter } from '@/types/group.type'
import type { AlbumId, AlbumVersionId } from '@/types/schema/album.type'
import type { ArtistId, ArtistWithAliases } from '@/types/schema/artist.type'

export const useAlbumStore = defineStore('album', () => {
  // ─── Composables ──────────────────────────────────────────────────────────

  const toast = useToast()
  const settingStore = useSettingStore()

  // ─── Invoke ───────────────────────────────────────────────────────────────

  const detailInvoke = useInvoke('album_get_detail', {
    onError(cause) {
      toast.error(cause)
    },
  })
  const versionsInvoke = useInvoke('album_get_versions', {
    defaults: [],
    onError(cause) {
      toast.error(cause)
    },
  })
  const digipacksInvoke = useInvoke('album_get_digipacks', {
    defaults: [],
    onError(cause) {
      toast.error(cause)
    },
  })
  const photocardsInvoke = useInvoke('album_get_photocards', {
    defaults: [],
    onError(cause) {
      toast.error(cause)
    },
  })
  const groupDetailInvoke = useInvoke('group_get_detail', {
    onError(cause) {
      toast.error(cause)
    },
  })
  const artistDetailInvoke = useInvoke('artist_get_detail', {
    onError(cause) {
      toast.error(cause)
    },
  })

  // ─── State ────────────────────────────────────────────────────────────────

  const possessionFilter = shallowRef<PossessionFilter>('all')
  const selectedMemberTab = shallowRef<ArtistId | 'all'>('all')
  const selectedVersionFilter = shallowRef<AlbumVersionId | 'all'>('all')

  // ─── Computed ─────────────────────────────────────────────────────────────

  const detail = computed(() => detailInvoke.data.value)
  const versions = computed(() => versionsInvoke.data.value ?? [])
  const digipacks = computed(() => digipacksInvoke.data.value ?? [])
  const photocards = computed(() => photocardsInvoke.data.value ?? [])

  const members = computed<ArtistWithAliases[]>(() => {
    if (groupDetailInvoke.data.value) {
      return groupDetailInvoke.data.value.members
    }
    if (artistDetailInvoke.data.value) {
      return [
        {
          artist: artistDetailInvoke.data.value.artist,
          aliases: artistDetailInvoke.data.value.aliases,
        },
      ]
    }
    return []
  })

  // true = solo artist album → hide member tabs in PhotocardSection
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

  // ─── Progress ─────────────────────────────────────────────────────────────

  const versionsProgress = computed(() => ({
    owned: detail.value?.versionsOwnedCount ?? 0,
    total: detail.value?.versionsTotalCount ?? 0,
    percent:
      detail.value && detail.value.versionsTotalCount > 0
        ? Math.round((detail.value.versionsOwnedCount / detail.value.versionsTotalCount) * 100)
        : 0,
  }))

  const digipacksProgress = computed(() => ({
    owned: detail.value?.digipacksOwnedCount ?? 0,
    total: detail.value?.digipacksTotalCount ?? 0,
    percent:
      detail.value && detail.value.digipacksTotalCount > 0
        ? Math.round((detail.value.digipacksOwnedCount / detail.value.digipacksTotalCount) * 100)
        : 0,
  }))

  // Only relevant when the includePhotocards setting is enabled
  const photocardsProgress = computed(() => ({
    owned: detail.value?.photocardsOwnedCount ?? 0,
    total: detail.value?.photocardsTotalCount ?? 0,
    percent:
      detail.value && detail.value.photocardsTotalCount > 0
        ? Math.round((detail.value.photocardsOwnedCount / detail.value.photocardsTotalCount) * 100)
        : 0,
  }))

  const showPhotocardsProgress = computed(
    () => settingStore.includePhotocardInCount && (detail.value?.photocardsTotalCount ?? 0) > 0,
  )

  // ─── Filtered lists ───────────────────────────────────────────────────────

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

  // ─── Actions ──────────────────────────────────────────────────────────────

  async function load(albumId: AlbumId, refresh = false) {
    // Reset members immediately to avoid showing stale data from the previous album
    groupDetailInvoke.data.value = null
    artistDetailInvoke.data.value = null
    resetFilters()

    await Promise.all([
      detailInvoke.invoke(
        { albumId, includeExclusiveItems: settingStore.includeExclusiveItems },
        { resetBeforeInvoke: !refresh },
      ),
      versionsInvoke.invoke(
        { albumId, includeExclusiveItems: settingStore.includeExclusiveItems },
        { resetBeforeInvoke: !refresh },
      ),
      digipacksInvoke.invoke(
        { albumId, includeExclusiveItems: settingStore.includeExclusiveItems },
        { resetBeforeInvoke: !refresh },
      ),
      photocardsInvoke.invoke(
        { albumId, includeExclusiveItems: settingStore.includeExclusiveItems },
        { resetBeforeInvoke: !refresh },
      ),
    ])

    const d = detailInvoke.data.value
    if (!d) return

    if (d.groupId) {
      await groupDetailInvoke.invoke({ groupId: d.groupId }, { resetBeforeInvoke: !refresh })
    } else if (d.artistId) {
      await artistDetailInvoke.invoke({ artistId: d.artistId }, { resetBeforeInvoke: !refresh })
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
    versionsProgress,
    digipacksProgress,
    photocardsProgress,
    showPhotocardsProgress,
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

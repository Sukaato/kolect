import { defineStore } from 'pinia'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useInvoke } from '@/composables/use-invoke'
import { useToast } from '@/composables/use-toast'
import type { ArtistId } from '@/types/schema/artist.type'
import type { GroupId } from '@/types/schema/group.type'
import { useCollectionStore } from './collection.store'
import { useDatasetStore } from './dataset.store'

export const useFavoriteStore = defineStore('favorite', () => {
  const { t } = useI18n()
  const toast = useToast()
  const collectionStore = useCollectionStore()
  const datasetStore = useDatasetStore()

  // ─── Invoke ────────────────────────────────────────────────────────────────

  const toggleGroupInvoke = useInvoke('favorite_toggle_group')
  const toggleArtistInvoke = useInvoke('favorite_toggle_artist')

  // ─── Computed ──────────────────────────────────────────────────────────────

  const loading = computed(
    () => toggleGroupInvoke.loading.value || toggleArtistInvoke.loading.value,
  )

  // ─── Actions ───────────────────────────────────────────────────────────────

  async function toggleGroup(groupId: GroupId): Promise<boolean | null> {
    const [error, isFavorite] = await toggleGroupInvoke.invoke({ groupId })

    if (error) {
      toast.error(error)
      return null
    }

    await Promise.all([datasetStore.refresh(), collectionStore.refresh()])
    toast.success(
      isFavorite ? t('screen.group.favorite.added') : t('screen.group.favorite.removed'),
    )

    return isFavorite
  }

  async function toggleArtist(artistId: ArtistId): Promise<boolean | null> {
    const [error, isFavorite] = await toggleArtistInvoke.invoke({ artistId })

    if (error) {
      toast.error(error)
      return null
    }

    toast.success(
      isFavorite ? t('screen.artist.favorite.added') : t('screen.artist.favorite.removed'),
    )

    await Promise.all([datasetStore.refresh(), collectionStore.refresh()])

    return isFavorite
  }

  return {
    // Invoke state
    loading,

    // Actions
    toggleGroup,
    toggleArtist,
  }
})

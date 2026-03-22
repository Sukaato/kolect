// src/stores/possession.store.ts

import { defineStore } from 'pinia'
import { computed, readonly, shallowRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { useInvoke } from '@/composables/use-invoke'
import { useToast } from '@/composables/use-toast'

export type CollectibleType =
  | 'album'
  | 'albumVersion'
  | 'digipack'
  | 'lightstick'
  | 'fanclubKit'
  | 'photocard'

export interface PossessionItem {
  type: CollectibleType
  id: string
  name: string
  imageUrl?: string | null
  ownedCount: number
  signedCount: number
  hasSigned: boolean
  maxCount?: number
  onSaved?: () => void
}

export const usePossessionStore = defineStore('possession', () => {
  const { t } = useI18n()
  const toast = useToast()

  // ─── Invoke ────────────────────────────────────────────────────────────────

  const updateInvoke = useInvoke<void>('collection_update_item', {
    defaults: null,
    showErrorToast: false,
  })

  // ─── State ─────────────────────────────────────────────────────────────────

  const isOpen = shallowRef(false)
  const item = shallowRef<PossessionItem | null>(null)
  const ownedCount = shallowRef(0)
  const signedCount = shallowRef(0)

  // ─── Computed ──────────────────────────────────────────────────────────────

  const loading = computed(() => updateInvoke.loading.value)

  const canIncrease = computed(
    () => item.value?.maxCount === undefined || ownedCount.value < item.value.maxCount,
  )
  const canDecrease = computed(() => ownedCount.value > 0)
  const canDecreaseSign = computed(() => signedCount.value > 0)
  const canIncreaseSign = computed(() => signedCount.value < ownedCount.value)
  const isDirty = computed(
    () =>
      item.value !== null &&
      (ownedCount.value !== item.value.ownedCount || signedCount.value !== item.value.signedCount),
  )

  // ─── Actions ───────────────────────────────────────────────────────────────

  function open(newItem: PossessionItem) {
    item.value = newItem
    ownedCount.value = newItem.ownedCount
    signedCount.value = newItem.signedCount
    isOpen.value = true
  }

  function close() {
    isOpen.value = false
    item.value = null
    ownedCount.value = 0
    signedCount.value = 0
  }

  function incrementOwned() {
    if (!canIncrease.value) return
    ownedCount.value++
  }

  function decrementOwned() {
    if (!canDecrease.value) return
    ownedCount.value--
    if (signedCount.value > ownedCount.value) {
      signedCount.value = ownedCount.value
    }
  }

  function incrementSigned() {
    if (!canIncreaseSign.value) return
    signedCount.value++
  }

  function decrementSigned() {
    if (!canDecreaseSign.value) return
    signedCount.value--
  }

  async function save(): Promise<boolean> {
    if (!item.value) return false

    const [error] = await updateInvoke.invoke({
      itemType: item.value.type,
      itemId: item.value.id,
      ownedCount: ownedCount.value,
      signedCount: signedCount.value,
    })

    if (error) {
      toast.error(error)
      return false
    }

    toast.success(t('modals.possession.saved'))

    // Notifier le contexte appelant pour qu'il recharge ses données
    item.value.onSaved?.()

    close()
    return true
  }

  return {
    // State
    isOpen: readonly(isOpen),
    item: readonly(item),
    ownedCount: readonly(ownedCount),
    signedCount: readonly(signedCount),

    // Computed
    loading,
    canIncrease,
    canDecrease,
    canDecreaseSign,
    canIncreaseSign,
    isDirty,

    // Actions
    open,
    close,
    incrementOwned,
    decrementOwned,
    incrementSigned,
    decrementSigned,
    save,
  }
})

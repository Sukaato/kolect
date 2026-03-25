<script setup lang="ts">
import { usePossessionStore } from '@/stores/possession.store'
import { MinusIcon, PlusIcon, XIcon } from '@lucide/vue'
import { storeToRefs } from 'pinia'

const store = usePossessionStore()
const {
  isOpen,
  item,
  ownedCount,
  signedCount,
  loading,
  canIncrease,
  canDecrease,
  canDecreaseSign,
  canIncreaseSign,
  isDirty,
} = storeToRefs(store)
</script>

<template>
  <Teleport to="body">
    <dialog :open="isOpen" class="modal modal-bottom sm:modal-middle">
      <div class="modal-box" v-if="item">

        <!-- Header -->
        <div class="flex items-center gap-3 mb-6">
          <img v-if="item.imageUrl" :src="item.imageUrl" :alt="item.name"
            class="w-12 h-12 rounded-lg object-cover shrink-0" />
          <div v-else class="w-12 h-12 rounded-lg bg-base-300 shrink-0" />

          <div class="flex-1 min-w-0">
            <p class="text-xs text-base-content/50 uppercase tracking-widest">
              {{ $t(`modal.possession.type.${item.type}`) }}
            </p>
            <h3 class="font-bold truncate">{{ item.name }}</h3>
          </div>

          <button class="btn btn-ghost btn-sm btn-circle" @click="store.close()">
            <XIcon class="w-4 h-4" />
          </button>
        </div>

        <!-- Owned count -->
        <div class="flex items-center justify-between py-4 border-b border-base-300">
          <div>
            <p class="font-semibold">{{ $t('modal.possession.owned') }}</p>
            <p class="text-xs text-base-content/50">
              {{ item.maxCount !== undefined
                ? $t('modal.possession.owned_hint_max', { max: item.maxCount })
                : $t('modal.possession.owned_hint') }}
            </p>
          </div>
          <div class="flex items-center gap-3">
            <button class="btn btn-circle btn-sm btn-outline" :disabled="!canDecrease || loading"
              @click="store.decrementOwned()">
              <MinusIcon class="w-4 h-4" />
            </button>
            <span class="text-2xl font-bold w-8 text-center tabular-nums">
              {{ ownedCount }}
            </span>
            <button class="btn btn-circle btn-sm btn-outline" :disabled="!canIncrease || loading"
              @click="store.incrementOwned()">
              <PlusIcon class="w-4 h-4" />
            </button>
          </div>
        </div>

        <!-- Signed count -->
        <div v-if="item.hasSigned" class="flex items-center justify-between py-4 border-b border-base-300">
          <div>
            <p class="font-semibold">{{ $t('modal.possession.signed') }}</p>
            <p class="text-xs text-base-content/50">{{ $t('modal.possession.signed_hint') }}</p>
          </div>
          <div class="flex items-center gap-3">
            <button class="btn btn-circle btn-sm btn-outline" :disabled="!canDecreaseSign || loading"
              @click="store.decrementSigned()">
              <MinusIcon class="w-4 h-4" />
            </button>
            <span class="text-2xl font-bold w-8 text-center tabular-nums">
              {{ signedCount }}
            </span>
            <button class="btn btn-circle btn-sm btn-outline" :disabled="!canIncreaseSign || loading"
              @click="store.incrementSigned()">
              <PlusIcon class="w-4 h-4" />
            </button>
          </div>
        </div>

        <!-- Actions -->
        <div class="modal-action mt-4">
          <button class="btn btn-ghost" :disabled="loading" @click="store.close()">
            {{ $t('common.action.cancel') }}
          </button>
          <button class="btn btn-primary" :disabled="!isDirty || loading" :class="{ loading }" @click="store.save()">
            {{ $t('common.action.save') }}
          </button>
        </div>

      </div>

      <form method="dialog" class="modal-backdrop">
        <button @click="store.close()">close</button>
      </form>
    </dialog>
  </Teleport>
</template>
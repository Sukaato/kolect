<script setup lang="ts">
import { usePossessionStore } from '@/stores/possession.store'
import type { FanclubKitItem } from '@/types/group.type'

const { fanclubKit, onSaved } = defineProps<{
  fanclubKit: FanclubKitItem
  onSaved?: () => void
}>()

const possessionStore = usePossessionStore()

function handleClick() {
  possessionStore.open({
    type: 'fanclubKit',
    id: fanclubKit.id,
    name: fanclubKit.name,
    imageUrl: fanclubKit.imageUrl,
    ownedCount: fanclubKit.ownedCount,
    signedCount: 0,
    hasSigned: false,
    maxCount: 1,
    onSaved,
  })
}
</script>

<template>
  <div class="shrink-0 w-20 text-center cursor-pointer" @click="handleClick">
    <div
      class="relative w-20 h-20 rounded-xl bg-base-100 border flex items-center justify-center text-2xl transition-colors active:opacity-70"
      :class="fanclubKit.ownedCount > 0 ? 'border-success bg-success/10' : 'border-base-300'">
      <img v-if="fanclubKit.imageUrl" :src="fanclubKit.imageUrl" :alt="fanclubKit.name"
        class="w-full h-full object-cover rounded-xl" />
      <span v-else>🎁</span>
      <span v-if="fanclubKit.ownedCount > 1"
        class="absolute top-1 right-1 bg-primary text-white text-[9px] font-bold w-4 h-4 rounded-full flex items-center justify-center">
        {{ fanclubKit.ownedCount }}
      </span>
      <span v-else-if="fanclubKit.ownedCount === 1"
        class="absolute bottom-1 right-1 w-2.5 h-2.5 rounded-full bg-success border-2 border-base-100" />
    </div>
    <p class="text-[10px] text-base-content/60 mt-1.5 truncate">{{ fanclubKit.name }}</p>
  </div>
</template>
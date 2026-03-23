<script setup lang="ts">
import { useSettingStore } from '@/stores/setting.store'
import type { AlbumSummary } from '@/types/group.type'
import { computed } from 'vue'

const { summary } = defineProps<{
  summary: AlbumSummary
}>()

const settingStore = useSettingStore()

const ownedCount = computed(() => summary.versionsOwnedCount + summary.digipacksOwnedCount)
const totalCount = computed(() => summary.versionsTotalCount + summary.digipacksTotalCount)
const isComplete = computed(() => ownedCount.value === totalCount.value && totalCount.value > 0)

const showPhotocards = computed(
  () => settingStore.includePhotocardCount && summary.photocardsOwnedCount > 0,
)
</script>

<template>
  <div class="album-card shrink-0 w-24 cursor-pointer group">
    <div
      class="relative w-24 h-24 rounded-xl bg-base-200 border border-base-300 flex items-center justify-center overflow-hidden group-active:opacity-70 transition-opacity">
      <img v-if="summary.imageUrl" :src="summary.imageUrl" :alt="summary.name" class="w-full h-full object-cover" />
      <span v-else class="text-3xl">🎵</span>

      <!-- Versions + digipacks badge -->
      <span class="absolute top-1.5 right-1.5 text-white text-[10px] font-semibold px-1.5 py-0.5 rounded-full"
        :class="isComplete ? 'bg-success' : 'bg-primary'">
        {{ ownedCount }}/{{ totalCount }}
      </span>

      <!-- Photocards badge (setting-gated, owned only) -->
      <span v-if="showPhotocards"
        class="absolute bottom-1.5 left-1.5 text-white text-[9px] font-semibold px-1 py-0.5 rounded-full bg-accent">
        🃏 {{ summary.photocardsOwnedCount }}
      </span>
    </div>
    <p class="text-[11px] font-semibold mt-1.5 truncate">{{ summary.name }}</p>
  </div>
</template>
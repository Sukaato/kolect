<script setup lang="ts">
import { useAlbumStore } from '@/stores/album.store'
import { storeToRefs } from 'pinia'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import PhotocardCard from './PhotocardCard.vue'

const { t } = useI18n()

const albumStore = useAlbumStore()
const {
  detail,
  members,
  versions,
  isSolo,
  selectedMemberTab,
  selectedVersionFilter,
  filteredPhotocards,
  photocardCountByMember,
} = storeToRefs(albumStore)

const memberTabs = computed(() => [
  { id: 'all', label: t('screen.album.photocard.tab.all'), count: filteredPhotocards.value.length } as const,
  ...members.value.map(m => ({
    id: m.artist.id,
    label: m.aliases.find(a => a.kind === 'group_stage' && a.isPrimary)?.name ?? m.artist.realName,
    count: photocardCountByMember.value.get(m.artist.id) ?? 0,
  } as const)),
])

const versionOptions = computed(() => [
  { id: 'all', label: t('screen.album.photocard.version_filter.all') } as const,
  ...versions.value.map(v => ({ id: v.id, label: v.name } as const)),
])
</script>

<template>
  <section class="album--photocard-section">
    <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
      {{ $t('screen.album.section.photocards') }}
    </h2>

    <div class="bg-base-100 rounded-2xl border border-base-300 overflow-hidden">

      <!-- Member tabs — hidden for solo artists -->
      <div v-if="!isSolo" class="flex overflow-x-auto scrollbar-none border-b border-base-300">
        <button v-for="tab in memberTabs" :key="tab.id"
          class="shrink-0 px-4 py-2.5 text-sm font-medium border-b-2 transition-colors whitespace-nowrap" :class="selectedMemberTab === tab.id
            ? 'border-primary text-primary'
            : 'border-transparent text-base-content/50'" @click="selectedMemberTab = tab.id">
          {{ tab.label }}
          <span class="ml-1 text-[10px] opacity-60">({{ tab.count }})</span>
        </button>
      </div>

      <!-- Version filter -->
      <div class="px-3 pt-3">
        <div class="flex gap-2 flex-wrap">
          <button v-for="opt in versionOptions" :key="opt.id" class="btn btn-xs rounded-full"
            :class="selectedVersionFilter === opt.id ? 'btn-primary' : 'btn-ghost'"
            @click="selectedVersionFilter = opt.id">
            {{ opt.label }}
          </button>
        </div>
      </div>

      <!-- Grid -->
      <div class="grid grid-cols-4 gap-2 p-3">
        <PhotocardCard v-for="pc in filteredPhotocards" :key="pc.id" :album-id="detail!.albumId" :photocard="pc"
          :members="members" :versions="versions" />
      </div>

      <!-- Empty state -->
      <div v-if="!filteredPhotocards.length" class="py-10 text-center text-sm text-base-content/40">
        {{ $t('screen.album.photocard.empty') }}
      </div>

    </div>
  </section>
</template>

<style scoped>
.scrollbar-none::-webkit-scrollbar {
  display: none;
}

.scrollbar-none {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
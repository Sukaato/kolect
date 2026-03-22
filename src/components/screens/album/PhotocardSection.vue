<script setup lang="ts">
import PhotocardCard from '@/components/screens/album/PhotocardCard.vue'
import { useAlbumStore } from '@/stores/album.store'
import { storeToRefs } from 'pinia'
import { computed } from 'vue'

// ─── Config ───────────────────────────────────────────────────────────────────
const PHOTOCARD_FILTER_MODE: 'chips' | 'dropdown' = 'chips'
// ─────────────────────────────────────────────────────────────────────────────

const albumStore = useAlbumStore()
const {
  detail,
  members,
  versions,
  selectedMemberTab,
  selectedVersionFilter,
  filteredPhotocards,
  photocardCountByMember,
} = storeToRefs(albumStore)

const memberTabs = computed(() => [
  { id: 'all', label: 'Toutes', count: filteredPhotocards.value.length } as const,
  ...members.value.map(m => ({
    id: m.artist.id,
    label: m.aliases.find(a => a.kind === 'group_stage' && a.isPrimary)?.name ?? m.artist.realName,
    count: photocardCountByMember.value.get(m.artist.id) ?? 0,
  } as const)),
])

const versionOptions = computed(() => [
  { id: 'all', label: 'Toutes versions' } as const,
  ...versions.value.map(v => ({ id: v.id, label: v.name } as const)),
])
</script>

<template>
  <section>
    <h2 class="text-xs font-semibold uppercase tracking-widest text-base-content/50 mb-3">
      {{ $t('screens.album.sections.photocards') }}
    </h2>

    <div class="bg-base-100 rounded-2xl border border-base-300 overflow-hidden">

      <!-- Onglets membres -->
      <div class="flex overflow-x-auto scrollbar-none border-b border-base-300">
        <button v-for="tab in memberTabs" :key="tab.id"
          class="shrink-0 px-4 py-2.5 text-sm font-medium border-b-2 transition-colors whitespace-nowrap" :class="selectedMemberTab === tab.id
            ? 'border-primary text-primary'
            : 'border-transparent text-base-content/50'" @click="selectedMemberTab = tab.id">
          {{ tab.label }}
          <span class="ml-1 text-[10px] opacity-60">({{ tab.count }})</span>
        </button>
      </div>

      <!-- Filtre par version -->
      <div class="px-3 pt-3">
        <div v-if="PHOTOCARD_FILTER_MODE === 'chips'" class="flex gap-2 flex-wrap">
          <button v-for="opt in versionOptions" :key="opt.id" class="btn btn-xs rounded-full"
            :class="selectedVersionFilter === opt.id ? 'btn-primary' : 'btn-ghost'"
            @click="selectedVersionFilter = opt.id">
            {{ opt.label }}
          </button>
        </div>

        <select v-else v-model="selectedVersionFilter" class="select select-sm select-bordered w-full">
          <option v-for="opt in versionOptions" :key="opt.id" :value="opt.id">
            {{ opt.label }}
          </option>
        </select>
      </div>

      <!-- Grille -->
      <div class="grid grid-cols-4 gap-2 p-3">
        <PhotocardCard v-for="pc in filteredPhotocards" :key="pc.id" :album-id="detail!.albumId" :photocard="pc"
          :members="members" :versions="versions" />
      </div>

      <div v-if="!filteredPhotocards.length" class="py-10 text-center text-sm text-base-content/40">
        {{ $t('screens.album.photocards.empty') }}
      </div>

    </div>
  </section>
</template>
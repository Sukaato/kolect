<script setup lang="ts">
import type { ArtistWithAliases } from '@/types/schema/artist.type';
import { ChevronRightIcon } from 'lucide-vue-next';
import { computed } from 'vue';

const { member } = defineProps<{
  member: ArtistWithAliases
}>()

const stageName = computed(() =>
  member.aliases.find(a => a.kind === 'group_stage' && a.isPrimary)?.name
  ?? member.artist.realName
)
</script>

<template>
  <button class="w-full flex items-center gap-4 px-5 py-3 hover:bg-base-200 transition-colors active:bg-base-300">
    <div
      class="w-11 h-11 rounded-full bg-base-200 border border-base-300 shrink-0 overflow-hidden flex items-center justify-center text-base font-semibold text-base-content/40">
      <img v-if="member.artist.imageUrl" :src="member.artist.imageUrl" :alt="stageName"
        class="w-full h-full object-cover" />
      <span v-else>{{ stageName?.charAt(0) }}</span>
    </div>

    <div class="flex-1 text-left min-w-0">
      <p class="font-semibold text-sm truncate">{{ stageName }}</p>
      <p class="text-xs text-base-content/50 truncate">{{ member.artist.realName }}</p>
    </div>

    <ChevronRightIcon class="w-4 h-4 text-base-content/30 shrink-0" />
  </button>
</template>
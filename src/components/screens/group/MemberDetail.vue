<script setup lang="ts">
import { ExternalLinkIcon } from 'lucide-vue-next'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { ArtistId, ArtistWithAliases } from '@/types/schema/artist.type';

const { member } = defineProps<{
  member: ArtistWithAliases
}>()

const emit = defineEmits<{
  navigateToSolo: [artistId: ArtistId]
}>()

const { locale } = useI18n()

const stageName = computed(() =>
  member.aliases.find(a => a.kind === 'group_stage' && a.isPrimary)?.name
  ?? member.artist.realName
)

const originalName = computed(() =>
  member.aliases.find(a => a.kind === 'original')?.name ?? null
)

const soloStageName = computed(() =>
  member.aliases.find(a => a.kind === 'solo_stage' && a.isPrimary)?.name ?? null
)

const birthDate = computed(() => {
  if (!member.artist.birthDate) return null
  return Intl.DateTimeFormat(locale.value, {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  }).format(new Date(member.artist.birthDate))
})

const soloDebutDate = computed(() => {
  if (!member.artist.soloDebutDate) return null
  return Intl.DateTimeFormat(locale.value, {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  }).format(new Date(member.artist.soloDebutDate))
})

const hasSoloCareer = computed(() => !!member.artist.soloDebutDate)
</script>

<template>
  <div class="px-5 py-4 space-y-6 pb-8">

    <!-- Avatar + noms -->
    <div class="flex items-center gap-4">
      <div
        class="w-16 h-16 rounded-full bg-base-200 border border-base-300 shrink-0 overflow-hidden flex items-center justify-center text-2xl font-semibold text-base-content/40">
        <!-- <img v-if="member.artist.imageUrl" :src="member.artist.imageUrl" :alt="stageName"
          class="w-full h-full object-cover" /> -->
        <img v-if="member.artist.imageUrl" src="https://picsum.photos/200/200" :alt="member.artist.realName"
          class="w-full h-full object-cover" />
        <span v-else>{{ stageName?.charAt(0) }}</span>
      </div>

      <div class="min-w-0">
        <p class="text-lg font-bold leading-tight">{{ stageName }}</p>
        <p class="text-sm text-base-content/50">{{ member.artist.realName }}</p>
        <p v-if="originalName" class="text-sm text-base-content/40">{{ originalName }}</p>
      </div>
    </div>

    <!-- Infos -->
    <div class="bg-base-200/50 rounded-xl overflow-hidden divide-y divide-base-300">

      <div v-if="birthDate" class="flex justify-between items-center px-4 py-3 text-sm">
        <span class="text-base-content/50">{{ $t('screens.group.members.detail.birth_date') }}</span>
        <span class="font-medium">{{ birthDate }}</span>
      </div>

      <!-- TODO: roles viennent de GroupMember, à enrichir ArtistWithAliases si besoin -->

      <div v-if="hasSoloCareer && soloStageName" class="flex justify-between items-center px-4 py-3 text-sm">
        <span class="text-base-content/50">{{ $t('screens.group.members.detail.solo_name') }}</span>
        <span class="font-medium">{{ soloStageName }}</span>
      </div>

      <div v-if="hasSoloCareer && soloDebutDate" class="flex justify-between items-center px-4 py-3 text-sm">
        <span class="text-base-content/50">{{ $t('screens.group.members.detail.solo_debut') }}</span>
        <span class="font-medium">{{ soloDebutDate }}</span>
      </div>

    </div>

    <!-- Stats photocards -->
    <!-- TODO: nécessite une commande Tauri member_get_photocard_stats -->
    <!--
    <div class="bg-base-200/50 rounded-xl px-4 py-3">
      <p class="text-xs text-base-content/50 uppercase tracking-widest mb-2">
        {{ $t('screens.group.members.detail.photocards') }}
      </p>
      <p class="text-2xl font-bold">{{ ownedCount }} <span class="text-sm font-normal text-base-content/40">/ {{ totalCount }}</span></p>
    </div>
    -->

    <!-- Carrière solo -->
    <button v-if="hasSoloCareer" class="btn btn-outline btn-sm w-full gap-2"
      @click="emit('navigateToSolo', member.artist.id)">
      <ExternalLinkIcon class="w-4 h-4" />
      {{ $t('screens.group.members.detail.solo_career') }}
    </button>

  </div>
</template>
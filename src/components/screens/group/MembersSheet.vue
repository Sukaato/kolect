<script setup lang="ts">
import { ChevronLeftIcon } from 'lucide-vue-next';
import { shallowRef } from 'vue';
import { useRouter } from 'vue-router';
import MemberDetail from './MemberDetail.vue';
import BottomSheet from '@/components/global/BottomSheet.vue';
import MemberListItem from '@/components/screens/group/MemberListItem.vue';
import { RouteName } from '@/types/routes';
import type { ArtistId, ArtistWithAliases } from '@/types/schema/artist.type';

defineProps<{
  open: boolean
  members: ArtistWithAliases[]
}>()

const emit = defineEmits<{
  close: []
}>()

const router = useRouter()

const selectedMember = shallowRef<ArtistWithAliases | null>(null)

const selectedStageName = shallowRef<string | null>(null)

function selectMember(member: ArtistWithAliases) {
  selectedMember.value = member
  selectedStageName.value = member.aliases.find(a => a.kind === 'group_stage' && a.isPrimary)?.name ?? member.artist.realName
}

function back() {
  selectedMember.value = null
  selectedStageName.value = null
}

function handleClose() {
  selectedMember.value = null
  selectedStageName.value = null
  emit('close')
}

function navigateToSolo(artistId: ArtistId) {
  handleClose()
  router.push({ name: RouteName.ARTIST, params: { id: artistId } })
}
</script>

<template>
  <BottomSheet :open="open" @close="handleClose">
    <template #header>
      <div class="flex items-center gap-2">
        <button v-if="selectedMember" class="btn btn-ghost btn-xs btn-circle" @click="back">
          <ChevronLeftIcon class="w-4 h-4" />
        </button>
        <h2 class="text-base font-semibold">
          {{ selectedStageName ?? $t('screens.group.members.title') }}
        </h2>
      </div>
    </template>

    <Transition name="sheet-view" mode="out-in">
      <div v-if="!selectedMember" key="list" class="py-2">
        <MemberListItem v-for="member in members" :key="member.artist.id" :member="member"
          @click="selectMember(member)" />
      </div>

      <MemberDetail v-else :key="selectedMember.artist.id" :member="selectedMember"
        @navigate-to-solo="navigateToSolo" />
    </Transition>
  </BottomSheet>
</template>

<style scoped>
.sheet-view-enter-active,
.sheet-view-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}

.sheet-view-enter-from {
  opacity: 0;
  transform: translateX(16px);
}

.sheet-view-leave-to {
  opacity: 0;
  transform: translateX(-16px);
}
</style>
<script setup lang="ts">
import { useDatasetStore } from "@/stores/dataset.store";
import { storeToRefs } from "pinia";
import { computed } from "vue";
import { LucideArrowLeft } from "lucide-vue-next";
import { GroupId } from "@/types/group";

const { id } = defineProps<{
  id: GroupId; // passed by the router
  mode: "discover" | "owned"; // passed by the router
}>();

const { group, albums, lightsticks } = useGroupData()

function useGroupData() {
  const datasetStore = useDatasetStore();
  const { groups, albums, lightsticks } = storeToRefs(datasetStore);
  const group = computed(() => groups.value.find((group) => group.id === id));
  const groupAlbums = computed(() =>
    albums.value.filter((album) => album.groupId === id),
  );
  const groupLightstick = computed(() =>
    lightsticks.value.filter((lightstick) => lightstick.groupId === id),
  );

  return {
    group,
    albums: groupAlbums,
    lightsticks: groupLightstick,
  }
}
</script>

<template>
  <div class="screen--group">
    <header class="navbar gap-2 bg-base-100 shadow-sm sticky top-0 z-10">
      <button class="btn btn-gost btn-circle" @click="$router.back">
        <LucideArrowLeft />
      </button>
      <div class="flex-1">
        <h1 class="text-left">{{ group?.name }}</h1>
      </div>
    </header>

    <div>
      <ul>
        <li v-for="album in albums">
          {{ album.name }}
        </li>
      </ul>
      <ul>
        <li v-for="lightstick in lightsticks">
          {{ lightstick.name }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useCollectionStore } from "@/stores/collection.store";
import { useDatasetStore } from "@/stores/dataset.store";
import { type Collectible, CollectibleKind } from "@/types/dataset";
import type { Group } from "@/types/schema/group.type";
import { RouteName } from "@/types/routes";
import { storeToRefs } from "pinia";
import { computed } from "vue";

const { id } = defineProps<Group>();

const {
  totalItems,
  totalOwnedItems,
} = useGroupData()

function useGroupData() {
  const datasetStore = useDatasetStore();
  const { albums, lightsticks } = storeToRefs(datasetStore);

  const groupAlbum = computed(
    () => albums.value.filter((album) => album.groupId === id) ?? [],
  );
  const albumIds = computed(() => groupAlbum.value.map((album) => album.id));

  const groupLightstick = computed(
    () => lightsticks.value.filter((lightstick) => lightstick.groupId === id) ?? [],
  );
  const lightstickIds = computed(() =>
    groupLightstick.value.map((album) => album.id),
  );

  const totalItems = computed(
    () => groupAlbum.value.length + groupLightstick.value.length,
  );

  const collectionStore = useCollectionStore();
  const { collection } = storeToRefs(collectionStore);
  const ownedAlbums = computed(
    () =>
      collection.value
        ?.filter((item: Collectible) => item.kind === CollectibleKind.ALBUM)
        .filter((album) => albumIds.value.includes(album.id)) ?? [],
  );
  const ownedLightsticks = computed(
    () =>
      collection.value
        ?.filter(
          (item: Collectible) => item.kind === CollectibleKind.LIGHTSTICK,
        )
        .filter((lightstick) =>
          lightstickIds.value.includes(lightstick.id),
        ) ?? [],
  );
  const totalOwnedItems = computed(
    () => ownedAlbums.value.length + ownedLightsticks.value.length,
  );

  return {
    totalItems,
    totalOwnedItems,
  }
}
</script>

<template>
  <RouterLink :to="{
    name: RouteName.GROUP,
    params: {
      id: id,
      mode: 'discover',
    },
  }" class="card bg-base-200 active:scale-95 transition-transform duration-150" :id="`group-${id}`">
    <figure class="aspect-square bg-base-300">
      <div class="text-3xl opacity-40">♫</div>
    </figure>

    <div class="p-3 flex">
      <div class="grow">
        <p class="font-medium text-sm truncate">{{ name }}</p>
        <p class="text-xs opacity-60 truncate">{{ agency }}</p>
      </div>
      <div class="flex flex-col-reverse">
        <p class="text-xs opacity-60 truncate">
          {{ totalOwnedItems }} / {{ totalItems }}
        </p>
      </div>
    </div>
  </RouterLink>
</template>

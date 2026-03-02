<script setup lang="ts">
import GroupCard from "@/components/shared/GroupCard.vue";
import { useDatasetStore } from "@/stores/dataset.store";
import { info, warn } from "@tauri-apps/plugin-log";
import { storeToRefs } from "pinia";
import { onMounted } from "vue";

const datasetStore = useDatasetStore();
const { dataset, groups } = storeToRefs(datasetStore);

onMounted(async () => {
  await info("Home screen mounted");

  if (!dataset.value) {
    await warn("Dataset is not loaded, fetching dataset...");
    await datasetStore.sync();
  }
});
</script>

<template>
  <div class="screen--home flex flex-col h-full bg-base-100">
    <!-- Header -->
    <div class="px-4 pt-6 pb-4">
      <h1 class="text-2xl font-bold">My Collection</h1>
    </div>

    <!-- Sticky search + filters -->
    <div class="sticky top-0 z-10 bg-base-100 px-4 pb-4 space-y-3">
      <!-- Search -->
      <input
        type="text"
        placeholder="Search..."
        class="input input-bordered w-full"
      />

      <!-- Horizontal filter pills -->
      <form class="filter">
        <input class="btn btn-sm btn-square" type="reset" value="×" />
        <input
          class="btn btn-sm"
          type="radio"
          name="item-filter"
          aria-label="Groups"
        />
        <input
          class="btn btn-sm"
          type="radio"
          name="item-filter"
          aria-label="Albums"
        />
        <input
          class="btn btn-sm"
          type="radio"
          name="item-filter"
          aria-label="Lightsticks"
        />
      </form>
    </div>

    <!-- Scrollable content -->
    <div class="flex-1 overflow-y-auto px-4 pb-24">
      <div class="group-list grid grid-cols-2 gap-4">
        <GroupCard v-for="group in groups" :key="group.id" v-bind="group" />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.filter:has(input:checked) > input:checked {
  @apply btn-primary;
}

.group-list {
  grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));

  @media screen and (width >= 900px) {
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  }
}
</style>

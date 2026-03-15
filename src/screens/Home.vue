<script setup lang="ts">
import { useDatasetStore } from "@/stores/dataset.store";
import { storeToRefs } from "pinia";

const datasetStore = useDatasetStore();
const { syncing } = storeToRefs(datasetStore);
</script>

<template>
  <div class="screen--home grow relative pb-10">

    <!-- Header -->
    <div class="sticky top-0 z-10 bg-base-100/80 backdrop-blur-md border-b border-base-300">
      <div class="flex items-center gap-3 px-4 py-4 max-w-lg mx-auto">
        <h1 class="text-xl font-bold tracking-tight">{{ $t('screens.home.title') }}</h1>
      </div>
    </div>

    <div class="max-w-lg mx-auto px-4 pt-6 space-y-5">
      <!-- Loading -->
      <div v-if="syncing" class="flex justify-center items-center h-40">
        <span class="loading loading-spinner loading-lg"></span>
      </div>

      <div v-else>
        {{ 'dataset synced' }}
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.filter:has(input:checked)>input:checked {
  @apply btn-primary;
}

.group-list {
  grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));

  @media screen and (width >=900px) {
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  }
}
</style>

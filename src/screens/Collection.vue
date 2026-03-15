<script setup lang="ts">
import { useCollectionStore } from '@/stores/collection.store';
import { storeToRefs } from 'pinia';
import { onMounted } from 'vue';
import { PlusIcon } from 'lucide-vue-next'

const collectionStore = useCollectionStore()
const { loading } = storeToRefs(collectionStore)

onMounted(async () => {
  // await collectionStore.load()
})
</script>

<template>
  <div class="screen--collection grow relative pb-10">

    <div class="fab absolute">
      <RouterLink to="/collection/add" tabindex="0" role="button" class="btn btn-lg btn-circle btn-primary">
        <PlusIcon />
      </RouterLink>
    </div>

    <!-- Header -->
    <div class="sticky top-0 z-10 bg-base-100/80 backdrop-blur-md border-b border-base-300">
      <div class="flex items-center gap-3 px-4 py-4 max-w-lg mx-auto">
        <h1 class="text-xl font-bold tracking-tight">{{ $t('screens.collection.title') }}</h1>
      </div>
    </div>

    <div class="max-w-lg mx-auto px-4 pt-6 space-y-5">
      <!-- Loading -->
      <div v-if="loading" class="flex justify-center items-center h-40">
        <span class="loading loading-spinner loading-lg"></span>
      </div>

      <div v-else>
        {{ 'collection synced' }}
      </div>
    </div>
  </div>
</template>

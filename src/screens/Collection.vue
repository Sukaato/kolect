<script setup lang="ts">
import { useCollectionStore } from '@/stores/collection.store';
import { LucideScanBarcode } from 'lucide-vue-next';
import { storeToRefs } from 'pinia';
import { onMounted } from 'vue';

const collectionStore = useCollectionStore()
const { collection, loading, error } = storeToRefs(collectionStore)

onMounted(async () => {
  await collectionStore.load()
})
</script>

<template>
  <div class="screen--collection">
    <header class="navbar bg-base-100 shadow-sm">
      <div class="flex-1">
        <h1 class="text-left">My Collection</h1>
      </div>
      <div class="flex-none">
        <button class="btn btn-primary btn-square">
          <LucideScanBarcode />
        </button>
      </div>
    </header>
    <div>
      <button @click="collectionStore.load()">Refresh</button>
      <Transition>
        <p v-if="loading">Loading...</p>
      </Transition>
      <Transition>
        <p v-if="error" class="text-error">Error: {{ error }}</p>
      </Transition>
      <TransitionGroup tag="ul">
        Collection
        <li v-for="value in collection" :key="value.id">
          {{ value.productId }} ({{ value.productType }})
        </li>
      </TransitionGroup>
    </div>
  </div>
</template>
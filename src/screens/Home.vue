<script setup lang="ts">
import CollectionGrid from '@/components/shared/CollectionGrid.vue'
import { useDatasetStore } from '@/stores/dataset.store'
import { PlusIcon } from 'lucide-vue-next'
import { onMounted } from 'vue'

const datasetStore = useDatasetStore()

onMounted(async () => {
  await datasetStore.fetch()
})
</script>

<template>
  <CollectionGrid :store="datasetStore" agencies-command="dataset_get_agencies" screen-class="screen--home">
    <template #title>{{ $t('screens.home.title') }}</template>

    <template #fab>
      <div class="absolute bottom-6 right-4 z-20">
        <RouterLink to="/collection/add" tabindex="0" role="button" class="btn btn-lg btn-circle btn-primary shadow-lg">
          <PlusIcon />
        </RouterLink>
      </div>
    </template>
  </CollectionGrid>
</template>
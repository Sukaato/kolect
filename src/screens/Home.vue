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
    <template #title>{{ $t('screen.home.title') }}</template>

    <template #fab>
      <div class="absolute bottom-3 right-4 z-20">
        <RouterLink to="/collection/add" tabindex="0" role="button" class="btn btn-lg btn-circle btn-primary shadow-lg">
          <PlusIcon />
        </RouterLink>
      </div>
    </template>

    <template #empty="{ hasActiveFilters }">
      <p class="text-sm">
        {{ $t(hasActiveFilters ? 'screen.home.list.empty_filtered' : 'screen.home.list.empty') }}
      </p>
    </template>

    <template #end-of-list>
      {{ $t('screen.home.list.end') }}
    </template>
  </CollectionGrid>
</template>
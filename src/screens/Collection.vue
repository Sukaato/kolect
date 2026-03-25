<script setup lang="ts">
import CollectionGrid from '@/components/shared/CollectionGrid.vue'
import { useCollectionStore } from '@/stores/collection.store'
import { PlusIcon } from '@lucide/vue'
import { onMounted } from 'vue'

const collectionStore = useCollectionStore()

onMounted(async () => {
  await collectionStore.fetch()
})
</script>

<template>
  <CollectionGrid :store="collectionStore" agencies-command="collection_get_agencies" screen-class="screen--collection">
    <template #title>{{ $t('screen.collection.title') }}</template>

    <template #fab>
      <div class="absolute bottom-6 right-4 z-20">
        <RouterLink to="/collection/add" tabindex="0" role="button" class="btn btn-lg btn-circle btn-primary shadow-lg">
          <PlusIcon />
        </RouterLink>
      </div>
    </template>

    <template #empty="{ hasActiveFilters }">
      <p class="text-sm">
        {{ $t(hasActiveFilters ? 'screen.collection.list.empty_filtered' : 'screen.collection.list.empty') }}
      </p>
    </template>

    <template #end-of-list>
      {{ $t('screen.collection.list.end') }}
    </template>
  </CollectionGrid>
</template>
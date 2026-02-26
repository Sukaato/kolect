<script setup lang="ts">
import { useDatasetStore } from '@/stores/dataset.store'
import { storeToRefs } from 'pinia'

const datasetStore = useDatasetStore()
const { syncing } = storeToRefs(datasetStore)
</script>

<template>
  <div class="card bg-base-200 shadow-sm">
    <div class="card-body">
      <h2 class="card-title">{{ $t('screens.setting.sections.data.dataset.title') }}</h2>
      <p class="text-sm text-base-content/60">{{ $t('screens.setting.sections.data.dataset.description') }}</p>
      <div class="card-actions justify-end">
        <button class="btn btn-soft btn-block flex gap-2" :disabled="syncing" @click="datasetStore.sync()">
          <div class="inline-block relative w-52 h-full overflow-y-hidden">
            <Transition name="slide-up">
              <span v-if="syncing" class="absolute inset-0 text-center">
                {{ $t('screens.setting.sections.data.dataset.actions.sync.syncing') }}
              </span>
              <span v-else class="absolute inset-0 text-center">
                {{ $t('screens.setting.sections.data.dataset.actions.sync.text') }}
              </span>
            </Transition>
          </div>
        </button>
      </div>
    </div>
  </div>
</template>

<style>
.slide-up-enter-active,
.slide-up-leave-active {
  transition: all 0.25s ease-out;
}

.slide-up-enter-from {
  opacity: 0;
  transform: translateY(30px);
}

.slide-up-leave-to {
  opacity: 0;
  transform: translateY(-30px);
}
</style>
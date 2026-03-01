<script setup lang="ts">
import { useDatasetStore } from '@/stores/dataset.store'
import { useSettingStore } from '@/stores/setting.store'
import { storeToRefs } from 'pinia'

const datasetStore = useDatasetStore()
const { syncing, fetchedAt } = storeToRefs(datasetStore)

const settingStore = useSettingStore()
const { lang } = storeToRefs(settingStore)
</script>

<template>
  <div class="card bg-base-200 shadow-sm">
    <div class="card-body">
      <h2 class="card-title">{{ $t('screens.setting.sections.data.dataset.title') }}</h2>
      <p class="text-sm text-base-content/60">{{ $t('screens.setting.sections.data.dataset.description') }}</p>
      <p class="text-sm text-base-content/60">last time fetched: {{ fetchedAt?.toLocaleDateString(lang) }} at {{ fetchedAt?.toLocaleTimeString(lang) }}</p>
      <div class="card-actions justify-end">
        <button class="btn btn-soft btn-block btn-primary flex gap-2" :disabled="syncing" @click="datasetStore.sync()">
          <div class="inline-flex justify-center items-center relative w-full h-full overflow-y-hidden">
            <Transition name="slide-up">
              <span v-if="syncing" class="absolute">
                {{ $t('screens.setting.sections.data.dataset.actions.sync.syncing') }}
              </span>
              <span v-else class="absolute">
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
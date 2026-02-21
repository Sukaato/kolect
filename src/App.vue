<script setup lang="ts">
import { computed, shallowRef, watch } from 'vue'
import { useRoute } from 'vue-router'
import Dock from './components/global/dock.vue'
import Toast from './components/global/toast.vue'
import { useDataset } from './composables/shared/use-dataset'
import { TransitionName } from '@/types/transitions'

const route = useRoute()
const { error } = useDataset()
const displayError = shallowRef<string | null>(null)

console.log('[App] Component mounted')

watch(() => route.path, (newPath) => {
  console.log('[App] Route changed to:', newPath)
})

const showDock = computed(() => route.path !== '/')

const transitionName = computed(() => {
  return route.path === '/' ? TransitionName.STARTUP : TransitionName.PAGE
})

watch(error, (newError) => {
  if (newError) {
    displayError.value = newError
    // Auto-hide after 5 seconds
    setTimeout(() => {
      displayError.value = null
    }, 5000)
  }
})
</script>

<template>
  <main class="app-root bg-base-300 h-dvh" data-theme="dark">
    <RouterView v-slot="{ Component }">
      <Transition :name="transitionName" mode="out-in">
        <component :is="Component" />
      </Transition>
    </RouterView>
  </main>
  <Transition :name="TransitionName.DOCK">
    <Dock v-if="showDock" />
  </Transition>
  <Toast :message="displayError" type="error" />
</template>

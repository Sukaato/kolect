<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import Dock from './components/global/dock.vue'
import Toast from './components/global/toast.vue'
import { useDataset } from './composables/shared/use-dataset'

const { sync, error } = useDataset()
const displayError = ref<string | null>(null)

onMounted(async () => {
  await sync()
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
    <router-view />
  </main>
  <Dock />
  <Toast :message="displayError" type="error" />
</template>

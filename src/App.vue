<script setup lang="ts">
import { onMounted } from 'vue'
import Dock from './components/global/dock.vue'
import { useDataset } from './composables/shared/use-dataset'

const { sync } = useDataset()

onMounted(async () => {
  try {
    await sync()
  } catch (error) {
    console.error('Failed to sync dataset:', error)
  }
})
</script>

<template>
  <main class="app-root bg-base-300 h-dvh" data-theme="dark">
    <router-view />
  </main>
  <Dock />
</template>

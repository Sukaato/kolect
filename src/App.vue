<script setup lang="ts">
import Dock from './components/global/dock.vue'
</script>

<template>
  <main class="app-root container bg-base-300 h-dvh" data-theme="dark">
    <router-view />
  </main>
  <Dock />
</template>

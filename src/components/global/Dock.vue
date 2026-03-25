<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { LucideHome, LucideLibrary, LucideUser } from '@lucide/vue';
</script>

<template>
  <div class="dock dock-sm sticky">
    <RouterLink to="/home">
      <LucideHome />
    </RouterLink>

    <RouterLink to="/collection">
      <LucideLibrary />
    </RouterLink>

    <RouterLink to="/setting">
      <LucideUser />
    </RouterLink>
  </div>
</template>
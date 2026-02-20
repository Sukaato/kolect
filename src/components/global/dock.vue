<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { LucideHome, LucideLibrary, LucideUser } from 'lucide-vue-next';
</script>

<template>
  <div class="dock dock-sm">
    <RouterLink to="/">
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
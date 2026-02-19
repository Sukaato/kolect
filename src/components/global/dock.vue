<script setup lang="ts">
import { LucideHome, LucideLibrary, LucideUser } from 'lucide-vue-next';


</script>

<template>
  <div class="dock dock-sm">
    <RouterLink to="/" active-class="dock-active">
      <LucideHome />
    </RouterLink>

    <RouterLink to="/collection" active-class="dock-active">
      <LucideLibrary />
    </RouterLink>

    <RouterLink to="/setting" active-class="dock-active">
      <LucideUser />
    </RouterLink>
  </div>
</template>
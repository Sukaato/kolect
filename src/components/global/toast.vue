<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  message: string | null
  type?: 'error' | 'success' | 'info' | 'warning'
}

const props = withDefaults(defineProps<Props>(), {
  type: 'error',
})

const isVisible = computed(() => props.message !== null && props.message !== '')

const alertClass = computed(() => {
  const base = 'alert'
  const typeClasses = {
    error: 'alert-error',
    success: 'alert-success',
    info: 'alert-info',
    warning: 'alert-warning',
  }
  return `${base} ${typeClasses[props.type] || 'alert-error'}`
})
</script>

<template>
  <Transition name="toast">
    <div v-if="isVisible" :class="alertClass" class="fixed bottom-24 left-4 right-4 max-w-md shadow-lg">
      <span>{{ message }}</span>
    </div>
  </Transition>
</template>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  transform: translateY(100%);
  opacity: 0;
}

.toast-leave-to {
  transform: translateY(100%);
  opacity: 0;
}
</style>

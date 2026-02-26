<script setup lang="ts">
import { useToastStore, type Toast } from '@/stores/toast.store';
import { computed, shallowRef } from 'vue'

const { id, type = 'error' } = defineProps<Toast>()

const toastStore = useToastStore()

const startX = shallowRef(0)
const currentX = shallowRef(0)
const dragging = shallowRef(false)

const threshold = 100 // px avant dismiss

const alertClass = computed(() => {
  const base = 'alert alert-soft'
  const typeClasses = {
    error: 'alert-error',
    success: 'alert-success',
    info: 'alert-info',
    warning: 'alert-warning',
  }
  return `${base} ${typeClasses[type] || 'alert-error'}`
})

const style = computed(() => {
  if (!dragging.value) return {}
  return {
    transform: `translateX(${currentX.value}px)`,
    opacity: 1 - Math.abs(currentX.value) / 300,
  }
})

function onPointerDown(e: PointerEvent) {
  // toastStore.pauseTimer()
  dragging.value = true
  startX.value = e.clientX
}

function onPointerMove(e: PointerEvent) {
  if (!dragging.value) return
  currentX.value = e.clientX - startX.value
}

function onPointerUp() {
  if (!dragging.value) return

  if (Math.abs(currentX.value) > threshold) {
    toastStore.remove(id)
  }

  dragging.value = false
  currentX.value = 0
}
</script>

<template>
  <div :class="alertClass" class="shadow-lg flex flex-col select-none" :style="style" @pointerdown="onPointerDown"
    @pointermove="onPointerMove" @pointerup="onPointerUp" @pointercancel="onPointerUp">
    <span v-if="options?.title" class="font-semibold">
      {{ options.title }}
    </span>
    <span class="text-sm">
      {{ message }}
    </span>
  </div>
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

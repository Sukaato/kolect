<script setup lang="ts">
import { useScrollLock, onKeyStroke } from '@vueuse/core'
import { watch } from 'vue'

const props = withDefaults(defineProps<{
  open: boolean
  title?: string
}>(), {
  title: undefined,
})

const emit = defineEmits<{
  close: []
}>()

// const bodyRef = useTemplateRef('sheet-body')
const isLocked = useScrollLock(document.body)

watch(() => props.open, (val) => {
  isLocked.value = val
})

onKeyStroke('Escape', () => {
  if (props.open) emit('close')
})
</script>

<template>
  <Teleport to="body">
    <Transition name="sheet-backdrop">
      <div v-if="open" class="fixed inset-0 z-40 bg-black/40" @click="emit('close')" />
    </Transition>

    <Transition name="sheet">
      <div v-if="open" ref="sheet-body"
        class="fixed bottom-0 left-0 right-0 z-50 max-w-lg mx-auto bg-base-100 rounded-t-2xl shadow-xl flex flex-col max-h-[85dvh]"
        role="dialog" :aria-label="title">
        <!-- Handle -->
        <div class="flex justify-center pt-3 pb-1 shrink-0">
          <div class="w-10 h-1 rounded-full bg-base-300" />
        </div>

        <!-- Header -->
        <div v-if="title || $slots.header" class="px-5 py-3 border-b border-base-200 shrink-0">
          <slot name="header">
            <h2 class="text-base font-semibold">{{ title }}</h2>
          </slot>
        </div>

        <!-- Contenu scrollable -->
        <div class="overflow-y-auto flex-1 overscroll-contain">
          <slot />
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.sheet-backdrop-enter-active,
.sheet-backdrop-leave-active {
  transition: opacity 0.25s ease;
}

.sheet-backdrop-enter-from,
.sheet-backdrop-leave-to {
  opacity: 0;
}

.sheet-enter-active,
.sheet-leave-active {
  transition: transform 0.3s cubic-bezier(0.32, 0.72, 0, 1);
}

.sheet-enter-from,
.sheet-leave-to {
  transform: translateY(100%);
}
</style>
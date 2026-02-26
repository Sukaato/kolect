<script setup lang="ts">
import { TransitionName } from '@/types/transitions'
import { storeToRefs } from 'pinia'
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import Dock from './components/global/dock.vue'
import Toast from './components/global/toast.vue'
import { useToastStore } from './stores/toast.store'

const route = useRoute()

const toastStore = useToastStore()
const { toasts } = storeToRefs(toastStore)

const showDock = computed(() => route.path !== '/')

const transitionName = computed(() => {
  return route.path === '/' ? TransitionName.STARTUP : TransitionName.PAGE
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

  <div class="fixed bottom-24 left-4 right-4 flex justify-center pointer-events-none">
    <TransitionGroup name="toast" tag="div" class="stack w-full max-w-md pointer-events-auto">
      <Toast v-for="toast in toasts" :key="toast.id" v-bind="toast" />
    </TransitionGroup>
  </div>
</template>

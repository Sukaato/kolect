<script setup lang="ts">
import { TransitionName } from '@/types/transitions'
import { usePreferredColorScheme } from '@vueuse/core'
import { storeToRefs } from 'pinia'
import { computed, onErrorCaptured, watch } from 'vue'
import { useRoute } from 'vue-router'
import Dock from './components/global/Dock.vue'
import PossessionModal from './components/global/PossessionModal.vue'
import Toast from './components/global/Toast.vue'
import { useSettingStore } from './stores/setting.store'
import { useToastStore } from './stores/toast.store'

const route = useRoute()
const showDock = computed(() => route.path !== '/')
const transitionName = computed(() => {
  return route.path === '/' ? TransitionName.STARTUP : TransitionName.PAGE
})

const systemTheme = usePreferredColorScheme()

const settingStore = useSettingStore()
const { theme } = storeToRefs(settingStore)
watch([theme, systemTheme], ([theme]) => {
  document.body.dataset.theme = theme === 'system' ? systemTheme.value : theme
})

const toastStore = useToastStore()
const { toasts } = storeToRefs(toastStore)

onErrorCaptured(error => {
  toastStore.show(error.message, 'error', {
    title: 'Unexpected Error'
  })
})
</script>

<template>
  <div class="flex flex-col h-dvh bg-base-300">
    <main class="app-root grow overflow-y-auto overflow-x-hidden flex flex-col">
      <RouterView v-slot="{ Component }">
        <Transition :name="transitionName" mode="out-in">
          <component :is="Component" />
        </Transition>
      </RouterView>
    </main>

    <Transition :name="TransitionName.DOCK">
      <Dock v-if="showDock" />
    </Transition>

    <div class="toast-container fixed bottom-24 left-4 right-4 flex justify-center pointer-events-none">
      <TransitionGroup name="toast" tag="div" class="toast-wrapper stack w-full max-w-md">
        <Toast v-for="toast in toasts" :key="toast.id" v-bind="toast" class="pointer-events-all" />
      </TransitionGroup>
    </div>

    <PossessionModal />
  </div>
</template>

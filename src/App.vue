<script setup lang="ts">
import { TransitionName } from '@/types/transitions'
import { storeToRefs } from 'pinia'
import { computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import Dock from './components/global/dock.vue'
import Toast from './components/global/toast.vue'
import { useSettingStore } from './stores/setting.store'
import { useToastStore } from './stores/toast.store'

const route = useRoute()
const showDock = computed(() => route.path !== '/')
const transitionName = computed(() => {
  return route.path === '/' ? TransitionName.STARTUP : TransitionName.PAGE
})

const settingStore = useSettingStore()
const { theme, lang } = storeToRefs(settingStore)

const toastStore = useToastStore()
const { toasts } = storeToRefs(toastStore)

const i18n = useI18n()

onMounted(async () => {
  await settingStore.init()

  i18n.locale.value = lang.value
})
</script>

<template>
  <div :data-theme="theme">
    <main class="app-root bg-base-300 h-dvh overflow-y-auto">
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
  </div>
</template>

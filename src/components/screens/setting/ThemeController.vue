<script setup lang="ts">
import { useThemeStore } from '@/stores/theme.store';
import { storeToRefs } from 'pinia';

const themeStore = useThemeStore()
const { theme } = storeToRefs(themeStore)
</script>

<template>
  <div class="dropdown dropdown-end">
    <div tabindex="0" role="button" class="btn m-1 capitalize" aria-haspopup="true"
      :aria-label="`Current theme: ${$t(`screens.setting.sections.appearence.theme.values.${theme}`)}`">
      {{ $t(`screens.setting.sections.appearence.theme.values.${theme}`) }}
      <svg width="12px" height="12px" class="inline-block h-2 w-2 fill-current opacity-60"
        xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2048 2048">
        <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
      </svg>
    </div>

    <ul tabindex="-1" class="dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl">
      <li>
        <input type="radio" name="theme-dropdown"
          class="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"
          :aria-label="$t('screens.setting.sections.appearence.theme.values.system')" value="system"
          @click="themeStore.setTheme('system')" />
      </li>
      <li>
        <input type="radio" name="theme-dropdown"
          class="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"
          :aria-label="$t('screens.setting.sections.appearence.theme.values.light')" value="light"
          @click="themeStore.setTheme('light')" />
      </li>
      <li>
        <input type="radio" name="theme-dropdown"
          class="theme-controller w-full btn btn-sm btn-block btn-ghost justify-start"
          :aria-label="$t('screens.setting.sections.appearence.theme.values.dark')" value="dark"
          @click="themeStore.setTheme('dark')" />
      </li>
    </ul>
  </div>
</template>
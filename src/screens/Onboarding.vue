<script setup lang="ts">
import { useStepper } from '@vueuse/core'
import { ChevronLeft } from 'lucide-vue-next'
import { useSettingStore } from '@/stores/setting.store'
import { storeToRefs } from 'pinia'
import OnboardingWelcome from '@/components/screens/onboarding/OnboardingWelcome.vue'
import OnboardingLanguage from '@/components/screens/onboarding/OnboardingLanguage.vue'
import OnboardingTheme from '@/components/screens/onboarding/OnboardingTheme.vue'
import OnboardingPreferences from '@/components/screens/onboarding/OnboardingPreferences.vue'
import OnboardingDone from '@/components/screens/onboarding/OnboardingDone.vue'
import { useRouter } from 'vue-router'
import { RouteName } from '@/types/routes'

const router = useRouter()

const settingStore = useSettingStore()
const { onboardingDone } = storeToRefs(settingStore)

const stepper = useStepper(['welcome', 'language', 'theme', 'preferences', 'done'])

async function finish(): Promise<void> {
  onboardingDone.value = true
  router.replace({ name: RouteName.HOME })
}
</script>

<template>
  <div class="screen screen--onboarding flex flex-col h-dvh bg-base-100 overflow-hidden">

    <!-- Top bar — back + skip -->
    <div v-if="!stepper.isLast.value" class="flex items-center justify-between px-6 pt-4">
      <button
        v-if="!stepper.isFirst.value"
        class="btn btn-ghost btn-sm"
        @click="stepper.goToPrevious"
      >
        <ChevronLeft :size="18" />
      </button>
      <div v-else></div>

      <button class="btn btn-ghost btn-sm text-base-content/40" @click="finish">
        {{ $t('screen.onboarding.actions.skip') }}
      </button>
    </div>

    <!-- Slides -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <OnboardingWelcome v-if="stepper.isCurrent('welcome')" @next="stepper.goToNext" />
      <OnboardingLanguage v-if="stepper.isCurrent('language')" @next="stepper.goToNext" />
      <OnboardingTheme v-if="stepper.isCurrent('theme')" @next="stepper.goToNext" />
      <OnboardingPreferences v-if="stepper.isCurrent('preferences')" @next="stepper.goToNext" />
      <OnboardingDone v-if="stepper.isCurrent('done')" @finish="finish" />
    </div>

    <!-- Pagination dots -->
    <div class="flex justify-center gap-2 pb-6 pt-2">
      <span
        v-for="step in stepper.steps.value"
        :key="step"
        class="block rounded-full transition-all duration-300"
        :class="stepper.isCurrent(step) ? 'w-6 h-2 bg-primary' : 'w-2 h-2 bg-base-content/20'"
      ></span>
    </div>

  </div>
</template>
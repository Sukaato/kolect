import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ToastType = 'error' | 'success' | 'info' | 'warning'

export interface Toast {
  id: number
  message: string
  type: ToastType
  options?: ToastOptions
}

export interface ToastOptions {
  title?: string
  duration?: number
  persistent?: boolean
}

let id = 0

export const useToastStore = defineStore('toast', () => {
  const toasts = ref<Toast[]>([])
  const activeTimer = ref<ReturnType<typeof setTimeout> | null>(null)

  function show(message: string, type: ToastType = 'error', options: ToastOptions = {}) {
    options.duration ??= 5000
    const toast: Toast = {
      id: id++,
      message,
      type,
      options,
    }

    toasts.value.push(toast)

    if (toasts.value.length === 1) {
      startTimer(toast)
    }
  }

  function startTimer(toast: Toast) {
    const duration = toast.options?.duration ?? 5000
    const persistent = toast.options?.persistent ?? false

    if (persistent || duration <= 0) return

    activeTimer.value = setTimeout(() => {
      remove(toast.id)
    }, duration)
  }

  function pauseTimer() {
    if (activeTimer.value) {
      clearTimeout(activeTimer.value)
      activeTimer.value = null
    }
  }

  function remove(toastId: number) {
    if (activeTimer.value) {
      clearTimeout(activeTimer.value)
      activeTimer.value = null
    }

    const index = toasts.value.findIndex(t => t.id === toastId)
    if (index === -1) return

    toasts.value.splice(index, 1)

    const nextToast = toasts.value[0]
    if (nextToast) {
      startTimer(nextToast)
    }
  }

  function clear() {
    if (activeTimer.value) {
      clearTimeout(activeTimer.value)
      activeTimer.value = null
    }

    toasts.value = []
  }

  return {
    toasts,
    show,
    remove,
    clear,
    pauseTimer,
  }
})

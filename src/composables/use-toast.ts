import { type ToastOptions, useToastStore } from '@/stores/toast.store'

export function useToast() {
  const toastStore = useToastStore()

  return {
    success: (msg: string, options?: ToastOptions) => toastStore.show(msg, 'success', options),
    error: (msg: string, options?: ToastOptions) => toastStore.show(msg, 'error', options),
    info: (msg: string, options?: ToastOptions) => toastStore.show(msg, 'info', options),
    warning: (msg: string, options?: ToastOptions) => toastStore.show(msg, 'warning', options),
  }
}

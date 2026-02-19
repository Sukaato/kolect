import { invoke } from '@tauri-apps/api/core'
import { shallowRef } from 'vue'

export function useGreet() {
  const message = shallowRef('')

  async function greet(name: string) {
    message.value = await invoke('greet', { name })
  }

  return {
    message,
    greet
  }
}
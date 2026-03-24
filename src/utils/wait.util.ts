import { type ShallowRef, watch } from 'vue'

/**
 * Wait for a specified duration in milliseconds
 * @param ms - Duration in milliseconds
 */
export function wait(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

export function waitUntilReady(readyRef: ShallowRef<boolean>) {
  return new Promise<void>(resolve => {
    if (readyRef.value) return resolve()

    const stop = watch(readyRef, v => {
      if (v) {
        stop()
        resolve()
      }
    })
  })
}

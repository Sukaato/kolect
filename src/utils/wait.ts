/**
 * Wait for a specified duration in milliseconds
 * @param ms - Duration in milliseconds
 */
export function wait(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

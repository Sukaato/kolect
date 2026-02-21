import { invoke } from '@tauri-apps/api/core'

type LogLevel = 'debug' | 'info' | 'warn' | 'error'

export function useLogger(context?: string) {
  async function log(level: LogLevel, message: string, body?: unknown) {
    const fullMessage = context ? `[${context}] ${message}` : message
    try {
      const bodyValue = body ? (typeof body === 'string' ? body : JSON.stringify(body)) : null
      await invoke('log', {
        level,
        message: fullMessage,
        body: bodyValue,
      })
    } catch (_e) {
      // Fallback to console if invoke fails
      const logFn = level === 'error' || level === 'warn' ? console[level] : console.log
      logFn(`[${level.toUpperCase()}] ${fullMessage}`, body)
    }
  }

  return {
    debug: (message: string, body?: unknown) => log('debug', message, body),
    info: (message: string, body?: unknown) => log('info', message, body),
    warn: (message: string, body?: unknown) => log('warn', message, body),
    error: (message: string, body?: unknown) => log('error', message, body),
  }
}

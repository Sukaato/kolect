import 'vue'

declare module 'vue' {
  interface SelectHTMLAttributes {
    onChange?: (event: Event & { target: HTMLSelectElement }) => void
  }
}
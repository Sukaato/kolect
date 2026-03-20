import 'vue'

declare module 'vue' {
  interface SelectHTMLAttributes {
    onChange?: (event: Event & { target: HTMLSelectElement }) => void
  }
  interface InputHTMLAttributes {
    onChange?: (event: Event & { target: HTMLInputElement }) => void
  }
}

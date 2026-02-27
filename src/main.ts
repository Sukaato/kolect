import { info } from '@tauri-apps/plugin-log'
import { createPinia } from 'pinia'
import { createApp } from 'vue'
import App from './App.vue'
import i18n from './i18n'
import router from './router'

import './styles/index.scss'
import './styles/tailwind.css'
import './styles/transitions.scss'

await info('Starting Kolect app...')

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(router)
app.use(i18n)

app.mount('#app')

await info('App mounted successfully')

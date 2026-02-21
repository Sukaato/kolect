import { createApp } from 'vue'
import App from './App.vue'
import i18n from './i18n'
import router from './router'

import './styles/tailwind.css'
import './styles/index.scss'
import './styles/transitions.scss'

console.log('[Main] Starting Kolect app...')

const app = createApp(App)

app.use(router)
app.use(i18n)

console.log('[Main] Plugins loaded')

app.mount('#app')

console.log('[Main] App mounted successfully')

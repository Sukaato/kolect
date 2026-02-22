import { createApp } from 'vue'
import App from './App.vue'
import { useLogger } from './composables/use-logger'
import i18n from './i18n'
import router from './router'

import './styles/tailwind.css'
import './styles/index.scss'
import './styles/transitions.scss'
import { createPinia } from 'pinia'

const { info } = useLogger('Main')

await info('Starting Kolect app...')

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(router)
app.use(i18n)

app.mount('#app')

await info('App mounted successfully')

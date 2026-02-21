import { createApp } from 'vue'
import App from './App.vue'
import { useLogger } from './composables/shared/use-logger'
import i18n from './i18n'
import router from './router'

import './styles/tailwind.css'
import './styles/index.scss'
import './styles/transitions.scss'

const { info } = useLogger('Main')

info('Starting Kolect app...')

const app = createApp(App)

app.use(router)
app.use(i18n)

info('Plugins loaded')

app.mount('#app')

info('App mounted successfully')

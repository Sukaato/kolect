import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import { useLogger } from '@/composables/use-logger'
import Home from '@/screens/Home.vue'
import Startup from '@/screens/Startup.vue'
import { RouteName } from '@/types/routes'

const routes: RouteRecordRaw[] = [
  { path: '/', name: RouteName.STARTUP, component: Startup },
  { path: '/home', name: RouteName.HOME, component: Home },
  {
    path: '/collection',
    name: RouteName.COLLECTION,
    component: () => import('@/screens/Collection.vue'),
  },
  { path: '/setting', name: RouteName.SETTING, component: () => import('@/screens/Setting.vue') },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  linkActiveClass: 'dock-active',
  routes,
})

const { debug } = useLogger('Router')

debug('Initializing routes')

router.beforeEach((to, from) => {
  debug(`Navigating from ${from.path} to ${to.path}`)
})

router.afterEach(to => {
  debug(`Navigation complete, now at ${to.path}`)
})

export default router

import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import Home from '@/screens/Home.vue'
import Startup from '@/screens/Startup.vue'
import { RouteName } from '@/types/routes'

const routes: RouteRecordRaw[] = [
  { path: '/', name: RouteName.Startup, component: Startup },
  { path: '/home', name: RouteName.Home, component: Home },
  {
    path: '/collection',
    name: RouteName.Collection,
    component: () => import('@/screens/Collection.vue'),
  },
  { path: '/setting', name: RouteName.Setting, component: () => import('@/screens/Setting.vue') },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  linkActiveClass: 'dock-active',
  routes,
})

console.log('[Router] Initializing routes')

router.beforeEach((to, from) => {
  console.log(`[Router] Navigating from ${from.path} to ${to.path}`)
})

router.afterEach(to => {
  console.log(`[Router] Navigation complete, now at ${to.path}`)
})

export default router

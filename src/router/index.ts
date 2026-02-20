import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import Startup from '@/screens/Startup.vue'

const routes: RouteRecordRaw[] = [
  { path: '/', name: 'Startup', component: Startup },
  { path: '/home', name: 'Home', component: () => import('@/screens/Home.vue') },
  { path: '/collection', name: 'Collection', component: () => import('@/screens/Collection.vue') },
  { path: '/setting', name: 'Setting', component: () => import('@/screens/Setting.vue') },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  linkActiveClass: 'dock-active',
  routes,
})

export default router

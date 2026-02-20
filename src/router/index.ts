import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import Home from '@/screens/Home.vue'

const routes: RouteRecordRaw[] = [
  { path: '/', name: 'Home', component: Home },
  { path: '/collection', name: 'Collection', component: () => import('@/screens/Collection.vue') },
  { path: '/setting', name: 'Setting', component: () => import('@/screens/Setting.vue') },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  linkActiveClass: 'dock-active',
  routes,
})

export default router

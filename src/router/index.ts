import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  { path: '/', name: 'Home', component: () => import('@/screens/Home.vue') },
  { path: '/collection', name: 'Collection', component: () => import('@/screens/Collection.vue') },
  { path: '/setting', name: 'Setting', component: () => import('@/screens/Setting.vue') },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router

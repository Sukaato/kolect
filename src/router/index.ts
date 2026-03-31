import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import Startup from '@/screens/Startup.vue'
import { RouteName } from '@/types/routes'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: RouteName.STARTUP,
    component: Startup,
  },
  {
    path: '/onboarding',
    name: RouteName.ONBOARDING,
    component: () => import('@/screens/Onboarding.vue'),
  },
  {
    path: '/home',
    name: RouteName.HOME,
    component: () => import('@/screens/Home.vue'),
  },
  {
    path: '/collection',
    name: RouteName.COLLECTION,
    component: () => import('@/screens/Collection.vue'),
  },
  {
    path: '/setting',
    name: RouteName.SETTING,
    component: () => import('@/screens/Setting.vue'),
  },
  {
    path: '/groups/:id',
    children: [
      {
        path: '',
        name: RouteName.GROUP,
        component: () => import('@/screens/Group.vue'),
        props: true,
      },
      {
        path: 'albums/:albumId',
        name: RouteName.GROUP_ALBUM,
        component: () => import('@/screens/Album.vue'),
        props: true,
      },
    ],
  },
  {
    path: '/artists/:id',
    children: [
      {
        path: '',
        name: RouteName.ARTIST,
        component: () => import('@/screens/Artist.vue'),
        props: true,
      },
      {
        path: 'albums/:albumId',
        name: RouteName.ARTIST_ALBUM,
        component: () => import('@/screens/Album.vue'),
        props: true,
      },
    ],
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL ?? '/'),
  linkActiveClass: 'dock-active text-primary',
  routes,
})

export default router

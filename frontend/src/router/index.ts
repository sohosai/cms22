import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import { paths } from '@/const/config'
import Top from '@/components/Top.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path:'/:catchAll(.*)',
    component: Top,
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
})

export default router

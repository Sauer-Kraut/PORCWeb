import BracketsView from '@/views/BracketsView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import QaAView from '@/views/QaAView.vue'
import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Brackets',
    component: BracketsView,
  },
  {
    path: '/qaa',
    name: 'QaA',
    component: QaAView,
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'Page not found',
    component: NotFoundView,
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router

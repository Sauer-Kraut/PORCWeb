import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import Brackets from '../views/Brackets.vue'
import QaA from '../views/QaA.vue'
import NotFound from '../views/NotFound.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Brackets',
    component: Brackets,
  },
  {
    path: '/qaa',
    name: 'QaA',
    component: QaA,
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'Page not found',
    component: NotFound,
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router

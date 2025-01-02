import TournamentView from '@/views/TournamentView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import QaAView from '@/views/QaAView.vue'
import RulesView from '@/views/RulesView.vue'
import SignUpView from '@/views/SignUpView.vue'
import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Tournament',
    component: TournamentView,
  },
  {
    path: '/rules',
    name: 'Rules',
    component: RulesView,
  },
  {
    path: '/qaa',
    name: 'QaA',
    component: QaAView,
  },
  {
    path: '/sign-up',
    name: 'Sign up',
    component: SignUpView,
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

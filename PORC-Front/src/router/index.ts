import TournamentView from '@/views/TournamentView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import FAQView from '@/views/FAQView.vue'
import RulesView from '@/views/RulesView.vue'
import SignUpView from '@/views/SignUpView.vue'
import MatchPlannerView from '@/views/MatchPlannerView.vue';
import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'Tournament',
        component: TournamentView,
    },
    {
        path: '/match-planner',
        name: 'Match Planner',
        component: MatchPlannerView,
    },
    {
        path: '/rules',
        name: 'Rules',
        component: RulesView,
    },
    {
    path: '/faq',
    name: 'FAQ',
    component: FAQView,
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'Page not found',
        component: NotFoundView,
    },
    {
        path: '/signup',
        name: 'Sign Up',
        component: SignUpView,
    },
    {
        path: '/discord/callback',
    redirect: '/',
    },
];

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
});

export default router;

import TournamentView from '@/views/TournamentView.vue';
import NotFoundView from '@/views/NotFoundView.vue';
import FAQView from '@/views/FAQView.vue';
import RulesView from '@/views/RulesView.vue';
import MatchPlannerView from '@/views/MatchPlannerView.vue';
import CallbackCatchView from '@/views/CallbackCatchView.vue';
import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router';
import OrganizerView from '@/views/OrganizerView.vue';

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
        path: '/discord/callback',
        name: 'Callback catch',
        component: CallbackCatchView,
    },
    // {
    //     path: '/organizer',
    //     name: 'Organizer',
    //     component: OrganizerView,
    // },
];

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
    scrollBehavior(to, from, savedPosition) {

        // if there's a hash, try to scroll to that element (wait briefly for DOM to mount)
        if (to.hash) {
            return new Promise((resolve) => {
                setTimeout(() => {
                    const el = document.querySelector(to.hash as string);
                    if (el) {
                        resolve({ el: to.hash, behavior: 'smooth' });
                    } else {
                        resolve({ top: 0 });
                    }
                }, 50); // increase if your target mounts async
            });
        }

        // default: top
        return { top: 0 };
    },
});

export default router;

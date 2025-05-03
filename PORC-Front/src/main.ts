import { createApp, Transition } from 'vue';
import App from './App.vue';
import router from './router';
import { createVfm } from 'vue-final-modal';
import FloatingVue, { PopperWrapper } from 'floating-vue';
import './assets/icons/style.css';
import 'vue-final-modal/style.css';
import '@vuepic/vue-datepicker/dist/main.css';
import 'floating-vue/dist/style.css';
import './assets/scss/tooltip/match-tooltip.scss';
import './assets/scss/styles.scss';
import './assets/scss/pages.scss';
import './assets/scss/global.scss';

const app = createApp(App);

app.use(FloatingVue, {
    themes: {
        'match-tooltip': {
            $extend: 'menu',
            triggers: ['click', 'hover'],
            delay: {
                show: 250,
                hide: 200,
            },
            autoHide: true,
            eagerMount: true,
            placement: 'bottom',
        },
        'match-request-tooltip': {
            $extend: 'match-tooltip',
        },
        'match-declined-tooltip': {
            $extend: 'match-tooltip',
        },
    },
});
app.use(router);
app.use(createVfm());
app.mount('#app');

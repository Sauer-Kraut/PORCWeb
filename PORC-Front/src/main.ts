import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import { createVfm } from 'vue-final-modal';
import FloatingVue, { PopperWrapper } from 'floating-vue';
import './assets/scss/styles.scss';
import './assets/scss/global.scss';
import './assets/icons/style.css';
import 'vue-final-modal/style.css';
import '@vuepic/vue-datepicker/dist/main.css';
import 'floating-vue/dist/style.css';
import './assets/scss/tooltip/match-tooltip.scss';

const app = createApp(App);

app.use(FloatingVue, {
    themes: {
        'match-tooltip': {
            $extend: 'menu',
            triggers: ['click', 'hover'],
            delay: {
                show: 0,
                hide: 200,
            },
            autoHide: true,
            eagerMount: true,
            placement: 'bottom',
        },
    },
});
app.use(router);
app.use(createVfm());
app.mount('#app');

export default {
    ...PopperWrapper,
    name: 'VMatchTooltip',
    vPopperTheme: 'match-tooltip',
};

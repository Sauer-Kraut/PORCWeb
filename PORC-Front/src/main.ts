import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import './assets/scss/styles.scss';
import './assets/scss/global.scss';
import './assets/icons/style.css';
import 'vue-final-modal/style.css'
import { createVfm } from 'vue-final-modal';

const app = createApp(App);

app.use(router);
app.use(createVfm());
app.mount('#app');

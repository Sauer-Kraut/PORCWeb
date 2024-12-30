import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import './assets/scss/styles.scss'
import './assets/scss/global.scss'

const app = createApp(App)

app.use(router)
app.mount('#app')

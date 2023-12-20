import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import fetchReq from './plugins/fetchReq.js'

const app = createApp(App)

app.use(router)
app.use(fetchReq)

app.mount('#app')

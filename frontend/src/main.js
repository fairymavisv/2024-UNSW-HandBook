import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import fetchReq from './plugins/fetchReq.js'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

const app = createApp(App)

app.use(router)
app.use(ElementPlus)
app.use(fetchReq)

app.mount('#app')

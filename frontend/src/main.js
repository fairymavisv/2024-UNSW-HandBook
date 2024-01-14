import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import fetchReq from './plugins/fetchReq.js'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue' //import all icons

const app = createApp(App)

app.use(router)
app.use(ElementPlus)
app.use(fetchReq)

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
  }

app.mount('#app')

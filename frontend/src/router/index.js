import { createRouter, createWebHistory } from 'vue-router'

const routes = [

  {
    path: '/courseList',
    name: 'courseList',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "CourseList" */ '../views/CourseList.vue')
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router

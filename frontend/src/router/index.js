import { createRouter, createWebHistory } from 'vue-router'
import AuthPage from '@/views/AuthPage.vue'
import CourseProfile from '@/views/CourseProfile.vue'

const routes = [
  { path: "/", redirect: "/login" },
  { path: "/login", component: AuthPage},
  { path: "/register", component: AuthPage},

  // route for course profile
  { path: "/courseprofile/:courseId", component: CourseProfile},

  {
    path: '/courseList',
    name: 'courseList',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "CourseList" */ '@/views/CourseList.vue')
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
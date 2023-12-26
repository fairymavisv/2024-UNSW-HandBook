import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import LoginView from '../views/LoginView.vue';
import RegisterView from '../views/RegisterView.vue';

const routes = [

  {
    path: '/',
    name: 'login',
    component: LoginView
  },

  {
    path: '/signup',
    name: 'signup',
    component: RegisterView

  },

  // {
  //   path: '/forgot-password',
  //   name: 'ForgotPassword',
  //   component: ForgotPassword
  // },

  {
    path: '/home',
    name: 'home',
    component: HomeView
  },
  
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

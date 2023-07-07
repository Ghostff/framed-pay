import { createRouter, createWebHistory } from 'vue-router'
import Login from "@/views/authentication/Login.vue";
import Register from "@/views/authentication/Register.vue";
import ForgotPassword from "@/views/authentication/ForgotPassword.vue";
import LandingPage from "@/views/LandingPage.vue";
import Dashboard from "@/views/dashboard/Dashboard.vue";
import Index from "@/views/dashboard/pages/Index.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/login',
      name: 'login',
      redirect: to => ({ name: 'landing-page', query: { show: 'login' } }),
    },
    {
      path: '/register',
      name: 'register',
      redirect: to => ({ name: 'landing-page', query: { show: 'register' } }),
    },
    {
      path: '/forgot-password',
      name: 'forgot-password',
      redirect: to => ({ name: 'landing-page', query: { show: 'forgot-password' } }),
    },
    {
      path: '/',
      name: 'landing-page',
      component: LandingPage
    },
    /*{
      path: '/',
      component: Dashboard,
      children: [
        { path: '', name: 'dashboard', component: Index },
      ]
    },*/
  ]
})

export default router


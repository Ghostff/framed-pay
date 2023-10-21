import { createRouter, createWebHistory } from 'vue-router';
import LandingPage from '@/views/LandingPage.vue';
import Dashboard from '@/views/dashboard/Dashboard.vue';
import Index from '@/views/dashboard/pages/Index.vue';
import Profile from "@/views/dashboard/pages/Profile.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'landing-page',
      component: LandingPage,
      children: [
        {
          path: 'login',
          name: 'login',
          redirect: () => ({ name: 'landing-page', query: { show: 'login' } }),
          meta: {
            requiresAuth: false
          }
        },
        {
          path: 'register',
          name: 'register',
          redirect: () => ({ name: 'landing-page', query: { show: 'register' } })
        },
        {
          path: 'forgot-password',
          name: 'forgot-password',
          redirect: () => ({ name: 'landing-page', query: { show: 'forgot-password' } })
        },
        {
          path: 'password-reset/:token/:uid',
          name: 'password-reset',
          redirect: (r) => ({
            name: 'landing-page', query: { show: 'password-reset', ...r.params } })
        }
      ]
    },
    { path: '/documentation', name: 'documentation', component: Profile },
    {
      path: '/dashboard',
      component: Dashboard,
      meta: {
        requiresAuth: true
      },
      children: [
        { path: '', name: 'dashboard', component: Index },
        { path: 'profile', name: 'profile', component: Profile },
        { path: 'integrations', name: 'integrations', component: Profile },
        { path: 'integrations/:id', name: 'integrations-vendor', component: Profile },
        { path: 'plans', name: 'plans', component: Profile },
      ]
    }
  ]
});

export default router;

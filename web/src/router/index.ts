import { createRouter, createWebHistory } from 'vue-router';
import LandingPage from '@/views/LandingPage.vue';
import Dashboard from '@/views/dashboard/Dashboard.vue';
import Index from '@/views/dashboard/pages/Index.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'landing-page',
      component: LandingPage,
      children: [
        {
          path: '/login',
          name: 'login',
          redirect: () => ({ name: 'landing-page', query: { show: 'login' } }),
          meta: {
            requiresAuth: false
          }
        },
        {
          path: '/register',
          name: 'register',
          redirect: () => ({ name: 'landing-page', query: { show: 'register' } })
        },
        {
          path: '/forgot-password',
          name: 'forgot-password',
          redirect: () => ({ name: 'landing-page', query: { show: 'forgot-password' } })
        }
      ]
    },
    {
      path: '/dashboard',
      component: Dashboard,
      meta: {
        requiresAuth: true
      },
      children: [{ path: '', name: 'dashboard', component: Index }]
    }
  ]
});

export default router;

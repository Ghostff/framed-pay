import { createApp } from 'vue';
import App from '@/App.vue';
import router from '@/router';
import axios from 'axios';
import { library } from '@fortawesome/fontawesome-svg-core';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import icons from '@/fonts';

import '@/assets/css/app.scss';
import 'preline';
import { useAuthStore } from '@/stores/auth';
import { createPinia } from 'pinia';
import { UserInterface } from '@/models/user';

library.add(...icons);

axios.defaults.baseURL = 'http://localhost/api';
axios.defaults.headers.post['Content-Type'] = 'application/json';
axios.defaults.withCredentials = true;

// Important: If axios is used with multiple domains, the AUTH_TOKEN will be sent to all of them.
// See below for an example using Custom instance defaults instead.
// axios.defaults.headers.common['Authorization'] = AUTH_TOKEN;

const pinia = createPinia();
const app = createApp(App);
const nonAuthRoutes = ['login', 'register', 'forgot-password'];
// Navigation guard to check authentication status
router.beforeEach(async (to, from, next) => {
  const store = useAuthStore();
  if (to.meta.requiresAuth && !store.isAuthenticated) {
    next({ name: 'login' });
  } else if (
    (nonAuthRoutes.includes(to.name as string) || nonAuthRoutes.includes(to.query.show as string)) &&
    store.isAuthenticated
  ) {
    next({ name: 'dashboard' });
  } else {
    next();
  }

  // remove modal overlay on page change.
  document.querySelector('.hs-overlay-backdrop')?.remove();
});

app.use(router);
app.use(pinia);
app.component('font-awesome-icon', FontAwesomeIcon);

axios
  .get('user')
  .then(({ data }: { data: { data: UserInterface } }) => useAuthStore().login(data.data))
  .finally(() => app.use(router).mount('#app'))
  .catch((error) => {
    if (error.response && error.response.status === 401 && error.response.data.message === 'Unauthenticated') {
      useAuthStore().logout();
    }
  });

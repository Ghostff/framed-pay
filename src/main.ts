import { createApp } from 'vue';
import App from '@/App.vue';
import router from '@/router';
import axios from 'axios';

import '@/assets/css/app.scss';
import 'preline';


axios.defaults.baseURL = 'http://localhost:80/api';
axios.defaults.headers.post['Content-Type'] = 'application/json';

// Important: If axios is used with multiple domains, the AUTH_TOKEN will be sent to all of them.
// See below for an example using Custom instance defaults instead.
// axios.defaults.headers.common['Authorization'] = AUTH_TOKEN;

const app = createApp(App);

app.use(router)
app.mount('#app')

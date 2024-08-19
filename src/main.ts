import App from '@/App.vue';
import router from '@/router';
import '@/assets/index.css';
import { createApp } from 'vue';
import { createManager } from '@vue-youtube/core';
import { createPinia } from 'pinia';

const app = createApp(App);

app.use(router);
app.use(createManager());
app.use(createPinia());

app.mount('#app');

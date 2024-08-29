import App from '@/App.vue';
import router from '@/router';
import '@/assets/index.css';
import { english, MessageFormat } from '@/translations';
import { createApp } from 'vue';
import { createManager } from '@vue-youtube/core';
import { createI18n } from 'vue-i18n';

const app = createApp(App);

app.use(router);
app.use(createManager());
app.use(createI18n<[MessageFormat], 'en'>({
  globalInjection: true,
  locale: 'en',
  fallbackLocale: 'en',
  legacy: false,
  messages: {
    en: english,
  }
}));

app.mount('#app');

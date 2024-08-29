import App from '@/App.vue';
import router from '@/router';
import '@/assets/index.css';
import english from '@/translations/english';
import french from '@/translations/french';
import { createApp } from 'vue';
import { createManager } from '@vue-youtube/core';
import { createI18n } from 'vue-i18n';

export type MessageFormat = {
  [key: string]: string | MessageFormat;
}

const app = createApp(App);

app.use(router);
app.use(createManager());
app.use(createI18n<[MessageFormat], 'en' | 'fr'>({
  globalInjection: true,
  locale: 'en',
  fallbackLocale: 'en',
  legacy: false,
  messages: {
    en: english,
    fr: french,
  }
}));

app.mount('#app');

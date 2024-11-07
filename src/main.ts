import App from '@/App.vue';
import router from '@/router';
import '@/assets/index.css';
import english from '@/translations/english';
import french from '@/translations/french';
import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import { createPinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia';

export type MessageFormat = {
  [key: string]: string | MessageFormat;
}

const app = createApp(App);

const pinia = createPinia();
pinia.use(createPlugin());

app.use(router);
app.use(pinia);

app.use(createI18n<[MessageFormat], 'en' | 'fr'>({
  globalInjection: true,
  fallbackLocale: 'en',
  legacy: false,
  messages: {
    en: english,
    fr: french,
  }
}));

app.mount('#app');

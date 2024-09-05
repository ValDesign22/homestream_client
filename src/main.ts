import App from '@/App.vue';
import router from '@/router';
import '@/assets/index.css';
import english from '@/translations/english';
import french from '@/translations/french';
import { createApp } from 'vue';
import { createManager } from '@vue-youtube/core';
import { createI18n } from 'vue-i18n';
import { useStore } from './lib/stores';
// import WebSocket from '@tauri-apps/plugin-websocket';
// import { invoke } from '@tauri-apps/api/core';
// import { IConfig } from './utils/types';

export type MessageFormat = {
  [key: string]: string | MessageFormat;
}

const app = createApp(App);

// const config = await invoke<IConfig | null>("get_config");
// if (config) {
//   console.log(config.ws_url);
//   const ws = await WebSocket.connect(config.ws_url);

//   ws.addListener((msg) => {

//   });
// }

const store = useStore;

app.use(router);
app.use(createManager());
app.use(createI18n<[MessageFormat], 'en' | 'fr'>({
  globalInjection: true,
  locale: (await store.getLocale()) || 'en',
  fallbackLocale: 'en',
  legacy: false,
  messages: {
    en: english,
    fr: french,
  }
}));

app.mount('#app');

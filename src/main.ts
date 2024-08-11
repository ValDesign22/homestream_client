import { createApp } from "vue";
import App from "@/App.vue";
import router from "@/router";
import "@/assets/index.css";
// import { invoke } from "@tauri-apps/api/core";
// import { fetch } from "@tauri-apps/plugin-http";
// import { Config } from "@/utils/types";

async function setup() {
  // const config = await invoke<Config | null>('get_config');
  // if (config) {
  //   const response = await fetch(config.http_server + '/setup');
  //   if (!response.ok) throw new Error('Error setting up frontend: ' + response.status);
  // }
}

window.addEventListener('DOMContentLoaded', () => {
  setup().then(() => {
    const app = createApp(App);

    app.use(router);

    app.mount("#app");
  }).catch((error) => {
    console.error('Error setting up frontend:', error);
  });
});

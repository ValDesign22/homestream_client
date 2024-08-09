import { createApp } from "vue";
import App from "@/App.vue";
import router from "@/router";
import "@/assets/index.css";
import { invoke } from "@tauri-apps/api/core";

async function setup() {
    await invoke('setup');
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

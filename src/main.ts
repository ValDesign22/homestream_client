import { createApp } from "vue";
import App from "./App.vue";
import router from "@/router";
import "./assets/index.css";
import { invoke } from "@tauri-apps/api/core";

function sleep(seconds: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, seconds * 1000));
}

async function setup() {
    await sleep(3);
    invoke('setup');
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

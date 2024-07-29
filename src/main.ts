import { createApp } from "vue";
import App from "./App.vue";
import "./assets/index.css";
import { invoke } from "@tauri-apps/api/core";

function sleep(seconds: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, seconds * 1000));
}

async function setup() {
    await sleep(3);
    invoke('set_complete', {task: 'frontend'})
}

window.addEventListener('DOMContentLoaded', () => {
  setup().then(() => {
    createApp(App).mount("#app");
  }).catch((error) => {
    console.error('Error setting up frontend:', error);
  });
});

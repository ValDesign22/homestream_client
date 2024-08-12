import { createApp } from "vue";
import App from "@/App.vue";
import router from "@/router";
import "@/assets/index.css";
import { createManager } from '@vue-youtube/core';

async function setup() {
}

setup().then(() => {
  const app = createApp(App);

  app.use(router);
  app.use(createManager());

  app.mount("#app");
}).catch((error) => {
  console.error('Error setting up frontend:', error);
});

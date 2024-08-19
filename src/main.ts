import { createApp } from "vue";
import App from "@/App.vue";
import router from "@/router";
import "@/assets/index.css";
import { createManager } from '@vue-youtube/core';

const app = createApp(App);

app.use(router);
app.use(createManager());

app.mount("#app");

<script setup lang="ts">
import { onBeforeMount, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { check } from '@tauri-apps/plugin-updater';
import { useRouter } from "vue-router";
import { IConfig } from "@/utils/types";
import useStore from "./lib/stores";
import { useI18n } from "vue-i18n";
import { useColorMode } from "@vueuse/core";

const router = useRouter();
const store = useStore();
const i18n = useI18n();
useColorMode();

onMounted(() => {
  void store.$tauri.start();
  document.documentElement.classList.add(`theme-${store.theme}`);
  i18n.locale.value = store.locale;
});
onBeforeMount(async () => {
  if (!import.meta.env.DEV) {
    const update = await check();
    if (update) console.log(`Found update ${update.version} from ${update.date} with notes ${update.body}`);
    else console.log("No updates found");
  }

  console.log("Checking if the app is registered");
  const config = await invoke<IConfig | null>("get_config");
  if (!config) return router.push({ path: "/register" });
});
</script>

<template>
  <RouterView />
</template>
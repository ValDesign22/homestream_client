<script setup lang="ts">
import { onBeforeMount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { check } from '@tauri-apps/plugin-updater';
import { useRouter } from "vue-router";
import { IConfig } from "@/utils/types";

const router = useRouter();

onBeforeMount(async () => {
  if (!import.meta.env.DEV) {
    const update = await check();
    if (update) console.log(`Found update ${update.version} from ${update.date} with notes ${update.body}`);
    else console.log("No updates found");
  }

  console.log("Checking if the app is registered");
  const config = await invoke<IConfig | null>("get_config");
  if (!config) return router.push({ path: "/register", replace: true });
});
</script>

<template>
  <RouterView />
</template>
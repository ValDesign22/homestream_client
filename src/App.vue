<script setup lang="ts">
import { onBeforeMount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { check } from '@tauri-apps/plugin-updater';
import { useRouter } from "vue-router";
import { IConfig, IProfile } from "@/utils/types";
import { fetch } from "@tauri-apps/plugin-http";

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

  const profiles = await fetch(config.http_server + '/profiles', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json'
    }
  });

  if (!profiles.ok) return router.push({ path: "/profiles", replace: true });
  const data: IProfile[] = await profiles.json();
  if (data.length === 0) return router.push({ path: "/profiles", replace: true });
});
</script>

<template>
  <RouterView />
</template>
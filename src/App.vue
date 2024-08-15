<script setup lang="ts">
import { onBeforeMount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import { Config } from "@/utils/types";

const router = useRouter();

onBeforeMount(async () => {
  console.log("Checking if the app is registered");
  const config = await invoke<Config | null>("get_config");
  if (config === null) router.push({ path: "/register", replace: true });
});
</script>

<template>
  <RouterView />
</template>
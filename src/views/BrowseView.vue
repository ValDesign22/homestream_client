<script setup lang="ts">
import { Config, Movie, TvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { onMounted, ref } from 'vue';

const stores = ref<Record<string, Movie[] | TvShow[]>>({});

async function fetchStores(http_server: string) {
  if (!http_server) return;

  const response = await fetch(http_server + '/stores', {
    method: 'GET',
  });

  if (!response.ok) console.error('An error occurred while fetching the folders', await response.text());
  else stores.value = await response.json();
  console.log(stores.value);
}

onMounted(async () => {
  const config = await invoke<Config | null>("get_config");
  if (config) fetchStores(config.http_server);
});
</script>

<template>
  <div class="flex flex-col justify-center">
    <h1 class="text-4xl font-bold">
      Browse View
    </h1>
  </div>
</template>

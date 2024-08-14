<script setup lang='ts'>
import { NavBar } from '@/components/navbar';
import { Config, Movie, TvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const item = ref<Movie | TvShow | null>(null);

onMounted(async () => {
  const config = await invoke<Config | null>('get_config');
  if (config) {
    const route = router.currentRoute.value;
    const itemId = route.params.id;
    if (!itemId) router.push({ path: '/browse' });

    const details = await fetch(config.http_server + `/details?id=${itemId}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    if (details.ok) {
      const response = await details.json();
      item.value = response;
    }
    else router.push({ path: '/browse' });
  }
  else router.push({ path: '/register', replace: true });
});
</script>

<template>
  <NavBar />
</template>
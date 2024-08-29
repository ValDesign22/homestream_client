<script setup lang="ts">
import TMDBImage from '@/components/image/TMDBImage.vue';
import { NavBar } from '@/components/navbar';
import { IConfig, IMovie, ITvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { onMounted, onUnmounted, ref } from 'vue';
import { watch } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();

const stores = ref<Record<string, IMovie[] | ITvShow[]>>({});
const results = ref<(IMovie | ITvShow)[]>([]);

function search(query: string) {
  results.value = [];
  for (const store in stores.value) {
    results.value.push(...stores.value[store].filter((item) => {
      return item.title.toLowerCase().includes(query.toLowerCase())
        || item.original_title.toLowerCase().includes(query.toLowerCase())
    }));
  }
}

watch(route, () => {
  search(route.query.q as string);
});

onMounted(async () => {
  const config = await invoke<IConfig | null>('get_config');
  if (config) {
    const response = await fetch(config.http_server + '/stores', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (response.ok) {
      stores.value = await response.json();
      search(route.query.q as string);
    }
  }
});

onUnmounted(() => {
  stores.value = {};
  results.value = [];
});
</script>

<template>
  <NavBar full />
  <div class="flex flex-col py-24 px-4">
    <h1 v-if="results.length === 0" class="text-2xl">
      {{ $t('pages.search.noResults', { query: route.query.q }) }}
    </h1>
    <div v-else class="flex flex-col gap-4">
      <h1 class="text-2xl">
        {{ $t('pages.search.results', { query: route.query.q }) }}
      </h1>
      <div class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-6 gap-4">
        <div v-for="result in results" :key="result.id" class="overflow-hidden rounded-lg shadow-lg">
          <TMDBImage
            :image="result.poster_path"
            :alt="result.title"
            type="poster"
            size="w500"
            class="rounded-lg object-cover hover:scale-105 transition-transform cursor-pointer"
            @click="() => $router.push({ path: `/details/${result.id}` })"
          />
        </div>
      </div>
    </div>
  </div>
</template>
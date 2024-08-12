<script setup lang="ts">
import { TMDBImage } from '@/components/image';
import { NavBar } from '@/components/navbar';
import { Config, Movie, TvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { onMounted, ref } from 'vue';

const stores = ref<Record<string, Movie[] | TvShow[]>>({});
const randomSelected = ref<Movie | TvShow | null>(null);

async function fetchStores(http_server: string) {
  if (!http_server) return;

  const response = await fetch(http_server + '/stores', {
    method: 'GET',
  });

  if (!response.ok) console.error('An error occurred while fetching the folders', await response.text());
  else {
    stores.value = await response.json();
    randomSelected.value = selectRandomTopRated(stores.value);
  }
}

function selectRandomTopRated(store: Record<string, Movie[] | TvShow[]>): Movie | TvShow {
  const keys = Object.keys(store);
  const randomKey = keys[Math.floor(Math.random() * keys.length)];
  const randomStore = store[randomKey];
  const randomItem = randomStore[Math.floor(Math.random() * randomStore.length)];
  return randomItem;
}

onMounted(async () => {
  const config = await invoke<Config | null>("get_config");
  if (config) fetchStores(config.http_server);
});
</script>

<template>
  <NavBar />
  <div class="flex flex-col justify-center">
    <div v-if="Object.keys(stores).length === 0">
      <p>Loading...</p>
    </div>
    <div v-else class="w-full h-auto">
      <div v-if="randomSelected" class="w-full relative">
        <TMDBImage
          :image="randomSelected.backdrop_path"
          :alt="randomSelected.id.toString()"
          type="backdrop"
          size="w1280"
          class="w-full h-full object-cover"
        />
        <div class="absolute bottom-0 left-0 w-full h-full flex justify-end flex-col p-4 bg-gradient-to-t from-black to-transparent">
          <h2 class="text-2xl font-bold text-white">{{ randomSelected.title }}</h2>
          <p class="text-white">{{ randomSelected.overview }}</p>
        </div>
      </div>
      <div class="grid grid-cols-2 gap-4">
        <div v-for="(store, key) in stores" :key="key" class="flex flex-col">
          <h2 class="text-2xl font-bold">{{ key }}</h2>
          <div class="grid grid-cols-2 gap-4">
            <div v-for="item in store" :key="item.id" class="flex flex-col">
              <TMDBImage
                :image="item.poster_path"
                :alt="item.id.toString()"
                type="poster"
                size="w185"
                class="w-full h-auto object-cover"
              />
              <p class="text-center">{{ item.title }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

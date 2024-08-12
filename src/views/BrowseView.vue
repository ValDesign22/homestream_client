<script setup lang="ts">
import { TMDBImage } from '@/components/image';
import { NavBar } from '@/components/navbar';
import { Button } from '@/components/ui/button';
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel';
import { Skeleton } from '@/components/ui/skeleton';
import { Config, Movie, TvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { InfoIcon, PlayIcon } from 'lucide-vue-next';
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

<template class="w-full h-screen flex flex-col justify-center items-center">
  <NavBar />
  <div class="flex flex-col justify-center">
    <div v-if="Object.keys(stores).length === 0" class="w-full h-auto">
      <div v-for="(_, index) in 10" :key="index" class="w-full relative">
        <div class="flex flex-col gap-8 p-16 bg-black">
          <Carousel
            class="relative w-full"
            :opts="{
              align: 'center',
            }"
          >
            <CarouselContent>
              <CarouselItem v-for="(_, index) in 25" :key="index" class="flex-grow p-1 basis-auto">
                <div class="p-1">
                  <Skeleton class="w-[185px] h-[278px] rounded-lg" />
                </div>
              </CarouselItem>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
          </Carousel>
        </div>
      </div>
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
        <div class="absolute bottom-0 left-0 w-full h-full flex justify-end flex-col p-12 bg-gradient-to-t from-black to-transparent gap-4">
          <TMDBImage
            v-if="randomSelected.logo_path"
            :image="randomSelected.logo_path"
            :alt="randomSelected.id.toString()"
            type="logo"
            size="original"
            class="w-96 h-auto object-cover"
          />
          <h2 v-else class="text-2xl font-bold text-white">{{ randomSelected.title }}</h2>
          <p class="text-white max-w-2xl">{{ randomSelected.overview.split(' ').slice(0, 20).join(' ') }}...</p>
          <div class="flex gap-4">
            <Button variant="secondary" class="flex items-center gap-2" @click="() => $router.push({ path: `/watch/${randomSelected!.id}` })">
              <PlayIcon class="w-6 h-6" />
              <span>Play</span>
            </Button>
            <Button class="flex items-center gap-2" @click="() => $router.push({ path: `/details/${randomSelected!.id}` })">
              <InfoIcon class="w-6 h-6" />
              <span>Details</span>
            </Button>
          </div>
        </div>
      </div>
      <div class="flex flex-col gap-8 p-16 bg-black">
        <div 
          v-for="(store, key) in stores"
          :key="key"
          class="flex flex-col gap-4"
        >
          <h2 class="text-2xl font-bold text-white">{{ key }}</h2>
          <Carousel
            class="relative w-full"
            :opts="{
              align: 'start',
            }"
          >
            <CarouselContent>
              <CarouselItem
                v-for="item in store.slice(0, 25)"
                :key="item.id"
                class="flex-grow p-1 basis-auto"
              >
                <div class="p-1">
                  <TMDBImage
                    :image="item.poster_path"
                    :alt="item.id.toString()"
                    type="poster"
                    size="w185"
                    class="w-full h-auto object-cover rounded-lg"
                  />
                </div>
              </CarouselItem>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
          </Carousel>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { TMDBImage } from '@/components/image';
import { NavBar } from '@/components/navbar';
import { Button } from '@/components/ui/button';
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel';
import { Skeleton } from '@/components/ui/skeleton';
import { Config, Genre, Movie, TvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { InfoIcon, PlayIcon } from 'lucide-vue-next';
import { onMounted, ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onUnmounted } from 'vue';
import { vIntersectionObserver } from '@vueuse/components'
import { useImage } from '@vueuse/core';
import { PlayerState, usePlayer } from '@vue-youtube/core';

const stores = ref<Record<string, Movie[] | TvShow[]>>({});
const genres = ref<Genre[]>([]);
const randomSelected = ref<Movie | TvShow | null>(null);

const { isLoading } = useImage({ src: randomSelected?.value?.backdrop_path ?? '' });

const windowFocused = ref(true);
const headerVisible = ref(true);
const showFullOverview = ref(false);

const videoKey = ref<string>('');
const videoPlayer = ref();
const videoPlaying = ref(true);
const videoError = ref(false);

const { instance, onError, onStateChange, onReady } =  usePlayer(videoKey, videoPlayer, {
  playerVars: {
    autoplay: 1,
    controls: 0,
    iv_load_policy: 3,
    rel: 0,
    showinfo: 0,
    modestbranding: 1,
    loop: 0,
  }
});

onError((event) => {
  console.error('An error occurred while playing the video', event.data)
  videoPlaying.value = false;
  videoError.value = true;
});

onStateChange((event) => {
  if (videoKey.value === '') return;
  if (videoError.value) return;
  if (event.data === PlayerState.PLAYING) videoPlaying.value = true;
  if (event.data === PlayerState.ENDED) {
    videoPlaying.value = false
    videoKey.value = '';
  };
  if (event.data === PlayerState.UNSTARTED) videoPlaying.value = false;
});

onReady((event) => {
  if (!event.target) return;
  if (videoKey.value === '') return;
  if (videoError.value) return;
  event.target.playVideo();
  videoPlaying.value = true;
});

function onIntersectionObserver([{ isIntersecting }]: IntersectionObserverEntry[]) {
  headerVisible.value = isIntersecting
}

async function fetchStores(http_server: string) {
  if (!http_server) return;

  const response = await fetch(http_server + '/stores', {
    method: 'GET',
  });

  if (!response.ok) console.error('An error occurred while fetching the folders', await response.text());
  else {
    stores.value = await response.json();
    genres.value = getGenres(stores.value);
    randomSelected.value = await selectRandomTopRated(stores.value, http_server);
  }
}

function getGenres(stores: Record<string, Movie[] | TvShow[]>) {
  const genres: Genre[] = [];
  for (const key in stores) {
    const store = stores[key];
    store.forEach((item) => {
      item.genres.forEach((genre) => {
        if (!genres.find((g) => g.id === genre.id)) genres.push(genre);
        const genreIndex = genres.findIndex((g) => g.id === genre.id);
        if (!genres[genreIndex].items) genres[genreIndex].items = [];
        genres[genreIndex].items.push(item);
      });
    });
  }
  return genres;
}

async function selectRandomTopRated(store: Record<string, Movie[] | TvShow[]>, http_server: string): Promise<Movie | TvShow> {
  const keys = Object.keys(store);
  const randomKey = keys[Math.floor(Math.random() * keys.length)];
  const randomStore = store[randomKey];
  const randomItem = randomStore[Math.floor(Math.random() * randomStore.length)];
  
  const previewVideoKey = await fetch(`${http_server}/preview?id=${randomItem.id}`, {
    method: 'GET',
  });
  if (previewVideoKey.ok) {
    const response = await previewVideoKey.text();
    videoKey.value = response || '';
  }

  return randomItem;
}

const interval = setInterval(async () => {
  windowFocused.value = await getCurrentWindow().isFocused();
  if (videoKey.value === '' || videoError.value || !instance.value) return videoPlaying.value = false;
  if (!instance.value?.getDuration) return videoPlaying.value = false;
  if (!windowFocused.value || !headerVisible.value) {
    instance.value.pauseVideo();
    videoPlaying.value = false;
  } else {
    instance.value.playVideo();
    videoPlaying.value = true;
  }
}, 1000);

onMounted(async () => {
  const config = await invoke<Config | null>("get_config");
  if (config) fetchStores(config.http_server);
});

onUnmounted(() => clearInterval(interval));
</script>

<template class="w-full h-screen flex flex-col justify-center items-center">
  <NavBar />
  <div
    ref="videoPlayer"
    class="w-full h-screen object-cover absolute top-0 left-0"
    :class="{ 'z-10': videoPlaying && !videoError, 'z-0': !videoPlaying || videoError }"
  />
  <div class="flex flex-col justify-center">
    <div v-if="Object.keys(stores).length === 0" class="w-full h-auto">
      <div v-for="(_, index) in 10" :key="index" class="w-full relative">
        <div class="flex flex-col gap-8 p-16 bg-black">
          <Carousel class="relative w-full" :opts="{ align: 'center' }">
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
      <div v-if="randomSelected" class="w-full h-screen relative" v-intersection-observer="onIntersectionObserver">
        <Skeleton v-if="isLoading" class="w-full h-full" />
        <TMDBImage
          v-if="!isLoading"
          :image="randomSelected.backdrop_path"
          :alt="randomSelected.id.toString()"
          type="backdrop"
          size="w1280"
          class="w-full h-full object-cover relative z-0"
          :class="{ 'z-[11]': !videoPlaying }"
        />
        <div class="absolute z-[12] bottom-0 left-0 w-full h-full flex justify-end flex-col p-12 gap-4 bg-gradient-to-t from-black from-10% to-transparent">
          <TMDBImage
            v-if="randomSelected.logo_path"
            :image="randomSelected.logo_path"
            :alt="randomSelected.id.toString()"
            type="logo"
            size="original"
            class="w-[30vw] h-auto object-cover"
          />
          <h2 v-else class="text-4xl font-bold sm:text-3xl text-white">{{ randomSelected.title }}</h2>
          <p class="text-white max-w-2xl" @click="showFullOverview = !showFullOverview">
            {{ showFullOverview ? randomSelected.overview : randomSelected.overview.split(' ').slice(0, 50).join(' ') + '...' }}
          </p>
          <div class="flex gap-4">
            <Button variant="secondary" class="flex items-center gap-2" @click="() => $router.push({ path: `/watch/${randomSelected!.id}`, replace: true })">
              <PlayIcon class="w-6 h-6" />
              <span>Play</span>
            </Button>
            <Button class="flex items-center gap-2" @click="() => $router.push({ path: `/details/${randomSelected!.id}`, replace: true })">
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
                    class="w-full h-auto object-cover rounded-lg cursor-pointer"
                    @click="() => $router.push({ path: `/details/${item.id}`, replace: true })"
                  />
                </div>
              </CarouselItem>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
          </Carousel>
        </div>
        <div v-for="(genre, key) in genres"
          :key="key"
          class="flex flex-col gap-4"
        >
          <h2 class="text-2xl font-bold text-white">{{ genre.name }}</h2>
          <Carousel
            class="relative w-full"
            :opts="{
              align: 'start',
            }"
          >
            <CarouselContent>
              <CarouselItem
                v-for="item in genre.items.slice(0, 25)"
                :key="item.id"
                class="flex-grow p-1 basis-auto"
              >
                <div class="p-1">
                  <TMDBImage
                    :image="item.poster_path"
                    :alt="item.id.toString()"
                    type="poster"
                    size="w185"
                    class="w-full h-auto object-cover rounded-lg cursor-pointer"
                    @click="() => $router.push({ path: `/details/${item.id}`, replace: true })"
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

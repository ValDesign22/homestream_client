<script setup lang="ts">
import { TMDBImage } from '@/components/image';
import { NavBar } from '@/components/navbar';
import { Button } from '@/components/ui/button';
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel';
import { Skeleton } from '@/components/ui/skeleton';
import { EMediaType, IConfig, IGenre, IMovie, IProfile, ITvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { InfoIcon, PlayIcon } from 'lucide-vue-next';
import { computed, onMounted, ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onUnmounted } from 'vue';
import { vIntersectionObserver } from '@vueuse/components'
import { PlayerState, usePlayer } from '@vue-youtube/core';
import { useStore } from '@/lib/stores';
import { getMovieFromId, getTvShowFromEpisode } from '@/utils/video';
import { useGamepad } from '@vueuse/core';

const isMobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);

const store = useStore;
const user = ref<IProfile | null>(null);

const stores = ref<Record<string, IMovie[] | ITvShow[]>>({});
const genres = ref<IGenre[]>([]);
const randomSelected = ref<IMovie | ITvShow | null>(null);
const history = ref<(IMovie | ITvShow)[]>([]);

const windowFocused = ref(true);
const headerVisible = ref(true);
const showFullOverview = ref(false);

const videoKey = ref<string>('');
const videoPlayer = ref();
const videoPlaying = ref(false);
const videoError = ref(false);

const { isSupported, gamepads } = useGamepad();
const gamepad = computed(() => gamepads.value.find(g => g.mapping === 'standard'));

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

function getGenres(stores: Record<string, IMovie[] | ITvShow[]>) {
  const genres: IGenre[] = [];
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

async function selectRandomTopRated(store: Record<string, IMovie[] | ITvShow[]>, http_server: string): Promise<IMovie | ITvShow> {
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

async function parseHistory() {
  if (!user.value) return;
  const userHistory = await store.getHistory();
  if (!userHistory) return;
  userHistory
    .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
    .forEach(async (item) => {
      if (item.media_type === EMediaType.TvShow) {
        const tvShow = await getTvShowFromEpisode(item.id);
        if (tvShow && !history.value.find((h) => h.id === tvShow.id)) history.value.push(tvShow);
      } else {
        const movie = await getMovieFromId(item.id);
        if (movie) history.value.push(movie);
      }
    });
}

const interval = setInterval(async () => {
  windowFocused.value = isMobile ? document.visibilityState === 'visible' : await getCurrentWindow().isFocused();
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

const gamepadInterval = setInterval(() => {
  if (!isSupported.value) return;
  if (!gamepad.value) return;
  const focusableElements = Array.from(document.querySelectorAll('[tabindex="0"]'));
  if (focusableElements.length === 0) return;
  const focusedElement = document.activeElement;
  let currentIndex = focusableElements.indexOf(focusedElement as HTMLElement);
  if (gamepad.value.buttons[14].pressed || gamepad.value.axes[0] < -0.5) {
    currentIndex = currentIndex === 0 ? focusableElements.length - 1 : currentIndex - 1;
    (focusableElements[currentIndex] as HTMLElement).focus();
  }
  if (gamepad.value.buttons[15].pressed || gamepad.value.axes[0] > 0.5) {
    currentIndex = currentIndex === focusableElements.length - 1 ? 0 : currentIndex + 1;
    (focusableElements[currentIndex] as HTMLElement).focus();
  }
  if (gamepad.value.axes[3] < -0.5) window.scrollBy(0, -100);
  if (gamepad.value.axes[3] > 0.5) window.scrollBy(0, 100);
  if (gamepad.value.buttons[0].pressed) (focusedElement as HTMLElement).click();
}, 200);

onMounted(async () => {
  const config = await invoke<IConfig | null>("get_config");
  if (config) fetchStores(config.http_server);

  user.value = await store.getProfile();
  parseHistory();
});

onUnmounted(() => {
  clearInterval(interval)
  clearInterval(gamepadInterval)
});
</script>

<template>
  <NavBar full />
  <div
    ref="videoPlayer"
    tabindex="-1"
    class="w-full h-screen absolute top-0 left-0"
    :class="{ 'z-10': videoPlaying && !videoError, 'z-0': !videoPlaying || videoError }"
  />
  <div class="flex flex-col justify-center">
    <div v-if="Object.keys(stores).length === 0" class="w-full h-auto">
      <div v-for="(_, index) in 10" :key="index" class="w-full relative">
        <div class="flex flex-col gap-8 p-16">
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
        <TMDBImage
          :image="randomSelected.backdrop_path"
          :alt="randomSelected.id.toString()"
          type="backdrop"
          size="original"
          class="w-full h-full object-center object-cover relative"
          :class="{ 'z-[11]': !videoPlaying || videoError, 'z-[-1]': videoPlaying && !videoError }"
        />
        <div class="absolute z-[12] bottom-0 left-0 w-full h-full flex justify-end flex-col p-12 gap-4 bg-gradient-to-tr from-background from-10% to-transparent">
          <TMDBImage
            v-if="randomSelected.logo_path"
            :image="randomSelected.logo_path"
            :alt="randomSelected.id.toString()"
            type="logo"
            size="original"
            class="w-[20vw] h-auto object-cover"
          />
          <h2 v-else class="text-4xl font-bold sm:text-3xl">{{ randomSelected.title }}</h2>
          <span v-if="randomSelected.overview.length" class="max-w-2xl" @click="showFullOverview = !showFullOverview">
            {{ showFullOverview ? randomSelected.overview : randomSelected.overview.split(' ').slice(0, isMobile ? 10 : 50).join(' ') + '...' }}
          </span>
          <div class="flex gap-4">
            <Button tabindex="0" class="flex items-center gap-2" @click="() => $router.push({ path: `/watch/${randomSelected!.id}`, replace: true })">
              <PlayIcon class="w-6 h-6" />
              <span>
                {{ $t('pages.browse.watch') }}
              </span>
            </Button>
            <Button tabindex="0" variant="secondary" class="flex items-center gap-2" @click="() => $router.push({ path: `/details/${randomSelected!.id}`, replace: true })">
              <InfoIcon class="w-6 h-6" />
              <span>
                {{ $t('pages.browse.details') }}
              </span>
            </Button>
          </div>
        </div>
      </div>
      <div class="flex flex-col gap-8 p-16">
        <div v-if="user && history.length !== 0" class="flex flex-col gap-4">
          <h2 class="text-2xl font-bold">
            {{ $t('pages.browse.continue') }}
          </h2>
          <Carousel
            tabindex="-1"
            class="relative w-full"
            :opts="{ align: 'start' }"
          >
            <CarouselContent>
              <CarouselItem
                v-for="item in history"
                :key="item.id"
                class="flex-grow p-1 basis-auto"
              >
                <div class="p-1 overflow-hidden rounded-lg">
                  <TMDBImage
                    tabindex="0"
                    :image="item.poster_path"
                    :alt="item.id.toString()"
                    type="poster"
                    size="w185"
                    class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform"
                    @click="() => $router.push({ path: `/details/${item.id}`, replace: true })"
                  />
                </div>
              </CarouselItem>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
          </Carousel>
        </div>
        <div v-if="user && user.watchlist.length !== 0" class="flex flex-col gap-4">
          <h2 class="text-2xl font-bold">
            {{ $t('pages.browse.watchlist') }}
          </h2>
          <Carousel
            tabindex="-1"
            class="relative w-full"
            :opts="{ align: 'start' }"
          >
            <CarouselContent>
              <CarouselItem
                v-for="item in user.watchlist"
                :key="item.id"
                class="flex-grow p-1 basis-auto"
              >
                <div class="p-1 overflow-hidden rounded-lg">
                  <TMDBImage
                    tabindex="0"
                    :image="item.poster_path"
                    :alt="item.id.toString()"
                    type="poster"
                    size="w185"
                    class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform"
                    @click="() => $router.push({ path: `/details/${item.id}`, replace: true })"
                  />
                </div>
              </CarouselItem>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
          </Carousel>
        </div>
        <div v-if="user && user.favorites.length !== 0" class="flex flex-col gap-4">
          <h2 class="text-2xl font-bold">
            {{ $t('pages.browse.favorites') }}
          </h2>
          <Carousel
            tabindex="-1"
            class="relative w-full"
            :opts="{ align: 'start' }"
          >
            <CarouselContent>
              <CarouselItem
                v-for="item in user.favorites"
                :key="item.id"
                class="flex-grow p-1 basis-auto"
              >
                <div class="p-1 overflow-hidden rounded-lg">
                  <TMDBImage
                    tabindex="0"
                    :image="item.poster_path"
                    :alt="item.id.toString()"
                    type="poster"
                    size="w185"
                    class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform"
                    @click="() => $router.push({ path: `/details/${item.id}`, replace: true })"
                  />
                </div>
              </CarouselItem>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
          </Carousel>
        </div>
        <div
          v-for="(store, key) in stores"
          :key="key"
          class="flex flex-col gap-4"
        >
          <h2 class="text-2xl font-bold">{{ key }}</h2>
          <Carousel
            tabindex="-1"
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
                <div class="p-1 overflow-hidden rounded-lg">
                  <TMDBImage
                    tabindex="0"
                    :image="item.poster_path"
                    :alt="item.id.toString()"
                    type="poster"
                    size="w185"
                    class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform"
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
          <h2 class="text-2xl font-bold">{{ genre.name }}</h2>
          <Carousel
            tabindex="-1"
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
                <div class="p-1 overflow-hidden rounded-lg">
                  <TMDBImage
                    tabindex="0"
                    :image="item.poster_path"
                    :alt="item.id.toString()"
                    type="poster"
                    size="w185"
                    class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform"
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

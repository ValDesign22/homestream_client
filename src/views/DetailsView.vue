<script setup lang='ts'>
import { TMDBImage } from '@/components/image';
import { NavBar } from '@/components/navbar';
import { Button } from '@/components/ui/button';
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel';
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Skeleton } from '@/components/ui/skeleton';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
import { useStore } from '@/lib/stores';
import getRecommendations from '@/utils/recommendations';
import { IConfig, IEpisode, IMovie, ITvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { fetch } from '@tauri-apps/plugin-http';
import { PlayerState, usePlayer } from '@vue-youtube/core';
import { useImage } from '@vueuse/core';
import { PlayIcon, SquareCheck, SquarePlus, Star, StarOff } from 'lucide-vue-next';
import { watch } from 'vue';
import { onUnmounted } from 'vue';
import { onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const router = useRouter();
const route = useRoute();

const isMobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);

const store = useStore;

const item = ref<IMovie | ITvShow | null>(null);
const isInFavorites = ref(false);
const isInWatchlist = ref(false);
const videoItem = ref<IMovie | IEpisode | null>(null);
const collection = ref<IMovie[]>([]);
const recommendations = ref<(IMovie | ITvShow)[]>([]);

const currentSeason = ref<string>("0");

const { isLoading } = useImage({ src: item.value?.backdrop_path ?? '' });

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

const loadData = async () => {
  const config = await invoke<IConfig | null>('get_config');
  if (config) {
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

      if (item.value) {
        isInFavorites.value = await store.isInFavorites(item.value);
        isInWatchlist.value = await store.isInWatchlist(item.value);

        if ('collection_id' in item.value) {
          const movie = item.value as IMovie;
          if (!movie.collection_id) collection.value = [];
          else {
            const collectionResponse = await fetch(config.http_server + '/collection?id=' + movie.collection_id, {
              method: 'GET',
              headers: {
                'Content-Type': 'application/json',
              },
            });
            if (collectionResponse.ok) {
              const response = await collectionResponse.json();
              collection.value = response;
            }
          }

          videoItem.value = item.value;
        } else {
          const tvshow = item.value as ITvShow;
          if (tvshow.seasons.length > 0) {
            videoItem.value = tvshow.seasons[0].episodes[0];
          }
        }

        const recommendationsResponse = await getRecommendations(item.value);
        recommendations.value = recommendationsResponse;

        const previewVideoKey = await fetch(`${config.http_server}/preview?id=${item.value.id}`, {
          method: 'GET',
        });
        if (previewVideoKey.ok) {
          const response = await previewVideoKey.text();
          videoKey.value = response || '';
        }
      }
      else router.push({ path: '/browse' });
    }
    else router.push({ path: '/browse' });
  }
  else router.push({ path: '/register', replace: true });
};

onMounted(loadData);

watch(route, loadData);

watch(isInFavorites, async () => {
  if (!item.value) return;
  if (isInFavorites.value) await store.addToFavorites(item.value);
  else await store.removeFromFavorites(item.value);
});

watch(isInWatchlist, async () => {
  if (!item.value) return;
  if (isInWatchlist.value) await store.addToWatchlist(item.value);
  else await store.removeFromWatchlist(item.value);
});

onUnmounted(() => clearInterval(interval));
</script>

<template>
  <NavBar full />
  <div
    ref="videoPlayer"
    class="w-full h-screen object-cover absolute top-0 left-0"
    :class="{ 'z-10': videoPlaying && !videoError, 'z-0': !videoPlaying || videoError }"
  />
  <div v-if="item" class="w-full h-auto">
    <div class="w-full h-screen relative">
      <Skeleton v-if="isLoading" class="w-full h-full" />
      <TMDBImage
        v-else
        :image="item.backdrop_path"
        :alt="item.title"
        type="backdrop"
        size="original"
        class="w-full h-full object-cover relative z-0"
        :class="{ 'z-[11]': !videoPlaying }"
      />
      <div class="absolute z-[12] bottom-0 left-0 w-full h-full flex justify-end flex-col p-12 gap-4 bg-gradient-to-t from-black from-10% to-transparent">
        <TMDBImage
          v-if="item.logo_path"
          :image="item.logo_path"
          :alt="item.id.toString()"
          type="logo"
          size="original"
          class="w-[20vw] h-auto object-cover"
        />
        <h2 v-else class="text-4xl font-bold sm:text-3xl">{{ item.title }}</h2>
        <span class="max-w-2xl" @click="showFullOverview = !showFullOverview">
          {{ showFullOverview ? item.overview : item.overview.split(' ').slice(0, isMobile ? 10 : 50).join(' ') + '...' }}
        </span>
        <div class="flex gap-4 items-center" v-if="videoItem">
          <Button class="flex items-center gap-2" @click="() => $router.push({ path: `/watch/${videoItem!.id}`, replace: true })">
            <PlayIcon class="w-6 h-6" />
            <span>Play</span>
          </Button>
          <span>{{ videoItem.runtime }} minutes</span>
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger as-child>
                <component
                  :is="isInWatchlist ? SquareCheck : SquarePlus"
                  class="w-8 h-8 cursor-pointer"
                  @click="() => isInWatchlist = !isInWatchlist"
                />
              </TooltipTrigger>
              <TooltipContent>
                <span>{{ isInWatchlist ? 'Remove from watchlist' : 'Add to watchlist' }}</span>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger as-child>
                <component
                  :is="isInFavorites ? StarOff : Star"
                  class="w-8 h-8 cursor-pointer"
                  @click="() => isInFavorites = !isInFavorites"
                />
              </TooltipTrigger>
              <TooltipContent>
                <span>{{ isInFavorites ? 'Remove from favorites' : 'Add to favorites' }}</span>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>
      </div>
    </div>
    <div v-if="'seasons' in item" class="flex flex-col gap-4 py-4 px-16 bg-black">
      <Select v-model="currentSeason">
        <SelectTrigger class="max-w-[200px]">
          <SelectValue :placeholder="`Season ${item.seasons[parseInt(currentSeason)].season_number}`" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem v-for="(season, index) in item.seasons" :key="index" :value="index.toString()">
              Season {{ season.season_number }}
            </SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
      <Carousel
        class="relative w-full"
        :opts="{
          align: 'start',
        }"
      >
        <CarouselContent>
          <CarouselItem
            v-for="episode in item.seasons[parseInt(currentSeason)].episodes"
            :key="episode.id"
            class="flex-grow p1 basis-auto"
          >
            <div class="p-1 relative overflow-hidden rounded-lg">
              <TMDBImage
                :image="episode.still_path"
                :alt="episode.title"
                type="still"
                size="w300"
                class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform duration-300"
                @click="() => $router.push({ path: `/watch/${episode.id}`, replace: true })"
              />
              <div class="absolute bottom-0 left-0 w-full h-auto pointer-events-none bg-gradient-to-t from-black from-10% to-transparent p-4">
                <span class="text-sm">Episode {{ episode.episode_number }}</span>
                <span class="text-sm">{{ episode.title }}</span>
              </div>
            </div>
          </CarouselItem>
        </CarouselContent>
        <CarouselPrevious />
        <CarouselNext />
      </Carousel>
    </div>
    <div v-if="collection.length > 0 && collection.length !== 1" class="flex flex-col gap-8 py-4 px-16 bg-black">
      <div v-if="collection.length > 0" class="w-full h-auto flex flex-col p-4">
        <h3 class="text-2xl font-bold">Collection</h3>
        <Carousel
          class="relative w-full"
          :opts="{
            align: 'start',
          }"
        >
          <CarouselContent>
            <CarouselItem
              v-for="movie in collection"
              :key="movie.id"
              class="flex-grow p1 basis-auto"
            >
              <div class="p-1 overflow-hidden rounded-lg">
                <TMDBImage
                  :image="movie.poster_path"
                  :alt="movie.title"
                  type="poster"
                  size="w185"
                  class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform duration-300"
                  @click="() => $router.push({ path: `/details/${movie.id}`, replace: true })"
                />
              </div>
            </CarouselItem>
          </CarouselContent>
          <CarouselPrevious />
          <CarouselNext />
        </Carousel>
      </div>
    </div>
    <div v-if="recommendations.length > 0" class="flex flex-col gap-8 py-4 px-16 bg-black">
      <div class="w-full h-auto flex flex-col gap-4">
        <h3 class="text-2xl font-bold">Recommendations</h3>
        <Carousel
          class="relative w-full"
          :opts="{
            align: 'start',
          }"
        >
          <CarouselContent>
            <CarouselItem
              v-for="movie in recommendations"
              :key="movie.id"
              class="flex-grow p1 basis-auto"
            >
              <div class="p-1 overflow-hidden rounded-lg">
                <TMDBImage
                  :image="movie.poster_path"
                  :alt="movie.title"
                  type="poster"
                  size="w185"
                  class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform duration-300"
                  @click="() => $router.push({ path: `/details/${movie.id}`, replace: true })"
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
</template>
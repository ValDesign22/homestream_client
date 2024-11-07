<script setup lang='ts'>
import { TMDBImage } from '@/components/image';
import { NavBar } from '@/components/navbar';
import { Button } from '@/components/ui/button';
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel';
import { DropdownMenu, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { Progress } from '@/components/ui/progress';
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
import useStore from '@/lib/stores';
import getRecommendations from '@/utils/recommendations';
import { IConfig, IEpisode, IMovie, IProfile, ITvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { EllipsisVertical, PlayIcon, SquareCheck, SquarePlus, Star, StarOff } from 'lucide-vue-next';
import { watch } from 'vue';
import { onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const router = useRouter();
const route = useRoute();

const isMobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);

const store = useStore();

const profile = ref<IProfile | null>(null);

const item = ref<IMovie | ITvShow | null>(null);
const progressItem = ref<{ id: number; progress: number } | null>(null);
const lastWatchedItem = ref<IMovie | IEpisode | null>(null);
const isInFavorites = ref(false);
const isInWatchlist = ref(false);
const videoItem = ref<IMovie | IEpisode | null>(null);
const collection = ref<IMovie[]>([]);
const recommendations = ref<(IMovie | ITvShow)[]>([]);

const currentSeason = ref<string>("0");

const showFullOverview = ref(false);

const calculateProgress = (id: number, duration: number) => {
  if (!profile.value) return 0;
  const item = profile.value.history.find((item) => item.id === id);
  if (!item) return 0;
  const minutes = Math.floor(item.progress / 60);
  return Math.floor((minutes / duration) * 100);
};

const getProgress = (): number => {
  if (!profile.value || !item.value) return 0;
  lastWatchedItem.value = 'seasons' in item.value ? item.value.seasons[0].episodes[0] : item.value; 
  if ('seasons' in item.value) {
    const tvShow = item.value as ITvShow;
    const lastWatchedEpisode = tvShow.seasons.reduce((acc, season) => {
      const lastWatched = season.episodes.reduce((acc, episode) => {
        const progress = profile.value!.history.find((item) => item.id === episode.id);
        if (progress && progress.id > acc.id) {
          lastWatchedItem.value = episode;
          return {
            id: progress.id,
            progress: calculateProgress(episode.id, episode.runtime),
          };
        }
        return acc;
      }, { id: 0, progress: 0 });
      if (lastWatched.id > acc.id) return lastWatched;
      return acc;
    }, { id: 0, progress: 0 });
    progressItem.value = lastWatchedEpisode;
    return lastWatchedEpisode.progress;
  } else {
    const movie = item.value as IMovie;
    const progress = profile.value.history.find((item) => item.id === movie.id);
    if (progress) {
      lastWatchedItem.value = movie;
      return calculateProgress(movie.id, movie.runtime);
    }
  }
  return 0;
};

const loadData = async () => {
  void store.$tauri.start();
  const profileRes = store.profile;
  if (!profileRes) router.push({ path: '/' });
  profile.value = profileRes;

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
      progressItem.value = { id: response.id, progress: getProgress() };

      if (item.value) {
        isInFavorites.value = store.isInFavorites(item.value);
        isInWatchlist.value = store.isInWatchlist(item.value);

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
            if (collectionResponse.ok) collection.value = await collectionResponse.json();
          }

          videoItem.value = item.value;
        } else {
          const tvshow = item.value as ITvShow;
          if (tvshow.seasons.length > 0) videoItem.value = tvshow.seasons[0].episodes[0];
        }

        const recommendationsResponse = await getRecommendations(item.value);
        recommendations.value = recommendationsResponse;
      }
      else router.push({ path: '/browse' });
    }
    else router.push({ path: '/browse' });
  }
  else router.push({ path: '/register' });
};

onMounted(loadData);

watch(route, loadData);

watch(isInFavorites, async () => {
  if (!item.value) return;
  if (isInFavorites.value) store.addToFavorites(item.value);
  else store.removeFromFavorites(item.value);
});

watch(isInWatchlist, async () => {
  if (!item.value) return;
  if (isInWatchlist.value) store.addToWatchlist(item.value);
  else store.removeFromWatchlist(item.value);
});
</script>

<template>
  <div>
    <NavBar full />
    <div v-if="item" class="w-full h-auto">
      <div class="w-full h-screen relative">
        <TMDBImage
          :image="item.backdrop_path"
          :alt="item.title"
          type="backdrop"
          size="original"
          class="w-full h-full object-center object-cover relative z-10"
        />
        <div class="absolute z-[12] bottom-0 left-0 w-full h-full flex justify-end flex-col p-12 gap-4 bg-gradient-to-tr from-background from-10% to-transparent">
          <TMDBImage
            v-if="item.logo_path"
            :image="item.logo_path"
            :alt="item.id.toString()"
            type="logo"
            size="original"
            class="w-[20vw] h-auto object-cover"
          />
          <h2 v-else class="text-4xl font-bold sm:text-3xl">{{ item.title }}</h2>
          <span v-if="item.overview.length" class="max-w-2xl" @click="showFullOverview = !showFullOverview">
            {{ showFullOverview ? item.overview : item.overview.split(' ').slice(0, isMobile ? 10 : 50).join(' ') + '...' }}
          </span>
          <div class="flex gap-4 items-center" v-if="videoItem || lastWatchedItem">
            <Button class="flex items-center gap-2" @click="() => $router.push({ path: `/watch/${lastWatchedItem ? lastWatchedItem.id : videoItem!.id}` })">
              <PlayIcon class="w-6 h-6" />
              <span v-if="lastWatchedItem && progressItem && progressItem.id === item.id && progressItem.progress > 0">{{ $t('pages.details.continue') }}</span>
              <span v-else>{{ $t('pages.details.watch') }}</span>
            </Button>
            <span v-if="lastWatchedItem && progressItem && progressItem.id === item.id && progressItem.progress > 0">{{ $t('pages.details.time_left', { time: Math.floor(lastWatchedItem.runtime - (lastWatchedItem.runtime * progressItem.progress / 100)) }) }}</span>
            <span v-else>{{ $t('pages.details.runtime', { runtime: videoItem?.runtime ?? 0 }) }}</span>
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
                  <span>
                    {{ isInWatchlist ? $t('pages.details.watchlist.remove') : $t('pages.details.watchlist.add') }}
                  </span>
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
                  <span>
                    {{ isInFavorites ? $t('pages.details.favorites.remove') : $t('pages.details.favorites.add') }}
                  </span>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
            <DropdownMenu>
              <DropdownMenuTrigger>
                <EllipsisVertical />
              </DropdownMenuTrigger>
            </DropdownMenu>
          </div>
          <div v-if="videoItem && lastWatchedItem && progressItem" class="flex gap-4 items-center max-w-2xl">
            <Progress
              v-if="progressItem.id === item.id && progressItem.progress > 0"
              class="h-2"
              :modelValue="progressItem.progress"
            />
          </div>
        </div>
      </div>
      <div v-if="'seasons' in item" class="flex flex-col gap-4 py-4 px-16">
        <Select v-model="currentSeason">
          <SelectTrigger class="max-w-[200px]">
             <SelectValue :placeholder="$t('pages.details.season', { season: item.seasons[parseInt(currentSeason)].season_number })" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectItem v-for="(season, index) in item.seasons" :key="index" :value="index.toString()">
                {{ $t('pages.details.season', { season: season.season_number }) }}
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
              class="flex-grow basis-auto"
            >
              <div class="p-1 relative overflow-hidden rounded-lg">
                <TMDBImage
                  :image="episode.still_path"
                  :alt="episode.title"
                  type="still"
                  size="w300"
                  class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform"
                  @click="() => $router.push({ path: `/watch/${episode.id}` })"
                />
                <div class="absolute top-0 left-0 w-full h-full pointer-events-none bg-gradient-to-tr from-background from-10% to-transparent">
                  <div class="absolute bottom-0 left-0 flex flex-col p-2 gap-2 w-full">
                    <p class="text-sm">{{ $t('pages.details.episode', { episode: episode.episode_number }) }}</p>
                    <p class="text-sm">{{ episode.title }}</p>
                    <Progress
                      v-if="profile && profile.history.find((item) => item.id === episode.id)"
                      class="h-2"
                      :modelValue="calculateProgress(episode.id, episode.runtime)"
                    />
                  </div>
                </div>
              </div>
            </CarouselItem>
          </CarouselContent>
          <CarouselPrevious />
          <CarouselNext />
        </Carousel>
      </div>
      <div v-if="collection.length > 0 && collection.length !== 1" class="flex flex-col gap-8 py-4 px-16">
        <div class="w-full h-auto flex flex-col gap-4">
          <h3 class="text-2xl font-bold">{{ $t('pages.details.collection') }}</h3>
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
                class="flex-grow basis-auto"
              >
                <div class="p-1 overflow-hidden rounded-lg">
                  <TMDBImage
                    :image="movie.poster_path"
                    :alt="movie.title"
                    type="poster"
                    size="w185"
                    class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform"
                    @click="() => $router.push({ path: `/details/${movie.id}` })"
                  />
                </div>
              </CarouselItem>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
          </Carousel>
        </div>
      </div>
      <div v-if="recommendations.length > 0" class="flex flex-col gap-8 py-4 px-16">
        <div class="w-full h-auto flex flex-col gap-4">
          <h3 class="text-2xl font-bold">{{ $t('pages.details.recommendations') }}</h3>
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
                class="flex-grow basis-auto"
              >
                <div class="p-1 overflow-hidden rounded-lg">
                  <TMDBImage
                    :image="movie.poster_path"
                    :alt="movie.title"
                    type="poster"
                    size="w185"
                    class="w-full h-auto object-cover rounded-lg cursor-pointer hover:scale-105 transition-transform"
                    @click="() => $router.push({ path: `/details/${movie.id}` })"
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
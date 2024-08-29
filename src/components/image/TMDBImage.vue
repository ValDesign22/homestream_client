<script setup lang="ts">
import { useImage } from '@vueuse/core';
import { computed, HTMLAttributes } from 'vue';
import { Skeleton } from '@/components/ui/skeleton';

type SizeOptions = {
  backdrop: "w300" | "w780" | "w1280" | "original";
  logo: "w45" | "w92" | "w154" | "w185" | "w300" | "w500" | "original";
  poster: "w92" | "w154" | "w185" | "w342" | "w500" | "w780" | "original";
  profile: "w45" | "w185" | "h632" | "original";
  still: "w92" | "w185" | "w300" | "original";
}

type TMDBImageProps<T extends keyof SizeOptions> = {
  type: T;
  size: SizeOptions[T];
  image: string;
  alt: string;
  class?: HTMLAttributes['class'];
};

const props = defineProps<TMDBImageProps<keyof SizeOptions>>();

const imageUrl = computed(() => `https://image.tmdb.org/t/p/${props.size}${props.image}`);
const { isLoading } = useImage({ src: imageUrl.value });
</script>

<template>
  <Skeleton v-if="isLoading" class="w-full h-full rounded-lg" />
  <img
    v-else
    :src="imageUrl"
    :alt="props.alt"
    :class="props.class"
  />
</template>
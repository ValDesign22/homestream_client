<script setup lang="ts">
import { useImage } from '@vueuse/core';
import { HTMLAttributes } from 'vue';
import { Skeleton } from '@/components/ui/skeleton';

interface BackdropProps {
  type: "backdrop";
  size:  "w300" | "w780" | "w1280" | "original";
}

interface LogoProps {
  type: "logo";
  size: "w45" | "w92" | "w154" | "w185" | "w300" | "w500" | "original";
}

interface PosterProps {
  type: "poster";
  size: "w92" | "w154" | "w185" | "w342" | "w500" | "w780" | "original";
}

interface ProfileProps {
  type: "profile";
  size: "w45" | "w185" | "h632" | "original";
}

interface StillProps {
  type: "still";
  size: "w92" | "w185" | "w300" | "original";
}

type TMDBImageProps = (BackdropProps | LogoProps | PosterProps | ProfileProps | StillProps) & {
  image: string;
  alt: string;
};

const props = defineProps<TMDBImageProps & { class?: HTMLAttributes['class'] }>();

const imageUrl = `https://image.tmdb.org/t/p/${props.size}${props.image}`;
const { isLoading } = useImage({ src: imageUrl });
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
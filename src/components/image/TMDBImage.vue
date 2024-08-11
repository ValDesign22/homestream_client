<script setup lang="ts">
import { HTMLAttributes } from 'vue';

interface TypeToValueMap {
  backdrop: "w300" | "w780" | "w1280" | "original";
  logo: "w45" | "w92" | "w154" | "w185" | "w300" | "w500" | "original";
  poster: "w92" | "w154" | "w185" | "w342" | "w500" | "w780" | "original";
  profile: "w45" | "w185" | "h632" | "original";
  still: "w92" | "w185" | "w300" | "original";
}

type TMDBImageProps = {
  [T in keyof TypeToValueMap]: {
    type: T;
    size: TypeToValueMap[T];
  };
}[keyof TypeToValueMap] & {
  image: string;
  alt: string;
}

const props = defineProps<TMDBImageProps & { class?: HTMLAttributes['class'] }>();
</script>

<template>
  <img
    :src="`https://image.tmdb.org/t/p/${props.size}${props.image}`"
    :alt="props.alt"
    :class="props.class"
  />
</template>
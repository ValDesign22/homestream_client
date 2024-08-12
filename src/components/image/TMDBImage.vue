<script setup lang="ts">
import { HTMLAttributes } from 'vue';

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
</script>

<template>
  <img
    :src="`https://image.tmdb.org/t/p/${props.size}${props.image}`"
    :alt="props.alt"
    :class="props.class"
  />
</template>
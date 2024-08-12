<script setup lang="ts">
import { useWindowScroll } from '@vueuse/core';

const { y } = useWindowScroll({ behavior: 'smooth' });
</script>

<template>
  <nav
    class="flex justify-between items-center p-4 fixed top-0 left-0 w-full z-10 text-white transition-all duration-300"
    :class="{
      'bg-gradient-to-b from-black from-5% to-transparent': y === 0,
      'bg-gradient-to-b from-black to-transparent backdrop-blur': y > 0 }"
  >
    <a href="/" class="text-2xl font-bold">HomeStream</a>
    <div class="flex items-center space-x-4">
      <a href="/browse">Browse</a>
      <a href="/search">Search</a>
    </div>
  </nav>
</template>
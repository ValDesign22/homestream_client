<script setup lang="ts">
import { useWindowScroll } from '@vueuse/core';
import { RouterLink } from 'vue-router';
import { Input } from '@/components/ui/input';
import { Search } from 'lucide-vue-next';

const { y } = useWindowScroll({ behavior: 'smooth' });
</script>

<template>
  <nav
    class="flex justify-between items-center p-4 fixed top-0 left-0 w-full z-20 transition-all duration-300"
    :class="{
      'bg-gradient-to-b from-black from-5% to-transparent': y === 0,
      'bg-gradient-to-b from-black to-transparent backdrop-blur': y > 0 }"
  >
    <RouterLink to="/" class="text-2xl font-bold">HomeStream</RouterLink>
    <div class="flex items-center space-x-4">
      <RouterLink to="/browse">Browse</RouterLink>
       <div class="relative w-full max-w-xs items-center">
        <Input id="search" type="text" placeholder="Search..." class="pl-10" />
        <span class="absolute start-0 inset-y-0 flex items-center justify-center px-2">
          <Search class="size-6 text-muted-foreground" />
        </span>
      </div>
    </div>
  </nav>
</template>
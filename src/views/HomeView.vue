<script setup lang="ts">
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import { useStore } from '@/lib/stores';
import { IConfig, IProfile } from '@/utils/types';
import { getVersion } from '@tauri-apps/api/app';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { UserRound } from 'lucide-vue-next';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const store = useStore();

const version = ref<string | null>(null);
const profiles = ref<IProfile[]>([]);

const selectProfile = (profile: IProfile) => {
  store.setProfile(profile);
  router.push({ path: '/browse' });
};

onMounted(async () => {
  const config = await invoke<IConfig | null>("get_config");
  if (!config) return router.push({ path: "/register", replace: true });

  version.value = await getVersion();

  const profilesRes = await fetch(config.http_server + '/profiles', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json'
    }
  });
  if (!profilesRes.ok) profiles.value = [];
  else profiles.value = await profilesRes.json();
});
</script>

<template>
  <div class="absolute top-0 left-1/2 transform -translate-x-1/2 p-4">
    <h1 class="text-4xl font-bold">HomeStream</h1>
  </div>
  <div class="flex flex-col items-center justify-center h-screen gap-4">
    <h2 class="text-2xl font-bold">Select a profile</h2>
    <div class="flex flex-col items-center gap-2">
      <span v-if="profiles.length === 0">No profiles found</span>
      <div v-else class="flex gap-2">
        <div v-for="profile in profiles" :key="profile.id" class="flex flex-col items-center gap-2 cursor-pointer" @click="() => selectProfile(profile)">
          <Avatar size="lg" class="relative">
            <AvatarImage :src="`https://avatar.vercel.sh/${profile.name}?size=128`" />
            <UserRound class="absolute inset-0 w-full h-full" />
            <AvatarFallback>{{ profile.name[0] }}</AvatarFallback>
          </Avatar>
          <span>{{ profile.name }}</span>
        </div>
      </div>
    </div>
    <div class="flex flex-col items-center gap-4">
      <Button variant="outline" @click="() => $router.push({ path: '/profiles' })">Manage profiles</Button>
    </div>
  </div>
  <div class="absolute bottom-0 right-0 p-4 ">
    <span v-if="version" class="text-sm text-gray-500">HomeStream v{{ version }}</span>
  </div>
</template>
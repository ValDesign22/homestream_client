<script setup lang="ts">
import { NavBar } from '@/components/navbar';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import useStore from '@/lib/stores';
import { Settings } from 'lucide-vue-next';
import { IConfig, IProfile } from '@/utils/types';
import { getVersion } from '@tauri-apps/api/app';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { SettingsMenu } from '@/components/settings';

const router = useRouter();
const store = useStore();

const version = ref<string | null>(null);
const profiles = ref<IProfile[]>([]);
const settingsOpened = ref(false);

const selectProfile = async (selectedProfile: IProfile) => {
  store.setProfile(selectedProfile);
  router.push({ path: '/browse' });
};

const toggleSettings = () => {
  settingsOpened.value = !settingsOpened.value;
};

onMounted(async () => {
  void store.$tauri.start();
  const config = await invoke<IConfig | null>("get_config");
  if (!config) return router.push({ path: "/register" });

  version.value = await getVersion();

  const profilesRes = await fetch(config.http_server + '/profiles', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json'
    }
  });
  console.log(profilesRes);
  if (!profilesRes.ok) profiles.value = [];
  else profiles.value = await profilesRes.json();
});
</script>

<template>
  <div>
    <NavBar :toggleSettings="toggleSettings" />
    <div class="flex flex-col items-center justify-center h-screen gap-4">
      <h2 class="text-2xl font-bold">
        {{ $t('pages.home.selectProfile') }}
      </h2>
      <div class="flex flex-col items-center gap-2">
        <span v-if="profiles.length === 0">
          {{ $t('pages.home.noProfiles') }}
        </span>
        <div v-else class="flex justify-center flex-wrap gap-2">
          <div v-for="profile in profiles" :key="profile.id" class="flex flex-col items-center gap-2 p-4 cursor-pointer rounded-lg hover:shadow-lg hover:bg-accent transition-colors" @click="() => selectProfile(profile)">
            <Avatar size="lg" shape="square">
              <AvatarImage :src="`https://avatar.vercel.sh/${profile.name}?size=128`" />
              <AvatarFallback>{{ profile.name[0] }}</AvatarFallback>
            </Avatar>
            <span>{{ profile.name }}</span>
          </div>
        </div>
      </div>
      <div class="flex flex-col items-center gap-4">
        <Button variant="outline" @click="() => $router.push({ path: '/profiles' })">
          {{ $t('pages.home.manageProfiles') }}
        </Button>
      </div>
    </div>
    <div class="absolute bottom-0 right-0 flex items-center justify-between w-full p-4">
      <Settings class="cursor-pointer w-6 h-6" @click="toggleSettings" />
      <span v-if="version" class="text-sm text-gray-500">
        {{ $t('app.footerVersion', { version }) }}
      </span>
    </div>
    <SettingsMenu :opened="settingsOpened" :toggleSettings="toggleSettings" />
  </div>
</template>
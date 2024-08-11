<script setup lang="ts">
import { TreeSelector } from '@/components/tree';
import { Config, RemoteFolder } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { ref } from 'vue';
import { onBeforeMount } from 'vue';

const folders = ref<RemoteFolder[]>([]);
const selectedItem = ref<RemoteFolder | null>(null);

const selectItem = (item: RemoteFolder | null) => {
  selectedItem.value = item;
};

async function fetchFolders(http_server: string) {
  if (!http_server) return;

  const response = await fetch(http_server + '/folders', {
    method: 'GET',
  });

  if (!response.ok) console.error('An error occurred while fetching the folders');
  else folders.value = await response.json();
}

onBeforeMount(async () => {
  const config = await invoke<Config | null>("get_config");
  if (config) fetchFolders(config.http_server);
});
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <h1 class="text-4xl font-bold">
      Browse View
    </h1>

    <TreeSelector
      :data="folders"
      :selectedItem="selectedItem"
      :selectItem="selectItem"
      />
  </div>
</template>

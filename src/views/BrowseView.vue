<script setup lang="ts">
import { TreeSelector } from '@/components/tree';
import { Button } from '@/components/ui/button';
import { Config, RemoteFolder } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { ref } from 'vue';
import { onBeforeMount } from 'vue';

const folders = ref<RemoteFolder[]>([]);
const selectedItem = ref<RemoteFolder | null>(null);
const selectorOpened = ref(false);

const selectItem = (item: RemoteFolder | null) => {
  selectedItem.value = item;
};

const toggleSelector = () => {
  selectorOpened.value = !selectorOpened.value;
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

    <div class="flex flex-col items-center">
      <span class="text-sm">Select a folder:</span>
      <span v-if="selectedItem" class="text-sm">{{ selectedItem.path }}</span>
      <Button
        @click="toggleSelector"
        class="bg-accent text-accent-foreground hover:bg-accent-dark"
      >
        Select
      </Button>
    </div>

    <TreeSelector
      :open="selectorOpened"
      :data="folders"
      :selectedItem="selectedItem"
      :selectItem="selectItem"
      :toggle="toggleSelector"
      />
  </div>
</template>

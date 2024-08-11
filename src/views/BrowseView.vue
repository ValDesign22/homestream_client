<script setup lang="ts">
import { TreeSelector } from '@/components/tree';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Config, Folder, RemoteFolder } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { Trash2 } from 'lucide-vue-next';
import { ref } from 'vue';
import { onBeforeMount } from 'vue';

const serverFolders = ref<RemoteFolder[]>([]);
const selectedItem = ref<RemoteFolder | null>(null);
const folders = ref<Folder[]>([]);
const selectors = ref<boolean[]>([]);

const selectItem = (item: RemoteFolder | null, index: number) => {
  selectedItem.value = item;
  folders.value[index].path = item ? item.path : '';
};

const toggleSelector = (index: number) => {
  selectors.value = selectors.value.map((_, i) => i === index ? !selectors.value[i] : false);
};

async function fetchFolders(http_server: string) {
  if (!http_server) return;

  const response = await fetch(http_server + '/folders', {
    method: 'GET',
  });

  if (!response.ok) console.error('An error occurred while fetching the folders');
  else serverFolders.value = await response.json();
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

    <div class="flex flex-col gap-4 w-full max-w-[460px] p-4 border rounded-md">
      <div v-if="!folders.length" class="flex items-center gap-4">
        <span class="text-lg font-bold">No folders added</span>
      </div>
      <div v-for="(folder, index) in folders" :key="index" class="flex items-center justify-between gap-4">
        <Select>
          <SelectTrigger class="w-auto">
            <SelectValue placeholder="Select a media type" />
          </SelectTrigger>
          <SelectContent class="w-auto">
            <SelectGroup class="w-auto">
              <SelectLabel>Media Type</SelectLabel>
              <SelectItem value="0">Movies</SelectItem>
              <SelectItem value="1">TV Shows</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
        <Input
          v-model="folders[index].path"
          class="text-sm truncate w-1/2"
          @click="toggleSelector(index)"
        >
          {{ folder.path.length ? folder.path : 'Select a folder' }}
        </Input>
        <Button variant="destructive" @click="() => {
          folders = folders.filter((_, i) => i !== index)
          selectors = selectors.filter((_, i) => i !== index)
        }">
          <Trash2 />
        </Button>

        <TreeSelector
          :open="selectors[index]"
          :index="index"
          :data="serverFolders"
          :selectedItem="selectedItem"
          :selectItem="selectItem"
          :toggle="(index) => toggleSelector(index)"
          />
      </div>
      <div class="flex items-center gap-4">
        <Button @click="() => {
          folders = [...folders, { id: folders.length, name: '', path: '', media_type: 0 }]
          selectors = [...selectors, false]
        }">
          Add Folder
        </Button>
      </div>
    </div>
  </div>
</template>

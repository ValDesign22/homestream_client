<script setup lang="ts">
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { HTMLAttributes, ref } from 'vue';
import { RemoteFolder } from '@/utils/types';
import TreeViewer from './TreeViewer.vue';

interface TreeSelectorProps {
  data: RemoteFolder[];
  selectedItem: RemoteFolder | null;
  selectItem: (item: RemoteFolder | null) => void;
}

const props = defineProps<TreeSelectorProps & { class?: HTMLAttributes['class'] }>();

const input = ref<string>('');

const searchByPath = (data: RemoteFolder[], path: string): RemoteFolder | null => {
  const stack = [...data];

  while (stack.length) {
    const item = stack.pop();
    if (!item) continue;

    if (item.path === path) return item;
    if (item.children) stack.push(...item.children);
  }

  return null;
};

const searchItem = (event: InputEvent) => {
  const value = (event.target as HTMLInputElement).value;
  if (!value) return;

  const item = searchByPath(props.data, value);
  props.selectItem(item);
};
</script>

<template>
  <div class="flex flex-col justify-center gap-4">
    <h1 class="text-lg font-bold">
      Select a folder
    </h1>
    <TreeViewer
      :data="props.data"
      :selectedItem="props.selectedItem"
      :selectItem="props.selectItem"
    />
    <div class="flex items-center gap-2">
      <Input
        v-model="input"
        @input="searchItem"
        />
      <Button @click="() => console.log(selectedItem)">Select</Button>
    </div>
  </div>
</template>
<script setup lang="ts">
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { HTMLAttributes, ref } from 'vue';
import { RemoteFolder } from '@/utils/types';
import TreeViewer from './TreeViewer.vue';

interface TreeSelectorProps {
  open: boolean;
  index: number;
  data: RemoteFolder[];
  selectedItem: RemoteFolder | null;
  selectItem: (item: RemoteFolder | null, index: number) => void;
  toggle: (index: number) => void;
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

const selectItem = (item: RemoteFolder | null) => {
  props.selectItem(item, props.index);
  input.value = item ? item.path : '';
};

const searchItem = (event: InputEvent) => {
  const value = (event.target as HTMLInputElement).value;
  if (!value) return;

  const item = searchByPath(props.data, value);
  props.selectItem(item, props.index);
};
</script>

<template>
  <div
    class="flex flex-col justify-center p-4 gap-4 border rounded-md fixed top-1/2 left-1/2 transform -translate-x-1/2 bg-background dark:bg-background-dark shadow-lg duration-300 transition-transform"
    :class="props.open ?
      'visible z-10 -translate-y-1/2' :
      'invisible z-[-1] -translate-y-1/3 pointer-events-none'"
  >
    <h1 class="text-lg font-bold">
      Select a folder
    </h1>
    <TreeViewer
      :data="props.data"
      :selectedItem="props.selectedItem"
      :selectItem="selectItem"
    />
    <div class="flex items-center gap-2">
      <Input
        v-model="input"
        @input="searchItem"
        />
      <Button @click="props.toggle">Select</Button>
    </div>
  </div>
</template>
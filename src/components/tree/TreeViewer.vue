<script setup lang="ts">
import { RemoteFolder } from '@/utils/types';
import { ScrollArea } from '@/components/ui/scroll-area';
import { HTMLAttributes, ref } from 'vue';
import TreeItem from './TreeItem.vue';
import { useResizeObserver } from '@vueuse/core';

interface TreeViewerProps {
  data: RemoteFolder[];
}

const opened_ids = ref<string[]>([]);
const selectedItem = ref<string | null>(null);

const treeElem = ref<HTMLDivElement | null>(null);
const widthRef = ref(0);
const heightRef = ref(0);

const toggleFolder = (id: string) => {
  if (opened_ids.value.includes(id)) opened_ids.value = opened_ids.value.filter((i) => i !== id);
  else opened_ids.value = [...opened_ids.value, id];
};

const selectItem = (id: string) => selectedItem.value = id;

useResizeObserver(treeElem, (entries) => {
  const entry = entries[0];
  const { width, height } = entry.contentRect;
  if (widthRef.value && heightRef.value) {
    widthRef.value = width;
    heightRef.value = height;
  }
});

const props = defineProps<TreeViewerProps & { class?: HTMLAttributes['class'] }>();
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <h1 class="text-4xl font-bold">
      Tree Viewer
    </h1>
    <div ref="treeElem" class="overflow-hidden flex-shrink-0 w-[460px] h-[460px] border-[1px]">
      <ScrollArea :class="`w-full h-full`">
        <div class="relative p-2">
          <TreeItem
            :data="props.data"
            :opened_ids="opened_ids"
            :selectedItem="selectedItem"
            :toggleFolder="toggleFolder"
            :selectItem="selectItem"
          />
        </div>
      </ScrollArea>
    </div>
  </div>
</template>
<script setup lang="ts">
import { RemoteFolder } from '@/utils/types';
import { Folder } from 'lucide-vue-next';
import { HTMLAttributes } from 'vue';

interface TreeLeaf {
  item: RemoteFolder;
  selectedItem: RemoteFolder | null;
  selectItem: (item: RemoteFolder) => void;
}

const props = defineProps<TreeLeaf & { class?: HTMLAttributes['class'] }>();
</script>

<template>
  <div
    class="flex items-center py-4 px-2 cursor-pointer border-b \
    hover:before:opacity-100 before:absolute before:left-0 before:right-1 before:w-full before:opacity-0 before:bg-muted/80 before:h-[1.75rem] before:-z-10"
    :class="{ 'before:opacity-100 before:bg-accent text-accent-foreground before:border-l-2 before:border-l-accent-foreground/50 dark:before:border-0': props.selectedItem && props.selectedItem.id === props.item.id }"
    @click="props.selectItem(props.item)"
  >
    <Folder class="w-4 h-4 inline-block shrink-0 mr-2 text-accent-foreground/50" />
    <span class="flex-grow text-sm truncate">
      {{ props.item.name }}
    </span>
  </div>
</template>
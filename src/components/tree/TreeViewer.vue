<script setup lang="ts">
import { RemoteFolder } from '@/utils/types';
import { ScrollArea } from '@/components/ui/scroll-area';
import { HTMLAttributes, ref } from 'vue';
import TreeNode from './TreeNode.vue';

interface TreeViewerProps {
  data: RemoteFolder[];
  selectedItem: RemoteFolder | null;
  selectItem: (item: RemoteFolder | null) => void;
}

const props = defineProps<TreeViewerProps & { class?: HTMLAttributes['class'] }>();

const opened_ids = ref<string[]>([]);

const toggleFolder = (id: string) => {
  if (opened_ids.value.includes(id)) opened_ids.value = opened_ids.value.filter((i) => i !== id);
  else opened_ids.value = [...opened_ids.value, id];
};
</script>

<template>
  <div class="overflow-hidden flex-shrink-0 w-[460px] h-[360px] border rounded-md">
    <ScrollArea :class="`w-full h-full`">
      <div class="relative px-2">
        <TreeNode
          :data="props.data"
          :opened_ids="opened_ids"
          :selectedItem="props.selectedItem"
          :toggleFolder="toggleFolder"
          :selectItem="props.selectItem"
        />
      </div>
    </ScrollArea>
  </div>
</template>
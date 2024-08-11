<script setup lang="ts">
import { RemoteFolder } from '@/utils/types';
import { FolderClosed, FolderOpen } from 'lucide-vue-next';
import { HTMLAttributes } from 'vue';
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@/components/ui/accordion';
import TreeLeaf from './TreeLeaf.vue';

interface TreeItemProps {
  data: RemoteFolder[] | RemoteFolder;
  opened_ids: string[];
  selectedItem: RemoteFolder | null;
  toggleFolder: (id: string) => void;
  selectItem: (item: RemoteFolder) => void;
}

const props = defineProps<TreeItemProps & { class?: HTMLAttributes['class'] }>();
</script>

<template>
  <div role="tree" :class="props.class">
    <ul>
      <!-- eslint-disable-next-line vue/no-use-v-if-with-v-for -->
      <li v-if="props.data instanceof Array" v-for="item in props.data as RemoteFolder[]" :key="String(item.id)">
        <Accordion v-if="item.children" type="multiple" :defaultValue="opened_ids">
          <AccordionItem :value="String(item.id)">
            <AccordionTrigger
              class="px-2 hover:before:opacity-100 before:absolute before:left-0 before:w-full before:opacity-0 before:bg-muted/80 before:h-[1.75rem] before:-z-10"
              :class="{ 'before:opacity-100 before:bg-accent text-accent-foreground before:border-l-2 before:border-l-accent-foreground/50 dark:before:border-0': props.selectedItem && props.selectedItem.id === item.id }"
              :onClick="() => {
                props.toggleFolder(String(item.id))
                props.selectItem(item)
              }"
            >
              <span class="text-sm truncate">
                <component :is="opened_ids.includes(String(item.id)) ? FolderOpen : FolderClosed" class="w-4 h-4 inline-block shrink-0 mr-2 text-accent-foreground/50" />
                {{ item.name }}
              </span>
            </AccordionTrigger>
            <AccordionContent class="pl-6">
              <TreeItem
                :data="item.children"
                :opened_ids="opened_ids"
                :selectedItem="props.selectedItem"
                :toggleFolder="props.toggleFolder"
                :selectItem="props.selectItem"
              />
            </AccordionContent>
          </AccordionItem>
        </Accordion>
        <TreeLeaf
          v-else
          :item="item"
          :selectedItem="props.selectedItem"
          :selectItem="props.selectItem"
        />
      </li>
      <li v-else>
        <TreeLeaf
          :item="props.data"
          :selectedItem="props.selectedItem"
          :selectItem="props.selectItem"
        />
      </li>
    </ul>
  </div>
</template>
<script setup lang="ts">
import { useFocus, useGamepad, useWindowScroll } from '@vueuse/core';
import { RouterLink, useRoute, useRouter } from 'vue-router';
import { Input } from '@/components/ui/input';
import { Settings, Search, Users } from 'lucide-vue-next';
import { computed, HTMLAttributes, onMounted, onUnmounted, ref, watch } from 'vue';
import { useStore } from '@/lib/stores';
import { IProfile } from '@/utils/types';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

interface NavBarProps {
  full?: boolean;
}

const props = defineProps<NavBarProps & { class?: HTMLAttributes['class'] }>();

const router = useRouter();
const route = useRoute();
const store = useStore;
const { y } = useWindowScroll({ behavior: 'smooth' });

const settingsOpened = ref(false);

const user = ref<IProfile | null>(null);
const searchInput = ref<HTMLInputElement | null>(null);
const { focused } = useFocus(searchInput);
const searchContent = ref<string>(route.query.q as string || '');

const { isSupported, gamepads } = useGamepad();
const gamepad = computed(() => gamepads.value.find(g => g.mapping === 'standard'));

const gamepadInterval = setInterval(() => {
  if (!isSupported.value) return;
  if (!gamepad.value) return;
  if (gamepad.value.buttons[1].pressed) router.push({ path: '/browse' });
  if (!searchInput.value) return;
  if (gamepad.value.buttons[3].pressed) focused.value = true;
}, 100);

watch(focused, (focused) => {
  if (!searchInput.value || !searchInput.value.focus) return;
  if (focused) searchInput.value.focus();
  else searchInput.value.blur();
});

watch(searchContent, () => {
  if (route.path === '/search') router.push({ query: { q: searchContent.value } });
  else router.push({ path: '/search', query: { q: searchContent.value } });
});

onMounted(async () => {
  user.value = await store.getProfile();
  if (!user.value) return router.push({ path: '/' });
});

onUnmounted(() => clearInterval(gamepadInterval));
</script>

<template>
  <nav
    class="flex justify-between items-center p-4 fixed top-0 left-0 w-full z-20 transition-all duration-300 bg-gradient-to-b from-black"
    :class="{
      'from-5% to-transparent': y === 0,
      'to-transparent backdrop-blur': y > 0
    }"
  >
    <RouterLink
      :to="user ? '/browse' : '/'"
      class="text-2xl font-bold"
      tabindex="-1"
    >
      HomeStream
    </RouterLink>
    <div v-if="props.full" class="flex items-center space-x-4">
       <div class="relative max-w-32 items-center">
        <Input ref="searchInput" v-model="searchContent" type="text" placeholder="Search..." class="pl-10" tabindex="0" />
        <span class="absolute start-0 inset-y-0 flex items-center justify-center px-2">
          <Search class="size-6 text-muted-foreground" />
        </span>
      </div>
      <DropdownMenu :modal="false">
        <DropdownMenuTrigger as-child tabindex="0">
          <Button variant="outline" class="px-0">
            <Avatar v-if="user" size="lg" shape="square" class="relative w-10 h-10 cursor-pointer">
              <AvatarImage :src="`https://avatar.vercel.sh/${user.name}?size=128`" />
              <AvatarFallback>{{ user.name }}</AvatarFallback>
            </Avatar>
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent class="w-56">
          <DropdownMenuLabel>Profile</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuItem class="cursor-pointer" @click="$router.push({ path: '/' })">
              <Users class="mr-2 h-4 w-4" />
              <span>Switch profile</span>
            </DropdownMenuItem>
          </DropdownMenuGroup>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuItem class="cursor-pointer" @click="settingsOpened = true">
              <Settings class="mr-2 h-4 w-4" />
              <span>Settings</span>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  </nav>
  <Dialog v-model:open="settingsOpened">
    <DialogContent class="min-w-[80vw] max-w-[80vw] max-h-[80vh]">
      <DialogHeader class="flex items-center justify-between">
        <DialogTitle>Settings</DialogTitle>
      </DialogHeader>
      <Tabs default-value="general" class="w-full">
        <TabsList class="w-full">
          <TabsTrigger value="general" class="w-full">
            General
          </TabsTrigger>
          <TabsTrigger value="server" class="w-full">
            Server
          </TabsTrigger>
        </TabsList>
        <TabsContent value="general">
          <div class="p-4">
            <h2 class="text-xl font-bold">General</h2>
          </div>
        </TabsContent>
        <TabsContent value="server">
          <div class="p-4">
            <h2 class="text-xl font-bold">Server</h2>
          </div>
        </TabsContent>
      </Tabs>
    </DialogContent>
  </Dialog>
</template>
<script setup lang="ts">
import { useFocus, useGamepad, useWindowScroll } from '@vueuse/core';
import { RouterLink, useRoute, useRouter } from 'vue-router';
import { Input } from '@/components/ui/input';
import { Settings, Search, Users } from 'lucide-vue-next';
import { computed, HTMLAttributes, onMounted, onUnmounted, ref, watch } from 'vue';
import { useStore } from '@/lib/stores';
import { IConfig, IProfile } from '@/utils/types';
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
import { Dialog, DialogContent, DialogTitle } from '@/components/ui/dialog';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { fetch } from '@tauri-apps/plugin-http';
import { invoke } from '@tauri-apps/api/core';
import { VisuallyHidden } from 'radix-vue';

interface NavBarProps {
  full?: boolean;
}

const props = defineProps<NavBarProps & { class?: HTMLAttributes['class'] }>();

const router = useRouter();
const route = useRoute();
const config = ref<IConfig | null>(null);
const store = useStore;
const { y } = useWindowScroll({ behavior: 'smooth' });

const settingsOpened = ref(false);

const serverVersion = ref<{ updateAvailable: boolean, latestVersion: string } | null>(null);

const getServerVersion = async () => {
  if (!config.value) return;

  const response = await fetch(`${config.value.http_server}/update`);
  if (response.ok) serverVersion.value = await response.json();
};

const updateServer = async () => {
  if (!config.value) return;

  await fetch(`${config.value.http_server}/update`, {
    method: 'POST',
  });

  getServerVersion();
};

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
  const configRes = await invoke<IConfig | null>("get_config");
  if (configRes) {
    config.value = configRes;
    getServerVersion();
  }

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
      {{ $t('app.name') }}
    </RouterLink>
    <div v-if="props.full" class="flex items-center space-x-4">
       <div class="relative max-w-32 items-center">
        <Input
          ref="searchInput"
          v-model="searchContent"
          type="text"
          :placeholder="$t('components.navbar.search')"
          class="pl-10"
          tabindex="0"
        />
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
          <DropdownMenuLabel>
            {{ $t('components.navbar.profile') }}
          </DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuItem class="cursor-pointer" @click="$router.push({ path: '/' })">
              <Users class="mr-2 h-4 w-4" />
              <span>{{ $t('components.navbar.switchProfile') }}</span>
            </DropdownMenuItem>
          </DropdownMenuGroup>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuItem class="cursor-pointer" @click="settingsOpened = true">
              <Settings class="mr-2 h-4 w-4" />
              <span>{{ $t('components.navbar.settings') }}</span>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  </nav>
  <Dialog v-model:open="settingsOpened">
    <DialogContent class="min-w-[80vw] max-w-[80vw] h-[80vh]">
      <VisuallyHidden>
        <DialogTitle>{{ $t('settings.title') }}</DialogTitle>
      </VisuallyHidden>
      <Tabs default-value="general" class="w-full p-4">
        <TabsList class="w-full">
          <TabsTrigger value="general" class="w-full">
            {{ $t('settings.tabs.general') }}
          </TabsTrigger>
          <TabsTrigger value="server" class="w-full">
            {{ $t('settings.tabs.server') }}
          </TabsTrigger>
        </TabsList>
        <TabsContent value="general">
          <div class="p-4">
            <h2 class="text-xl font-bold">{{ $t('settings.general.title') }}</h2>
          </div>
        </TabsContent>
        <TabsContent value="server">
          <div class="p-4">
            <h2 class="text-xl font-bold">{{ $t('settings.server.title') }}</h2>
            <div v-if="serverVersion" class="flex items-center space-x-4">
              <span>{{ $t('settings.server.serverVersion') }}</span>
              <span>{{ serverVersion.latestVersion }}</span>
              <Button v-if="serverVersion.updateAvailable" @click="updateServer">
                {{ $t('settings.server.update') }}
              </Button>
            </div>
          </div>
        </TabsContent>
      </Tabs>
    </DialogContent>
  </Dialog>
</template>
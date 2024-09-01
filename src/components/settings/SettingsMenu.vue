<script setup lang="ts">
import { Dialog, DialogClose, DialogContent, DialogTitle } from '@/components/ui/dialog';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Button } from '@/components/ui/button';
import { VisuallyHidden } from 'radix-vue';
import { onMounted, ref, watch } from 'vue';
import { fetch } from '@tauri-apps/plugin-http';
import { invoke } from '@tauri-apps/api/core';
import { Color, colors, IConfig } from '@/utils/types';
import { X } from 'lucide-vue-next'
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { useI18n } from 'vue-i18n';
import { useStore } from '@/lib/stores';

interface SettingsMenuProps {
  opened: boolean;
  toggleSettings: () => void;
}

const props = defineProps<SettingsMenuProps>();

const i18n = useI18n();
const store = useStore;

const theme = ref<Color>('slate');

const config = ref<IConfig | null>(null);
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

watch(i18n.locale, async (value) => {
  await store.setLocale(value);
});

watch(theme, async (value) => {
  if (value) {
    document.documentElement.classList.remove(...colors.map((c) => `theme-${c}`));
    document.documentElement.classList.add(`theme-${value}`);
    return await store.setTheme(value)
  };
});

onMounted(async () => {
  const configRes = await invoke<IConfig | null>("get_config");
  if (configRes) {
    config.value = configRes;
    getServerVersion();
  }

  theme.value = await store.getTheme();
});
</script>

<template>
  <Dialog v-model:open="props.opened">
    <DialogContent
      class="min-w-[80vw] max-w-[80vw] h-[80vh]"
      @pointer-down-outside="props.toggleSettings" 
      @escape-key-down="props.toggleSettings"
    >
      <VisuallyHidden>
        <DialogTitle>{{ $t('settings.title') }}</DialogTitle>
      </VisuallyHidden>
      <Tabs default-value="general" class="w-full p-4">
        <TabsList class="w-full">
          <TabsTrigger value="general" class="w-full">
            {{ $t('settings.tabs.general') }}
          </TabsTrigger>
          <TabsTrigger value="appearance" class="w-full">
            {{ $t('settings.tabs.appearance') }}
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
        <TabsContent value="appearance">
          <div class="flex flex-col p-4 gap-4">
            <h2 class="text-xl font-bold">{{ $t('settings.appearance.title') }}</h2>
            <div class="flex items-center space-x-4">
              <span>{{ $t('settings.appearance.language') }}</span>
              <Select :defaultValue="$i18n.locale" v-model="$i18n.locale">
                <SelectTrigger class="w-[180px]">
                  <SelectValue :placeholder="$t('settings.appearance.selectLanguage')" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem v-for="lang in $i18n.availableLocales" :key="lang" :value="lang">
                      {{ $t(`languages.${lang}`) }}
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
            </div>
            <div class="flex items-center space-x-4">
              <span>{{ $t('settings.appearance.theme') }}</span>
              <Select :defaultValue="theme" v-model="theme">
                <SelectTrigger class="w-[180px]">
                  <SelectValue :placeholder="$t('settings.appearance.selectTheme')" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem v-for="color in colors" :key="color" :value="color">
                      {{ $t(`themes.${color}`) }}
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
            </div>
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
      <DialogClose
        class="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground"
        @click="props.toggleSettings"
      >
        <X class="w-4 h-4" />
        <span class="sr-only">Close</span>
      </DialogClose>
    </DialogContent>
  </Dialog>
</template>
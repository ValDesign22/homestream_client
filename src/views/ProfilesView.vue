<script setup lang="ts">
import { ERole, IConfig, IProfile } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { Button } from '@/components/ui/button';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { toTypedSchema } from '@vee-validate/zod';
import zod from 'zod';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { GenericObject } from 'vee-validate';
import { NavBar } from '@/components/navbar';
import { Trash2, X } from 'lucide-vue-next';
import { getVersion } from '@tauri-apps/api/app';

const router = useRouter();

const version = ref<string | null>(null);
const profiles = ref<IProfile[]>([]);
const hovering = ref<number | null>(null);

const formSchema = toTypedSchema(
  zod.object({
    name: zod.string().min(3).max(32),
    password: zod.string().min(8).max(32).optional(),
    role: zod.enum(["admin", "user"])
  })
);

async function onSubmit(values: GenericObject) {
  const config = await invoke<IConfig | null>("get_config");
  if (!config) return router.push({ path: "/register", replace: true });

  const newProfile: IProfile = {
    id: Math.floor(Date.now() + Math.random() * 1000),
    name: values.name as string,
    password: values.password as string,
    role: (values.role as string) === "admin" ? ERole.Admin : ERole.User,
    favorites: [],
    history: [],
    watchlist: []
  };

  const res = await fetch(config.http_server + '/profiles', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(newProfile)
  });

  if (res.ok) profiles.value.push(newProfile);
  else console.error("Failed to add profile");
}

async function deleteProfile(id: number) {
  const config = await invoke<IConfig | null>("get_config");
  if (!config) return router.push({ path: "/register", replace: true });

  const res = await fetch(`${config.http_server}/profiles?id=${id}`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json'
    }
  });

  if (res.ok) profiles.value = profiles.value.filter(profile => profile.id !== id);
  else console.error("Failed to delete profile", await res.json());
}

onMounted(async () => {
  const config = await invoke<IConfig | null>("get_config");
  if (!config) return router.push({ path: "/register", replace: true });

  version.value = await getVersion();

  const profilesRes = await fetch(config.http_server + '/profiles', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json'
    }
  });
  if (!profilesRes.ok) profiles.value = [];
  else profiles.value = await profilesRes.json();
});
</script>

<template>
  <NavBar />
  <div class="flex flex-col items-center justify-center h-screen gap-4">
    <h1 class="text-4xl font-bold">
      {{ $t('pages.profiles.title') }}
    </h1>

    <div class="flex flex-col items-center gap-2">
      <span v-if="profiles.length === 0">
        {{ $t('pages.profiles.noProfiles') }}
      </span>
      <div v-else class="flex justify-center flex-wrap gap-2">
        <div
          v-for="profile in profiles"
          :key="profile.id"
          class="flex flex-col items-center gap-2 p-4 cursor-pointer rounded-lg \
          hover:shadow-lg hover:bg-accent transition-colors"
          @click="deleteProfile(profile.id)"
          @mouseenter="hovering = profile.id"
          @mouseleave="hovering = null"
        >
          <Avatar size="lg" shape="square" class="relative">
            <AvatarImage :src="`https://avatar.vercel.sh/${profile.name}?size=128`" />
            <Trash2 class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-1/2 h-1/2 text-red-500 transition-opacity" :class="{ 'opacity-100': hovering === profile.id, 'opacity-0': hovering !== profile.id }" />
            <AvatarFallback>{{ profile.name[0] }}</AvatarFallback>
          </Avatar>
          <span>{{ profile.name }}</span>
        </div>
      </div>
      <div class="flex gap-4">
        <Button variant="ghost" @click="$router.push({ path: '/' })">
          {{ $t('pages.profiles.back') }}
        </Button>
        <Dialog>
          <DialogTrigger as-child>
            <Button variant="outline">
              {{ $t('pages.profiles.addProfile') }}
            </Button>
          </DialogTrigger>
          <DialogContent class="sm:max-w-[425px]">
            <DialogHeader>
              <DialogTitle>
                {{ $t('pages.profiles.dialog.title') }}
              </DialogTitle>
              <DialogDescription>
                {{ $t('pages.profiles.dialog.description') }}
              </DialogDescription>
            </DialogHeader>
  
            <Form :validation-schema="formSchema" @submit="onSubmit" class="flex flex-col gap-4">
              <FormField v-slot="{ componentField }" name="name">
                <FormItem>
                  <FormLabel>
                    {{ $t('pages.profiles.dialog.name') }}
                  </FormLabel>
                  <FormControl>
                    <Input type="text" placeholder="ValDesign" v-bind="componentField" />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              </FormField>
  
              <FormField v-slot="{ componentField }" name="password">
                <FormItem>
                  <FormLabel>
                    {{ $t('pages.profiles.dialog.password') }}
                  </FormLabel>
                  <FormControl>
                    <Input type="password" placeholder="********" v-bind="componentField" />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              </FormField>
  
              <FormField v-slot="{ componentField }" name="role">
                <FormItem>
                  <FormLabel>
                    {{ $t('pages.profiles.dialog.role') }}
                  </FormLabel>
                  <FormControl>
                    <Input type="text" placeholder="admin" v-bind="componentField" />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              </FormField>
  
              <DialogFooter>
                <DialogClose as-child>
                  <Button variant="outline">
                    {{ $t('pages.profiles.dialog.cancel') }}
                  </Button>
                </DialogClose>
                <DialogClose as-child>
                  <Button type="submit">
                    {{ $t('pages.profiles.dialog.add') }}
                  </Button>
                </DialogClose>
              </DialogFooter>
            </Form>

            <DialogClose
              class="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground"
            >
              <X class="w-4 h-4" />
              <span class="sr-only">Close</span>
            </DialogClose>
          </DialogContent>
        </Dialog>
      </div>
    </div>
  </div>

  <div class="absolute bottom-0 right-0 p-4 ">
    <span v-if="version" class="text-sm text-gray-500">
      {{ $t('app.footerVersion', { version }) }}
    </span>
  </div>
</template>
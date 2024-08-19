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
import { UserRound, Trash2 } from 'lucide-vue-next';

const router = useRouter();

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

  const url = config.http_server + '/profiles?id=' + id;
  console.log(url);
  const res = await fetch(url, {
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
    <h1 class="text-4xl font-bold">Profiles</h1>

    <div class="flex flex-col items-center gap-2">
      <span v-if="profiles.length === 0">No profiles found</span>
      <div v-else class="flex gap-2">
        <div
          v-for="profile in profiles"
          :key="profile.id"
          class="flex flex-col items-center gap-2 p-4 cursor-pointer rounded-lg \
          hover:shadow-lg hover:bg-accent transition-colors"
          @click="deleteProfile(profile.id)"
          @mouseenter="hovering = profile.id"
          @mouseleave="hovering = null"
        >
          <Avatar size="lg" class="relative">
            <AvatarImage :src="`https://avatar.vercel.sh/${profile.name}?size=128`" />
            <component
              :is="hovering === profile.id ? Trash2 : UserRound"
              class="absolute inset-0 w-full h-full"
              :class="hovering === profile.id ? 'text-red-500' : 'text-white'"
            />
            <AvatarFallback>{{ profile.name[0] }}</AvatarFallback>
          </Avatar>
          <span>{{ profile.name }}</span>
        </div>
      </div>
      <Dialog>
        <DialogTrigger as-child>
          <Button variant="outline">Add profile</Button>
        </DialogTrigger>
        <DialogContent class="sm:max-w-[425px]">
          <DialogHeader>
            <DialogTitle>Add profile</DialogTitle>
            <DialogDescription>
              Add new profile to HomeStream
            </DialogDescription>
          </DialogHeader>

          <Form :validation-schema="formSchema" @submit="onSubmit" class="flex flex-col gap-4">
            <FormField v-slot="{ componentField }" name="name">
              <FormItem>
                <FormLabel>Name</FormLabel>
                <FormControl>
                  <Input type="text" placeholder="ValDesign" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>

            <FormField v-slot="{ componentField }" name="password">
              <FormItem>
                <FormLabel>Password</FormLabel>
                <FormControl>
                  <Input type="password" placeholder="********" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>

            <FormField v-slot="{ componentField }" name="role">
              <FormItem>
                <FormLabel>Role</FormLabel>
                <FormControl>
                  <Input type="text" placeholder="admin" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>

            <DialogFooter>
              <DialogClose as-child>
                <Button variant="outline">Cancel</Button>
              </DialogClose>
              <DialogClose as-child>
                <Button type="submit">Add</Button>
              </DialogClose>
            </DialogFooter>
          </Form>
        </DialogContent>
      </Dialog>
    </div>
  </div>
</template>
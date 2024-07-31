<script setup lang="ts">
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { get, set } from '@vueuse/core';
import { Check, Circle, Dot } from 'lucide-vue-next';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import { toTypedSchema } from '@vee-validate/zod'
import * as zod from 'zod';
import { Stepper, StepperItem, StepperSeparator, StepperTitle, StepperTrigger } from '@/components/ui/stepper';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { GenericObject } from 'vee-validate';
import { create, exists, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
// import { onOpenUrl } from '@tauri-apps/plugin-deep-link';
// import { open } from '@tauri-apps/plugin-shell';

const router = useRouter();

const formSchema = [
  zod.object({
    host: zod.string().min(7).max(15),
    port: zod.number().int().positive(),
    username: zod.string().min(3),
    password: zod.string().min(3),
  }),
  zod.object({
    tmdbApiKey: zod.string().min(32),
    tmdbLanguage: zod.string().min(2).max(2),
  }),
  zod.object({
    ftpPath: zod.string().min(1),
    moviesFolder: zod.string().min(1),
    seriesFolder: zod.string().min(1),
    animesFolder: zod.string().min(1),
    appStoragePath: zod.string().min(1),
  }),
]

const stepIndex = ref(1);
const steps = [{
  step: 1,
  title: 'FTP Server',
  description: 'Connect your FTP server to import your movies and TV shows.',
}, {
  step: 2,
  title: 'The Movie Database',
  description: 'Connect to The Movie Database to get metadata for your movies and TV shows.',
}, {
  step: 3,
  title: 'File configuration',
  description: 'Configure the folders where your movies and TV shows are stored.',
}, {
  step: 4,
  title: 'Success',
  description: 'You have successfully connected to your FTP server and The Movie Database.',
}];

const canGoNext = computed(() => stepIndex.value < steps.length);
const canGoBack = computed(() => stepIndex.value > 1);
function goNext() {
  if (get(canGoNext)) set(stepIndex, stepIndex.value + 1);
}
function goBack() {
  if (get(canGoBack)) set(stepIndex, stepIndex.value - 1);
}

async function onSubmit(values: GenericObject) {
  const fileExists = await exists("config.json", { baseDir: BaseDirectory.AppConfig });

  if (!fileExists) {
    await create("config.json", { baseDir: BaseDirectory.AppConfig });
  }

  await writeTextFile("config.json", JSON.stringify({
    ftp_host: values.host,
    ftp_port: String(values.port),
    ftp_user: values.username,
    ftp_password: values.password,
    ftp_path: values.ftpPath,
    movies_folder: values.moviesFolder,
    series_folder: values.seriesFolder,
    animes_folder: values.animesFolder,
    app_storage_path: values.appStoragePath,
    tmdb_api_key: values.tmdbApiKey,
    tmdb_language: values.tmdbLanguage,
  }), { baseDir: BaseDirectory.AppConfig });

  router.push({ path: "/", replace: true });
}

// const openAuthUrl = async () => {
//   await open('https://www.themoviedb.org/auth/access');
// }

// const handleOpenUrl = async () => {
//   await onOpenUrl((urls) => {
//     urls.forEach((url) => {
//       console.log(url);
//     });
//   });
// }

// handleOpenUrl();
</script>

<template>
  <div class="w-full flex items-center justify-center px-12 py-6">
    <Form
      v-slot="{ meta, values, validate }"
      as="" keep-values :validation-schema="toTypedSchema(formSchema[stepIndex - 1])"
    >
      <form
        @submit="(e) => {
          e.preventDefault();
          validate();
  
          if (stepIndex === steps.length) {
            onSubmit(values);
          }
        }"
      >
        <Stepper v-model="stepIndex" class="flex w-full items-start gap-2">
          <StepperItem
            v-for="step in steps"
            :key="step.step"
            v-slot="{ state }"
            class="relative flex w-full flex-col items-center gap-2"
            :step="step.step"
          >
            <StepperSeparator
              v-if="step.step !== steps[steps.length - 1].step"
              class="absolute left-[calc(50%+20px)] right-[calc(-50+10px)] top-5 block h-0.5 shrink-0 rounded-full bg-muted group-data-[state=completed]:bg-primary"
            />
            
            <StepperTrigger as-child>
              <Button
                :variant="state === 'completed' || state === 'active' ? 'default' : 'outline'"
                size="icon"
                class="z-10 rounded-full shrink-0"
                :class="[state === 'active' && 'ring-2 ring-ring ring-offset-2 ring-offset-background']"
                :disabled="state !== 'completed' && !meta.valid"
              >
                <Check v-if="state === 'completed'" class="size-5" />
                <Circle v-if="state === 'active'" />
                <Dot v-if="state === 'inactive'" />
              </Button>
            </StepperTrigger>
  
            <div class="mt-5 flex flex-col items-center text-center">
              <StepperTitle
                :class="[state === 'active' && 'text-primary']"
                class="text-sm font-semibold transition lg:text-base"
              >
                {{ step.title }}
              </StepperTitle>
            </div>
          </StepperItem>
        </Stepper>
  
        <div class="flex flex-col gap-4 mt-4 w-fit px-96">
          <p>{{ steps[stepIndex - 1].description }}</p>

          <template v-if="stepIndex === 1">
            <FormField v-slot="{ componentField }" name="host">
              <FormItem>
                <FormLabel>Host</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
  
            <FormField v-slot="{ componentField }" name="port">
              <FormItem>
                <FormLabel>Port</FormLabel>
                <FormControl>
                  <Input type="number" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
  
            <FormField v-slot="{ componentField }" name="username">
              <FormItem>
                <FormLabel>Username</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
  
            <FormField v-slot="{ componentField }" name="password">
              <FormItem>
                <FormLabel>Password</FormLabel>
                <FormControl>
                  <Input type="password" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </template>
          
          <template v-if="stepIndex === 2">
            <FormField v-slot="{ componentField }" name="tmdbApiKey">
              <FormItem>
                <FormLabel>TMDB API Key</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>

            <FormField v-slot="{ componentField }" name="tmdbLanguage">
              <FormItem>
                <FormLabel>TMDB Language</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </template>

          <template v-if="stepIndex === 3">
            <FormField v-slot="{ componentField }" name="ftpPath">
              <FormItem>
                <FormLabel>FTP Path</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
  
            <FormField v-slot="{ componentField }" name="moviesFolder">
              <FormItem>
                <FormLabel>Movies Folder</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
  
            <FormField v-slot="{ componentField }" name="seriesFolder">
              <FormItem>
                <FormLabel>Series Folder</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
  
            <FormField v-slot="{ componentField }" name="animesFolder">
              <FormItem>
                <FormLabel>Animes Folder</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
  
            <FormField v-slot="{ componentField }" name="appStoragePath">
              <FormItem>
                <FormLabel>App Storage Path</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </template>

          <template v-if="stepIndex === 4">
            <div class="flex flex-col gap-4">
              <p>
                You have successfully connected to your FTP server and The Movie Database.
              </p>
            </div>
          </template>
        </div>
  
        <div class="flex items-center justify-between mt-4 px-96">
          <Button :disabled="!canGoBack" variant="outline" size="sm" @click="goBack">
            Back
          </Button>
          <div class="flex items-center gap-3">
            <Button v-if="stepIndex !== steps.length" :type="meta.valid ? 'button' : 'submit'" :disabled="!canGoNext" size="sm" @click="meta.valid && goNext()">
              Next
            </Button>
            <Button
              v-if="stepIndex === steps.length" size="sm" type="submit"
            >
              Submit
            </Button>
          </div>
        </div>
      </form>
    </Form>
  </div>
</template>

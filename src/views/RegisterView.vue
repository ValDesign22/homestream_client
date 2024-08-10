<script setup lang="ts">
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { get, set } from '@vueuse/core';
import { Check, Circle, Dot, Trash2 } from 'lucide-vue-next';
import { computed, ref } from 'vue';
// import { useRouter } from 'vue-router';
import { toTypedSchema } from '@vee-validate/zod'
import * as zod from 'zod';
import { Stepper, StepperItem, StepperSeparator, StepperTitle, StepperTrigger } from '@/components/ui/stepper';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { GenericObject } from 'vee-validate';
import { create, exists, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { Folder, RemoteFolder } from '@/utils/types';
import { fetch } from '@tauri-apps/plugin-http';
import { Select, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@/components/ui/select';

// const router = useRouter();

const formSchema = [
  zod.object({
    httpServer: zod.string().min(1),
  }),
  zod.object({
    tmdbLanguage: zod.string().min(2).max(2),
  }),
  zod.object({
    folders: zod.array(zod.object({ media_type: zod.string().max(1), path: zod.string().min(1) })),
  }),
];

const remoteFolders = ref<RemoteFolder[]>([]);
const folders = ref<Folder[]>([]);
const httpServer = ref('');

const stepIndex = ref(1);
const steps = [{
  step: 1,
  title: 'Server Configuration',
  description: 'Connect your HomeStream server to the app.',
}, {
  step: 2,
  title: 'The Movie Database',
  description: 'Configure your app language',
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

async function fetchFolders() {
  console.log(httpServer.value);
  if (!httpServer.value) return;

  const response = await fetch(httpServer.value + '/folders', {
    method: 'GET',
  });

  if (!response.ok) console.error('An error occurred while fetching the folders');
  else remoteFolders.value = await response.json();

  console.log(remoteFolders.value);
}

async function onSubmit(values: GenericObject) {
  const fileExists = await exists("config.json", { baseDir: BaseDirectory.AppConfig });

  if (!fileExists) await create("config.json", { baseDir: BaseDirectory.AppConfig });

  await writeTextFile("config.json", JSON.stringify({
    http_server: values.httpServer,
  }), { baseDir: BaseDirectory.AppConfig });

  const response = await fetch(httpServer.value + '/config', {
    method: 'PATCH',
    body: JSON.stringify({
      folders: folders.value,
      tmdb_language: values.tmdbLanguage,
    }),
  });

  fetchFolders();

  if (response.ok) console.log('Configuration saved successfully');
  else console.error('An error occurred while saving the configuration');

  // router.push({ path: "/", replace: true });
}
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
            <FormField v-slot="{ componentField }" name="httpServer">
              <FormItem>
                <FormLabel>HomeStream Server Address</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" v-model="httpServer" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </template>
          
          <template v-if="stepIndex === 2">
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
            <div class="flex flex-col gap-4">
              <div class="flex flex-col gap-4">
                <div v-for="(_, index) in folders" :key="index" class="flex items-center gap-4">
                  <FormField v-slot="{ componentField }" :name="'folders[' + index + '].media_type'">
                    <FormItem>
                      <FormLabel>Media Type</FormLabel>
                      <FormControl>
                        <!-- <Input type="number" v-bind="componentField" onchange="folders[index].media_type = $event.target.value" /> -->
                         <Select v-bind="componentField">
                          <SelectTrigger>
                            <SelectValue placeholder="Select a media type" />
                          </SelectTrigger>
                          <SelectContent>
                            <SelectGroup>
                              <SelectLabel>Media Type</SelectLabel>
                              <SelectItem value="0">Movies</SelectItem>
                              <SelectItem value="1">TV Shows</SelectItem>
                            </SelectGroup>
                          </SelectContent>
                         </Select>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  </FormField>
                  <FormField v-slot="{ componentField }" :name="'folders[' + index + '].path'">
                    <FormItem>
                      <FormControl>
                        <div class="flex w-full max-w-sm items-center gap-1.5">
                          <Input type="text" v-bind="componentField" onchange="folders[index].path = $event.target.value" />
                          <Button variant="destructive" @click="folders = folders.filter((_, i) => i !== index)">
                            <Trash2 />
                          </Button>
                        </div>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  </FormField>
                </div>
                <div class="flex items center gap-4">
                  <Button @click="folders = [...folders, { id: folders.length, name: '', path: '', media_type: 0 }]">
                    Add Folder
                  </Button>
                </div>
              </div>
            </div>
          </template>

          <template v-if="stepIndex === 4">
            <div class="flex flex-col gap-4">
              <p>
                Your configuration has been saved successfully. You can now start using the app.
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

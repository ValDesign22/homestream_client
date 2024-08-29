<script setup lang="ts">
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { get, set } from '@vueuse/core';
import { Check, Circle, Dot, Trash2 } from 'lucide-vue-next';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import { toTypedSchema } from '@vee-validate/zod'
import zod from 'zod';
import { Stepper, StepperItem, StepperSeparator, StepperTitle, StepperTrigger } from '@/components/ui/stepper';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { GenericObject } from 'vee-validate';
import { create, exists, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { IFolder, IRemoteFolder } from '@/utils/types';
import { fetch } from '@tauri-apps/plugin-http';
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@/components/ui/select';
import { TreeSelector } from '@/components/tree';
import { Separator } from '@/components/ui/separator';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const router = useRouter();

const formSchema = [
  zod.object({
    httpServer: zod.string().min(1),
  }),
  zod.object({
    tmdbLanguage: zod.string().min(2).max(2),
  }),
  zod.object({
    folders: zod.array(zod.object({
      media_type: zod.string(),
      path: zod.string(),
      name: zod.string(),
    })).optional(),
  }),
];

const httpServer = ref('');
const skipped = ref(false);
const remoteFolders = ref<IRemoteFolder[]>([]);
const folders = ref<IFolder[]>([]);
const selectedItem = ref<IRemoteFolder | null>(null);
const selectors = ref<boolean[]>([]);

const selectItem = (item: IRemoteFolder | null, index: number) => {
  selectedItem.value = item;
  folders.value[index].path = item ? item.path : '';
};

const toggleSelector = (index: number) => {
  fetchFolders();
  selectors.value = selectors.value.map((_, i) => i === index ? !selectors.value[i] : false);
};

const stepIndex = ref(1);
const steps = [{
  step: 1,
  title: t('register.steps.connect'),
  description: t('register.steps.connectDescription'),
}, {
  step: 2,
  title: t('register.steps.language'),
  description: t('register.steps.languageDescription'),
}, {
  step: 3,
  title: t('register.steps.folders'),
  description: t('register.steps.foldersDescription'),
}, {
  step: 4,
  title: t('register.steps.success'),
  description: t('register.steps.successDescription'),
}];

const canGoNext = computed(() => stepIndex.value < steps.length);
const canGoBack = computed(() => stepIndex.value > 1);
function goNext() {
  if (get(canGoNext)) set(stepIndex, stepIndex.value + 1);
}
function goBack() {
  if (get(canGoBack)) set(stepIndex, stepIndex.value - 1);
}

async function fetchConfig() {
  if (!httpServer.value) return;

  const configResponse = await fetch(httpServer.value + '/config', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!configResponse.ok) console.error('An error occurred while fetching the config');
  else {
    const config = await configResponse.json();
    folders.value = config.folders;
    selectors.value = config.folders.map(() => false);
  }
}

async function fetchFolders() {
  if (!httpServer.value) return;

  const response = await fetch(httpServer.value + '/folders', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) console.error('An error occurred while fetching the folders');
  else remoteFolders.value = await response.json();
}

async function onSubmit(values: GenericObject) {
  const fileExists = await exists("config.json", { baseDir: BaseDirectory.AppConfig });

  if (!fileExists) await create("config.json", { baseDir: BaseDirectory.AppConfig });

  await writeTextFile("config.json", JSON.stringify({
    http_server: values.httpServer,
  }), { baseDir: BaseDirectory.AppConfig });

  const foldersToSend: IFolder[] = values.folders ? values.folders.map((folder: { media_type: string, name: string, path: string }, index: number) => ({
    id: index,
    name: folder.name,
    media_type: parseInt(folder.media_type),
    path: folder.path,
  })) : [];

  const response = await fetch(httpServer.value + '/config', {
    method: 'PATCH',
    body: JSON.stringify({
      folders: skipped.value ? folders.value : foldersToSend,
      tmdb_language: values.tmdbLanguage,
    }),
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (response.ok) router.push({ path: "/", replace: true });
  else console.error('An error occurred while saving the configuration', await response.text());
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
  
          if (stepIndex === steps.length) onSubmit(values);
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
  
        <div class="flex flex-col gap-4 mt-4">
          <span>{{ steps[stepIndex - 1].description }}</span>

          <template v-if="stepIndex === 1">
            <FormField v-slot="{ componentField }" name="httpServer">
              <FormItem>
                <FormLabel>{{ $t('register.form.address') }}</FormLabel>
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
                <FormLabel>{{ $t('register.form.language') }}</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </template>

          <template v-if="stepIndex === 3">
            <div class="flex flex-col gap-4 w-full p-4 border rounded-md">
              <div v-if="!folders.length" class="flex items-center gap-4">
                <span class="text-md font-bold">
                  {{ $t('register.form.noFolders') }}
                </span>
              </div>
              <div v-for="(folder, index) in folders" :key="index" class="flex flex-col gap-4">
                <div class="flex gap-4">
                  <FormField v-slot="{ componentField }" :name="'folders[' + index + '].media_type'" :defaultValue="folder.media_type.toString()">
                    <FormItem>
                      <Select v-bind="componentField" :defaultValue="folder.media_type.toString()">
                        <FormControl>
                          <SelectTrigger class="w-auto">
                            <SelectValue :placeholder="$t('register.form.selectType')" />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent class="w-auto">
                          <SelectGroup class="w-auto">
                            <SelectLabel>
                              {{ $t('register.form.mediaType') }}
                            </SelectLabel>
                            <SelectItem value="0">
                              {{ $t('register.form.movies') }}
                            </SelectItem> 
                            <SelectItem value="1">
                              {{ $t('register.form.tvShows') }}
                            </SelectItem>
                          </SelectGroup>
                        </SelectContent>
                      </Select>
                    </FormItem>
                  </FormField>
                  <FormField v-slot="{ componentField }" :name="'folders[' + index + '].name'" :defaultValue="folder.name">
                    <FormItem>
                      <FormControl>
                        <Input v-bind="componentField" type="text" v-model="folder.name" :placeholder="$t('register.form.folderName')" />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  </FormField>
                </div>
                <div class="flex gap-4">
                  <FormField v-slot="{ componentField }"  :name="'folders[' + index + '].path'" :defaultValue="folder.path">
                    <FormItem>
                      <FormControl>
                        <Input v-bind="componentField" type="text" @click="() => toggleSelector(index)" :value="folder.path.length ? folder.path : $t('register.form.selectFolder')" />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  </FormField>
                  <Button variant="destructive" @click="() => {
                    folders = folders.filter((_, i) => i !== index)
                    selectors = selectors.filter((_, i) => i !== index)
                  }">
                    <Trash2 />
                  </Button>
                </div>

                <TreeSelector
                  :open="selectors[index]"
                  :index="index"
                  :data="remoteFolders"
                  :selectedItem="selectedItem"
                  :selectItem="selectItem"
                  :toggle="(index) => toggleSelector(index)"
                />

                <Separator v-if="index !== folders.length - 1" />
              </div>
              <div class="flex items-center gap-4">
                <Button @click="() => {
                  folders = [...folders, { id: folders.length, name: '', path: '', media_type: 0 }]
                  selectors = [...selectors, false]
                }">
                  {{ $t('register.form.addFolder') }}
                </Button>
              </div>
            </div>
          </template>

          <template v-if="stepIndex === 4">
            <div class="flex flex-col gap-4">
              <span>
                {{ $t('register.form.success') }}
              </span>
            </div>
          </template>
        </div>
  
        <div class="flex items-center justify-between mt-4">
          <Button :disabled="!canGoBack" variant="outline" size="sm" @click="goBack">
            {{ $t('register.form.back') }}
          </Button>
          <div class="flex items-center gap-3">
            <Button v-if="stepIndex === 3 && !meta.touched" :type="meta.valid ? 'button' : 'submit'" :disabled="!canGoNext" size="sm" @click="() => {
              fetchFolders();
              goNext();
              skipped = true;
            }">
              {{ $t('register.form.skip') }}
            </Button>
            <Button v-else-if="stepIndex !== steps.length" :type="meta.valid ? 'button' : 'submit'" :disabled="!canGoNext" size="sm" @click="() => {
              fetchConfig();
              meta.valid && goNext();
            }">
              {{ $t('register.form.next') }}
            </Button>
            <Button v-if="stepIndex === steps.length" size="sm" type="submit">
              {{ $t('register.form.submit') }}
            </Button>
          </div>
        </div>
      </form>
    </Form>
  </div>
</template>

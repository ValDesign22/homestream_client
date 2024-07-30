<script setup lang="ts">
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { get, set } from '@vueuse/core';
import { Check, Circle, Dot } from 'lucide-vue-next';
import { computed, ref } from 'vue';
import { toTypedSchema } from '@vee-validate/zod'
import * as zod from 'zod';
import { Stepper, StepperItem, StepperSeparator, StepperTitle, StepperTrigger } from '@/components/ui/stepper';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { GenericObject } from 'vee-validate';

const formSchema = [
  zod.object({
    host: zod.string().min(7).max(15),
    port: zod.number().int().positive(),
    username: zod.string().min(3),
    password: zod.string().min(3),
  }),
]

const stepIndex = ref(1);
const steps = [{
  step: 1,
  title: 'FTP Server',
  description: 'Connect your FTP server to import your movies and TV shows.',
}, {
  step: 2,
  title: 'TheMovieDB',
  description: 'Connect your TheMovieDB account to get the latest movies and TV shows.',
}, {
  step: 3,
  title: 'Load',
  description: 'Load your movies and TV shows to start watching.',
}, {
  step: 4,
  title: 'Enjoy',
  description: 'Enjoy your movies and TV shows.',
}];

const canGoNext = computed(() => stepIndex.value < steps.length);
const canGoBack = computed(() => stepIndex.value > 1);
function goNext() {
  if (get(canGoNext)) set(stepIndex, stepIndex.value + 1);
}
function goBack() {
  if (get(canGoBack)) set(stepIndex, stepIndex.value - 1);
}

function onSubmit(values: GenericObject) {
  console.log(values);
}
</script>

<template>
  <container class="w-full flex items-center justify-center px-12 py-6">
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
  
        <div class="flex flex-col gap-4 mt-4 px-96">
          <template v-if="stepIndex === 1">
            <p>{{ steps[stepIndex - 1].description }}</p>
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
        </div>
  
        <div class="flex items-center justify-between mt-4 px-96">
          <Button :disabled="!canGoBack" variant="outline" size="sm" @click="goBack">
            Back
          </Button>
          <div class="flex items-center gap-3">
            <Button v-if="stepIndex !== 3" :type="meta.valid ? 'button' : 'submit'" :disabled="!canGoNext" size="sm" @click="meta.valid && goNext()">
              Next
            </Button>
            <Button
              v-if="stepIndex === 3" size="sm" type="submit"
            >
              Submit
            </Button>
          </div>
        </div>
      </form>
    </Form>
  </container>
</template>

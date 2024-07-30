<script setup lang="ts">
import { Check, HardDrive, TvMinimalPlay, User } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Stepper, StepperDescription, StepperIndicator, StepperItem, StepperSeparator, StepperTitle, StepperTrigger } from '@/components/ui/stepper';
import { ref } from 'vue';

let steps = [{
  step: 1,
  title: 'FTP Server',
  description: 'Connect your FTP server to import your movies and TV shows.',
  icon: HardDrive,
}, {
  step: 2,
  title: 'TheMovieDB',
  description: 'Connect your TheMovieDB account to get the latest movies and TV shows.',
  icon: User,
}, {
  step: 3,
  title: 'Load',
  description: 'Load your movies and TV shows to start watching.',
  icon: TvMinimalPlay,
}, {
  step: 4,
  title: 'Enjoy',
  description: 'Enjoy your movies and TV shows.',
  icon: Check,
}];

const currentStep = ref(1);
const connectionTested = ref(false);
const connectionSuccess = ref(false);
</script>

<template>
  <container class="flex flex-col items-center justify-between h-screen p-4">
    <Stepper>
      <StepperItem
        v-for="item in steps"
        :key="item.step"
        class="basis-1/4"
        :step="item.step"
      >
        <StepperTrigger :onClick="() => currentStep = item.step">
          <StepperIndicator>
            <component :is="item.icon" class="w-4 h-4" />
          </StepperIndicator>
          <div class="flex flex-col">
            <StepperTitle>{{ item.title }}</StepperTitle>
            <StepperDescription>{{ item.description }}</StepperDescription>
          </div>
        </StepperTrigger>
        <StepperSeparator
          v-if="item.step !== steps[steps.length - 1].step"
          class="w-full h-px"
        />
      </StepperItem>
    </Stepper>
    <Card class="w-[350px]" v-if="currentStep === 1">
      <CardHeader>
        <CardTitle>FTP Server</CardTitle>
        <CardDescription>
          Connect your FTP server to import your movies and TV shows.
        </CardDescription>
      </CardHeader>
      <CardContent class="space-y-2">
        <form>
          <div class="space-y-1">
            <Label for="ftp-host">Host</Label>
            <Input id="ftp-host" type="text" placeholder="ex: 192.168.1.254" />
          </div>
          <div class="space-y-1">
            <Label for="ftp-port">Port</Label>
            <Input id="ftp-port" type="number" placeholder="ex: 21" />
          </div>
          <div class="space-y-1">
            <Label for="ftp-username">Username</Label>
            <Input id="ftp-username" type="text" placeholder="ex: user" />
          </div>
          <div class="space-y-1">
            <Label for="ftp-password">Password</Label>
            <Input id="ftp-password" type="password" placeholder="ex: password" />
          </div>
        </form>
      </CardContent>
      <CardFooter class="flex justify-between">
        <div class="flex items-center space-x-2">
          <Button variant="ghost">Test connection</Button>
          <span>{{ connectionTested ? connectionSuccess ? 'Connection successful' : 'Connection failed' : '' }}</span>
        </div>
        <Button :disabled="!connectionTested" :onClick="() => currentStep = 2">Next</Button>
      </CardFooter>
    </Card>
  </container>
</template>

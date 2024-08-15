<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
</script>

<template>
  <div class="flex flex-col items-center justify-center h-screen gap-4">
    <Card class="w-96">
      <CardHeader class="flex flex-col items-center gap-2">
        <CardTitle>Welcome to HomeStream</CardTitle>
        <CardDescription>
          HomeStream is a simple and easy-to-use media server that allows you to stream your media files to any device.
        </CardDescription>
      </CardHeader>
      <CardContent class="flex flex-col items-center gap-4">
        <Button @click="() => $router.push({ path: '/browse' })">Start browsing</Button>
      </CardContent>
    </Card>
  </div>
</template>
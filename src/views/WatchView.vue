<script setup lang="ts">
import { TMDBImage } from '@/components/image';
import { Button } from '@/components/ui/button';
import { HoverCard, HoverCardContent, HoverCardTrigger } from '@/components/ui/hover-card';
import { Slider } from '@/components/ui/slider';
import { AudioTrack, Config, Episode, Movie, SubtitleTrack } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { useEventListener, useGamepad } from '@vueuse/core';
import { ChevronLeft, Maximize, MessageSquareText, Minimize, Pause, Play, RotateCcw, RotateCw, Volume1, Volume2, VolumeX } from 'lucide-vue-next';
import { onMounted, onUnmounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { getNameByISO6392B } from '@/utils/languages';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { computed } from 'vue';

const router = useRouter();
const route = useRoute();

const movieLogo = ref<string | null>(null);

const videoItem = ref<Movie | Episode | null>(null);
const videoElem = ref<HTMLVideoElement>();
const sourceElem = ref<HTMLSourceElement | null>(null);
const playing = ref(false);
const showControls = ref(true);
const controlsBox = ref<HTMLDivElement | null>(null);
const progressValue = ref([0]);
const isFullscreen = ref(false);
const isUserSliding = ref(false);
const isHoveringVolume = ref(false);
const playerVolume = ref([1]);
const audioTracks = ref<AudioTrack[]>([]); // Here but currently not supported https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/audioTracks#browser_compatibility
const subtitles = ref<SubtitleTrack[]>([]);
const isHoveringTracks = ref(false);

const { isSupported, gamepads } = useGamepad();
const gamepad = computed(() => gamepads.value.find(g => g.mapping === 'standard'));

let hideControlsTimeout: ReturnType<typeof setTimeout>;

const formatTime = (seconds: number): string => {
  const hrs = ~~(seconds / 3600);
  const mins = ~~((seconds % 3600) / 60);
  const secs = ~~seconds % 60;
  if (hrs > 0) return `${hrs}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  else return `${mins}:${secs.toString().padStart(2, '0')}`;
}

const startHideControlsTimer = () => {
  if (hideControlsTimeout) clearTimeout(hideControlsTimeout);
  hideControlsTimeout = setTimeout(() => {
    if (playing.value) showControls.value = false;
  }, 5000);
}

const resetControlsTimer = () => {
  showControls.value = true;
  startHideControlsTimer();
}

const backward = () => {
  if (videoElem.value) videoElem.value.currentTime -= 10;
};

const forward = () => {
  if (videoElem.value) videoElem.value.currentTime += 10;
};

const togglePlaying = () => {
  if (!videoElem.value) return;
  playing.value = !playing.value;
  if (playing.value) {
    videoElem.value.play();
    startHideControlsTimer();
  } else {
    videoElem.value.pause();
    resetControlsTimer();
  }
};

const toggleFullscreen = async (state: boolean) => {
  const appwindow = getCurrentWindow();
  const isAppFullscreen = await appwindow.isFullscreen();
  await appwindow.setFullscreen(state ?? !isAppFullscreen);
  isFullscreen.value = state ?? !isAppFullscreen;
};

const useSubtitleTrack = (index: number) => {
  if (videoElem.value) {
    // const track = videoElem.value.textTracks[index];
    // if (track) {
    //   track.mode = track.mode === 'showing' ? 'hidden' : 'showing';
    // }
    console.log(`Subtitle index ${index} currently not supported`);
  }
};

const changeVolume = (delta: number) => {
  playerVolume.value = [Math.min(Math.max(playerVolume.value[0] + delta, 0), 1)];
};

useEventListener(document, 'mousemove', () => {
  if (playing.value) resetControlsTimer();
});

useEventListener(document, 'click', (event) => {
  if (isHoveringTracks.value) return;
  if (controlsBox.value) {
    const bbox = controlsBox.value.getBoundingClientRect();
    if (!(event.clientX >= bbox.left && event.clientX <= bbox.right && event.clientY >= bbox.top && event.clientY <= bbox.bottom)) togglePlaying();
  }
});

useEventListener(document, 'keydown', (event) => {
  if (!videoElem.value) return;
  if (event.key === ' ' || event.key === 'k' || event.key === 'MediaPlayPause') togglePlaying();
  if (event.key === 'm') playerVolume.value = [playerVolume.value[0] === 0 ? 1 : 0];
  if (event.key === 'f') toggleFullscreen(!isFullscreen.value);
  if (event.key === 'Escape') toggleFullscreen(false);
  if (event.key === 'ArrowLeft' || event.key === 'j') backward();
  if (event.key === 'ArrowRight' || event.key === 'l') forward();
  if (event.key === 'ArrowUp') changeVolume(0.1);
  if (event.key === 'ArrowDown') changeVolume(-0.1);
  if (event.key === '0') videoElem.value.currentTime = 0;
  if (event.key === '1') videoElem.value.currentTime = videoElem.value?.duration * 0.1;
  if (event.key === '2') videoElem.value.currentTime = videoElem.value?.duration * 0.2;
  if (event.key === '3') videoElem.value.currentTime = videoElem.value?.duration * 0.3;
  if (event.key === '4') videoElem.value.currentTime = videoElem.value?.duration * 0.4;
  if (event.key === '5') videoElem.value.currentTime = videoElem.value?.duration * 0.5;
  if (event.key === '6') videoElem.value.currentTime = videoElem.value?.duration * 0.6;
  if (event.key === '7') videoElem.value.currentTime = videoElem.value?.duration * 0.7;
  if (event.key === '8') videoElem.value.currentTime = videoElem.value?.duration * 0.8;
  if (event.key === '9') videoElem.value.currentTime = videoElem.value?.duration * 0.9;
});

watch(progressValue, () => {
  if (isUserSliding.value && videoElem.value) videoElem.value.currentTime = videoElem.value.duration * progressValue.value[0] / 100;
});

watch(playerVolume, () => {
  if (videoElem.value) videoElem.value.volume = playerVolume.value[0];
});

const gamepadInterval = setInterval(() => {
  if (!isSupported.value) return;
  if (!gamepad.value) return;
  if (!videoElem.value) return;
  if (gamepad.value.buttons[0].pressed) togglePlaying(); // A or Cross
  if (gamepad.value.buttons[1].pressed) playerVolume.value = [playerVolume.value[0] === 0 ? 1 : 0]; // B or Circle
  if (gamepad.value.buttons[3].pressed) toggleFullscreen(!isFullscreen.value); // Y or Triangle
  if (gamepad.value.buttons[4].pressed || gamepad.value.axes[0] < -0.5) backward(); // L1 or L Stick Left
  if (gamepad.value.buttons[5].pressed || gamepad.value.axes[0] > 0.5) forward(); // R1 or L Stick Right
  const currentVideoTime = videoElem.value.currentTime;
  if (gamepad.value.buttons[6].pressed) { // L2
    const percentageBackward = Math.floor(currentVideoTime / videoElem.value.duration * 10) / 10;
    videoElem.value.currentTime = videoElem.value.duration * (percentageBackward - 0.1);
  }
  if (gamepad.value.buttons[7].pressed) { // R2
    const percentageForward = Math.ceil(currentVideoTime / videoElem.value.duration * 10) / 10;
    videoElem.value.currentTime = videoElem.value.duration * percentageForward;
  }
  if (gamepad.value.buttons[12].pressed) changeVolume(0.1); // D-Pad Up
  if (gamepad.value.buttons[13].pressed) changeVolume(-0.1); // D-Pad Down
}, 100);

const loadData = async () => {
  const config = await invoke<Config | null>("get_config");
  if (config) {
    const route = router.currentRoute.value;
    const videoId = route.params.id;
    if (!videoId) router.push({ path: '/browse' });

    const details = await fetch(config.http_server + `/details?id=${videoId}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    if (details.ok) {
      const video = await details.json();
      videoItem.value = video;

      if (videoItem.value) {
        if ('logo_path' in videoItem.value) movieLogo.value = `https://image.tmdb.org/t/p/w300${(videoItem.value as Movie).logo_path}`;

        const tracks = await fetch(config.http_server + `/tracks?id=${videoId}`, {
          method: 'GET',
          headers: {
            'Content-Type': 'application/json',
          },
        });
        if (tracks.ok) {
          const data = await tracks.json();
          data.tracks.forEach((track: AudioTrack | SubtitleTrack) => {
            if (track.codec_type === 'audio') audioTracks.value.push({
              ...track as AudioTrack,
              enabled: false,
            });
            if (track.codec_type === 'subtitle') subtitles.value.push({
              ...track as SubtitleTrack,
              enabled: false,
            });
          });
        }

        if (sourceElem.value && videoElem.value) {
          sourceElem.value.src = config.http_server + `/video?id=${videoId}`;
          videoElem.value.load();

          for (const track of subtitles.value) {
            const trackElem = document.createElement('track') as HTMLTrackElement;
            trackElem.src = config.http_server + `/extract?id=${videoId}&extract_type=subtitle&track_index=${track.index}`;
            trackElem.label = track.language;
            trackElem.kind = 'subtitles';
            videoElem.value.appendChild(trackElem);
          }

          videoElem.value.onloadedmetadata = () => {
            if (!videoElem.value) return;
            videoElem.value.volume = playerVolume.value[0];
            videoElem.value.play();
            playing.value = true;
          };

          videoElem.value.ontimeupdate = () => {
            if (!videoElem.value || isUserSliding.value) return;
            progressValue.value = [videoElem.value.currentTime / videoElem.value.duration * 100];
          };
        }
      }
    }
    else router.push({ path: '/browse' });
  }
  else router.push({ path: "/register", replace: true });
};

onMounted(loadData);

watch(route, loadData);

onUnmounted(() => {
  if (videoElem.value) {
    videoElem.value.pause();
    videoElem.value.src = '';
    videoElem.value.load();
    toggleFullscreen(false);
  }
  clearInterval(gamepadInterval);
});
</script>

<template>
  <div class="w-full h-screen flex flex-col justify-center items-center bg-black relative" :class="{ 'cursor-none': !showControls }">
    <video ref="videoElem" class="w-full h-full">
      <source ref="sourceElem" type="video/mp4" />
    </video>
    <div v-if="videoItem" class="absolute top-0 left-0 w-full h-full flex flex-col justify-between p-8 bg-black bg-opacity-50">
      <div v-if="showControls" class="flex">
        <ChevronLeft class="cursor-pointer" @click="router.back()" />
      </div>
      <div v-if="!playing" class="flex flex-col gap-4">
        <TMDBImage
          v-if="movieLogo"
          :image="movieLogo"
          :alt="videoItem.title"
          type="logo"
          size="original"
          class="w-[20vw] h-auto object-cover"
        />
        <h1 v-else class="text-4xl">{{ videoItem.title }}</h1>
      </div>
      <div v-if="showControls" ref="controlsBox" class="flex flex-col gap-4">
        <Slider
          v-model="progressValue"
          ref="slider"
          class="w-full"
          :max="100"
          :step="0.01"
          @pointerdown="() => isUserSliding = true"
          @pointerup="() => isUserSliding = false"
        />
        <div class="relative flex justify-between">
          <div v-if="videoElem" class="flex gap-4">
            <span>{{ formatTime(videoElem.currentTime) }} / {{ formatTime(videoElem.duration) }}</span>
          </div>
          <div class="absolute left-1/2 transform -translate-x-1/2 flex gap-4">
            <RotateCcw @click="backward" class="cursor-pointer" />
            <component :is="playing ? Pause : Play" class="cursor-pointer" @click="togglePlaying" />
            <RotateCw @click="forward" class="cursor-pointer" />
          </div>
          <div class="flex gap-4">
            <div
              class="flex gap-4"
              @mouseenter="() => isHoveringVolume = true"
              @mouseleave="() => isHoveringVolume = false"
            >
              <Slider
                v-if="isHoveringVolume"
                v-model="playerVolume"
                class="w-32"
                :max="1"
                :step="0.01"
              />
              <component :is="playerVolume[0] === 0 ? VolumeX : playerVolume[0] < 0.5 ? Volume1 : Volume2" class="cursor-pointer" @click="() => playerVolume[0] = playerVolume[0] === 0 ? 0.5 : 0" />
            </div>
            <HoverCard v-if="subtitles.length > 0">
              <HoverCardTrigger as-child>
                <MessageSquareText class="cursor-pointer" />
              </HoverCardTrigger>
              <HoverCardContent @mouseenter="() => isHoveringTracks = true" @mouseleave="() => isHoveringTracks = false" class="flex flex-col gap-4">
                <span>Subtitles</span>
                <Button variant="ghost" v-for="(track, index) in subtitles" :key="track.index" @click="() => useSubtitleTrack(index)">
                  {{ `${getNameByISO6392B(track.language)}${track.handler_name ? `- ${track.handler_name}` : ''}` }}
                </Button>
              </HoverCardContent>
            </HoverCard>
            <component :is="isFullscreen ? Minimize : Maximize" class="cursor-pointer" @click="() => toggleFullscreen(!isFullscreen)" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
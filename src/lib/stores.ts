import { TColor, colors, EMediaType, IConfig, IEpisode, IMovie, IProfile, ITvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { defineStore } from 'pinia';
import { saveAll } from 'tauri-plugin-pinia';
import { ref } from 'vue';

const patchProfile = async (profile: IProfile) => {
  const config = await invoke<IConfig | null>('get_config');
  if (!config) return;

  const res = await fetch(`${config.http_server}/profiles?id=${profile.id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ updatedProfile: profile }),
  });

  return res.ok;
};

function debounce() {
  const locale = ref('en');
  const theme = ref<TColor>('zinc');
  const profile = ref<IProfile | null>(null);

  const setLocale = (newLocale: string) => {
    locale.value = newLocale;
    save();
  };

  const setTheme = (newTheme: TColor) => {
    theme.value = newTheme;
    document.documentElement.classList.remove(
      ...colors.map((c) => `theme-${c}`)
    );
    document.documentElement.classList.add(`theme-${newTheme}`);
    save();
  };

  const addToFavorites = (item: IMovie | ITvShow) => {
    if (!profile.value) return;
    if (profile.value.favorites.some((i) => i.id === item.id)) return;
    profile.value.favorites.push(item);

    saveProfile();
    save();
  };
  const removeFromFavorites = (item: IMovie | ITvShow) => {
    if (!profile.value) return;
    profile.value.favorites = profile.value.favorites.filter((i) => i.id !== item.id);

    saveProfile();
    save();
  }
  const isInFavorites = (item: IMovie | ITvShow) => {
    if (!profile.value) return false;
    return profile.value.favorites.some((i) => i.id === item.id);
  }

  const addToWatchlist = (item: IMovie | ITvShow) => {
    if (!profile.value) return;
    if (profile.value.watchlist.some((i) => i.id === item.id)) return;
    profile.value.watchlist.push(item);

    saveProfile();
    save();
  };
  const removeFromWatchlist = (item: IMovie | ITvShow) => {
    if (!profile.value) return;
    profile.value.watchlist = profile.value.watchlist.filter((i) => i.id !== item.id);

    saveProfile();
    save();
  }
  const isInWatchlist = (item: IMovie | ITvShow) => {
    if (!profile.value) return false;
    return profile.value.watchlist.some((i) => i.id === item.id);
  }

  const getProgress = (item: IMovie | IEpisode) => {
    if (!profile.value) return 0;
    const historyItem = profile.value.history.find((i) => i.id === item.id);
    return historyItem ? historyItem.progress : 0;
  }
  const setProgress = (item: IMovie | IEpisode, progress: number, watched: boolean = false) => {
    if (!profile.value) return;
    const itemIndex = profile.value.history.findIndex((i) => i.id === item.id);
    if (itemIndex === -1) profile.value.history.push({
      id: item.id,
      date: new Date().toISOString(),
      title: item.title,
      media_type: 'episode_number' in item ? EMediaType.TvShow : EMediaType.Movie,
      watched,
      progress,
    });
    else profile.value.history[itemIndex] = {
      ...profile.value.history[itemIndex],
      date: new Date().toISOString(),
      watched,
      progress,
    };

    saveProfile();
    save();
  }

  const setProfile = (newProfile: IProfile | null) => {
    profile.value = newProfile;
    save();
  }

  const saveProfile = async () => {
    if (!profile.value) return;
    const ok = await patchProfile(profile.value);
    if (!ok) throw new Error('Failed to save profile');
  }

  const save = async () => {
    await saveAll();
  }

  return {
    locale,
    theme,
    profile,
    setLocale,
    setTheme,
    addToFavorites,
    removeFromFavorites,
    isInFavorites,
    addToWatchlist,
    removeFromWatchlist,
    isInWatchlist,
    getProgress,
    setProgress,
    setProfile,
    saveProfile,
    save,
  };
}

const useStore = defineStore('homestream', debounce, {
  tauri: {
    syncStrategy: 'debounce',
    syncInterval: 1000,
  }
});

export default useStore;

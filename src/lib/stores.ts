import { TColor, colors, EMediaType, IConfig, IEpisode, IMovie, IProfile, ITvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { defineStore } from 'pinia';
import { saveAll } from 'tauri-plugin-pinia';

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

const useStore = defineStore('homestream', {
  state: () => ({
    locale: 'en',
    theme: 'zinc' as TColor,
    profile: null as IProfile | null,
  }),
  actions: {
    setLocale(locale: string) {
      this.locale = locale;
      this.save();
    },
    setTheme(theme: TColor) {
      this.theme = theme;
      document.documentElement.classList.remove(
        ...colors.map((c) => `theme-${c}`)
      );
      document.documentElement.classList.add(`theme-${theme}`);
      this.save();
    },
    addToFavorites(item: IMovie | ITvShow) {
      if (!this.profile) return;
      if (this.profile.favorites.some((i) => i.id === item.id)) return;
      this.profile.favorites.push(item);

      this.saveProfile();
      this.save();
    },
    removeFromFavorites(item: IMovie | ITvShow) {
      if (!this.profile) return;
      this.profile.favorites = this.profile.favorites.filter((i) => i.id !== item.id);

      this.saveProfile();
      this.save();
    },
    isInFavorites(item: IMovie | ITvShow) {
      if (!this.profile) return false;
      return this.profile.favorites.some((i) => i.id === item.id);
    },
    addToWatchlist(item: IMovie | ITvShow) {
      if (!this.profile) return;
      if (this.profile.watchlist.some((i) => i.id === item.id)) return;
      this.profile.watchlist.push(item);

      this.saveProfile();
      this.save();
    },
    removeFromWatchlist(item: IMovie | ITvShow) {
      if (!this.profile) return;
      this.profile.watchlist = this.profile.watchlist.filter((i) => i.id !== item.id);

      this.saveProfile();
      this.save();
    },
    isInWatchlist(item: IMovie | ITvShow) {
      if (!this.profile) return false;
      return this.profile.watchlist.some((i) => i.id === item.id);
    },
    getProgress(item: IMovie | IEpisode) {
      if (!this.profile) return 0;
      const historyItem = this.profile.history.find((i) => i.id === item.id);
      return historyItem ? historyItem.progress : 0;
    },
    setProgress(item: IMovie | IEpisode, progress: number, watched: boolean = false) {
      if (!this.profile) return;
      const itemIndex = this.profile.history.findIndex((i) => i.id === item.id);
      if (itemIndex === -1) this.profile.history.push({
        id: item.id,
        date: new Date().toISOString(),
        title: item.title,
        media_type: 'episode_number' in item ? EMediaType.TvShow : EMediaType.Movie,
        watched,
        progress,
      });
      else this.profile.history[itemIndex] = {
        ...this.profile.history[itemIndex],
        date: new Date().toISOString(),
        watched,
        progress,
      };

      this.saveProfile();
      this.save();
    },
    setProfile(profile: IProfile | null) {
      this.profile = profile;
      this.save();
    },
    async saveProfile() {
      if (!this.profile) return;
      const ok = await patchProfile(this.profile);
      if (!ok) throw new Error('Failed to save profile');
    },
    async save() {
      await saveAll();
    }
  },
  tauri: {
    debounce: 1000,
  }
});

export default useStore;

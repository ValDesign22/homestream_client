import { IConfig, IMovie, IProfile, ITvShow } from '@/utils/types';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { Store } from '@tauri-apps/plugin-store';

class StoreService {
  store: Store;

  constructor() {
    this.store = new Store('store.bin');
  }

  async save() {
    await this.store.save();
  } 

  async getProfile(): Promise<IProfile | null> {
    return await this.store.get('profile') as IProfile | null;
  }

  async setProfile(profile: IProfile | null) {
    await this.store.set('profile', profile);
    await this.save();
  }

  async isInFavorites(item: IMovie | ITvShow): Promise<boolean> {
    const profile = await this.getProfile();
    if (!profile) return false;
    return profile.favorites.some((i) => i.id === item.id);
  }

  async isInWatchlist(item: IMovie | ITvShow): Promise<boolean> {
    const profile = await this.getProfile();
    if (!profile) return false;
    return profile.watchlist.some((i) => i.id === item.id);
  }

  async addToFavorites(item: IMovie | ITvShow) {
    const profile = await this.getProfile();
    if (!profile) return;

    const config = await invoke<IConfig | null>('get_config');
    if (!config) return;

    if (await this.isInFavorites(item)) return;

    profile.favorites.push(item);

    const res = await fetch(`${config.http_server}/profiles?id=${profile.id}`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(profile),
    });

    if (!res.ok) profile.favorites = profile.favorites.filter((i) => i.id !== item.id);

    await this.setProfile(profile);
  }

  async removeFromFavorites(item: IMovie | ITvShow) {
    const profile = await this.getProfile();
    if (!profile) return;

    const config = await invoke<IConfig | null>('get_config');
    if (!config) return;

    if (!await this.isInFavorites(item)) return;

    profile.favorites = profile.favorites.filter((i) => i.id !== item.id);

    const res = await fetch(`${config.http_server}/profiles?id=${profile.id}`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(profile),
    });

    if (!res.ok) profile.favorites.push(item);

    await this.setProfile(profile);
  }

  async addToWatchlist(item: IMovie | ITvShow) {
    const profile = await this.getProfile();
    if (!profile) return;

    const config = await invoke<IConfig | null>('get_config');
    if (!config) return;

    if (await this.isInWatchlist(item)) return;

    profile.watchlist.push(item);

    const res = await fetch(`${config.http_server}/profiles?id=${profile.id}`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(profile),
    });

    if (!res.ok) profile.watchlist = profile.watchlist.filter((i) => i.id !== item.id);

    await this.setProfile(profile);
  }

  async removeFromWatchlist(item: IMovie | ITvShow) {
    const profile = await this.getProfile();
    if (!profile) return;

    const config = await invoke<IConfig | null>('get_config');
    if (!config) return;

    if (!await this.isInWatchlist(item)) return;

    profile.watchlist = profile.watchlist.filter((i) => i.id !== item.id);

    const res = await fetch(`${config.http_server}/profiles?id=${profile.id}`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(profile),
    });

    if (!res.ok) profile.watchlist.push(item);

    await this.setProfile(profile);
  }
}

export const useStore = new StoreService();

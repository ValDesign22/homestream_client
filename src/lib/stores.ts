import { IProfile } from '@/utils/types';
import { defineStore } from 'pinia';

export const useStore = defineStore('main', {
  state: () => ({
    profile: null as IProfile | null,
  }),
  actions: {
    setProfile(profile: IProfile | null) {
      this.profile = profile;
    },
  },
});

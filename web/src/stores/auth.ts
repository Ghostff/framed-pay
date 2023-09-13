import { defineStore } from 'pinia';
import { UserInterface } from '@/models/user';
import router from '@/router';
import axios from 'axios';

export const useAuthStore = defineStore('auth', {
  state: (): { user: UserInterface | null } => ({
    user: null
  }),
  getters: {
    isAuthenticated(state) {
      return state.user !== null;
    },
    roleName(state) {
      return state.user?.role || '';
    }
  },
  actions: {
    setUser(userData: UserInterface | null): void {
      this.user = userData;
    },
    async login(userData: UserInterface): Promise<void> {
      if (typeof userData === 'object' && userData !== null) {
        this.setUser(userData);
        await router.push({ name: 'dashboard' });
      }
    },
    async logout(): Promise<void> {
      await axios.get('logout');
      await router.push({ name: 'landing-page' });
      this.setUser(null);
    }
  }
});

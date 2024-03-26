import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/auth.store';
import { credentialsStore } from '$lib/stores/credentials.store';
import { popupCenter } from '$lib/utils/login-popup.utils';
import { AuthClient } from '@dfinity/auth-client';

let cachedClient: AuthClient | undefined = undefined;
const getAuthClient = async () => {
  if (!cachedClient) {
    cachedClient = await AuthClient.create({
      idleOptions: {
        disableIdle: true,
      },
    });
  }
  return cachedClient;
};

export const login = async () => {
  // This service never fails. It will manage the error handling internally.
  try {
    // TODO: Set loading state
    const authClient = await getAuthClient();
    return new Promise<void>((resolve) => {
      authClient.login({
        onSuccess: async () => {
          const identity = authClient.getIdentity();
          authStore.set({ identity });
          resolve();
        },
        onError: () => {
          // TODO: Handle error
          authStore.set({ identity: null });
          resolve();
        },
        identityProvider: import.meta.env.VITE_INTERNET_IDENTITY_URL,
        windowOpenerFeatures: popupCenter(),
        // One week
        maxTimeToLive: 7n * 24n * 3_600_000_000_000n,
      });
    });
  } catch (err) {
    // TODO: Handle error
  }
};

export const syncAuth = async () => {
  try {
    const authClient = await getAuthClient();
    if (await authClient.isAuthenticated()) {
      const identity = authClient.getIdentity();
      authStore.set({ identity });
    } else {
      authStore.set({ identity: null });
    }
  } catch (err) {
    // TODO: Handle error
    console.log('error syncAuth');
    console.error(err);
  }
};

export const logout = async () => {
  try {
    const authClient = await getAuthClient();
    await authClient.logout();
  } catch (err) {
    // TODO: Handle error
  } finally {
    // Always clear the cached client and the identity store.
    credentialsStore.reset();
    cachedClient = undefined;
    authStore.set({ identity: null });
    goto('/');
  }
};

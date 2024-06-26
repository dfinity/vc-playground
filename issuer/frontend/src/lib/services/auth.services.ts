import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/auth.store';
import { popupCenter } from '$lib/utils/login-popup.utils';
import { AuthClient } from '@dfinity/auth-client';
import type { ToastStore } from '@skeletonlabs/skeleton';

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

export const login = async (toastStore: ToastStore) => {
  // This service never fails. It will manage the error handling internally.
  try {
    const authClient = await getAuthClient();
    return new Promise<void>((resolve) => {
      authClient.login({
        onSuccess: async () => {
          const identity = authClient.getIdentity();
          authStore.set({ identity });
          resolve();
        },
        onError: (err) => {
          authStore.set({ identity: null });
          if (err === 'UserInterrupt') {
            // We do not display an error if user explicitly cancelled the process of sign-in. User is most certainly aware of it.
            return;
          }
          toastStore.trigger({
            message: `Oops! There was an error while trying to login. Please try again. ${err}`,
            background: 'variant-filled-error',
          });
          resolve();
        },
        identityProvider: import.meta.env.VITE_INTERNET_IDENTITY_URL,
        windowOpenerFeatures: popupCenter(),
        // One week
        maxTimeToLive: 7n * 24n * 3_600_000_000_000n,
      });
    });
  } catch (err) {
    toastStore.trigger({
      message: `Oops! There was an error while trying to login. Please try again. ${err}`,
      background: 'variant-filled-error',
    });
  }
};

export const syncAuth = async () => {
  const authClient = await getAuthClient();
  if (await authClient.isAuthenticated()) {
    const identity = authClient.getIdentity();
    authStore.set({ identity });
  } else {
    authStore.set({ identity: null });
  }
};

export const logout = async () => {
  try {
    const authClient = await getAuthClient();
    await authClient.logout();
  } catch (err) {
    console.info('Error while trying to logout', err);
  } finally {
    // Always clear the cached client and the identity store.
    cachedClient = undefined;
    authStore.set({ identity: null });
    goto('/');
  }
};

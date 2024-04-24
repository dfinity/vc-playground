import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/auth.store';
import { credentialsStore } from '$lib/stores/credentials.store';
import { popupCenter } from '$lib/utils/login-popup.utils';
import type { ToastStore } from '@skeletonlabs/skeleton';
import { AuthClient } from '@dfinity/auth-client';

let cachedClient: AuthClient | undefined = undefined;
const getAuthClient = async () => {
  if (!cachedClient) {
    cachedClient = await AuthClient.create();
  }
  return cachedClient;
};

const resetCachedAuthClient = () => {
  cachedClient = undefined;
};

export const login = async ({ toastStore, cb }: { toastStore: ToastStore; cb?: () => void }) => {
  // This service never fails. It will manage the error handling internally.
  try {
    const authClient = await getAuthClient();
    return new Promise<void>((resolve) => {
      authClient.login({
        onSuccess: async () => {
          const identity = authClient.getIdentity();
          authStore.set({ identity });
          resolve();
          if (cb !== undefined) {
            cb();
          }
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
        windowOpenerFeatures: popupCenter(),
        identityProvider: import.meta.env.VITE_INTERNET_IDENTITY_URL,
        derivationOrigin: import.meta.env.VITE_RP_DERIVATION_ORIGIN,
        // One week
        maxTimeToLive: 7n * 24n * 3_600_000_000_000n,
      });
    });
  } catch (err) {
    toastStore.trigger({
      message: `An error occurred while trying to login. ${err}`,
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
    credentialsStore.reset();
    resetCachedAuthClient();
    authStore.set({ identity: null });
    goto('/');
  }
};

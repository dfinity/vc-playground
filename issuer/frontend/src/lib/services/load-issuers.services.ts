import { queryGroups } from '$lib/api/queryGroups.api';
import { getIssuersStore } from '$lib/stores/issuers.store';
import { type Identity } from '@dfinity/agent';
import type { ToastStore } from '@skeletonlabs/skeleton';

export const loadIssuers = async ({
  identity,
  toastStore,
}: {
  identity: Identity;
  toastStore: ToastStore;
}) => {
  try {
    const groups = await queryGroups({ identity });
    const issuersStore = getIssuersStore(identity);
    issuersStore.set(groups);
  } catch (err: unknown) {
    toastStore.trigger({
      message: `Oops! There was an error while trying to load issuers. Please try again. ${err}`,
      background: 'variant-filled-error',
    });
  }
};

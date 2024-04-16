import { queryGroup } from '$lib/api/queryGroup.api';
import { getIssuerDetailStore } from '$lib/stores/issuer-detail.store';
import { type Identity } from '@dfinity/agent';
import type { ToastStore } from '@skeletonlabs/skeleton';

export const loadIssuerDetail = async ({
  identity,
  issuerName,
  toastStore,
}: {
  identity: Identity;
  issuerName: string;
  toastStore: ToastStore;
}) => {
  try {
    const group = await queryGroup({ identity, groupName: issuerName });
    const issuersStore = getIssuerDetailStore({ identity, issuerName });
    issuersStore.set(group);
  } catch (err: unknown) {
    toastStore.trigger({
      message: `Oops! There was an error while trying to load issuer data. Please try again. ${err}`,
      background: 'variant-filled-error',
    });
  }
};

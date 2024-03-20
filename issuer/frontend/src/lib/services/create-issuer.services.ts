import { addGroup } from '$lib/api/addGroup.api';
import { isNullish } from '$lib/utils/is-nullish.utils';
import type { Identity } from '@dfinity/agent';
import { loadIssuers } from './load-issuers.services';
import { isProfane } from 'no-profanity';
import type { ToastStore } from '@skeletonlabs/skeleton';
import { NO_IDENTITY_MESSAGE, PROFANITY_MESSAGE } from '$lib/constants/messages';

export const createIssuer = async ({
  identity,
  issuerName,
  toastStore,
}: {
  identity: Identity | null | undefined;
  issuerName: string;
  toastStore: ToastStore;
}) => {
  try {
    if (isNullish(identity)) {
      throw new Error(NO_IDENTITY_MESSAGE);
    }
    if (isProfane(issuerName)) {
      throw new Error(PROFANITY_MESSAGE);
    }
    await addGroup({ identity, issuerName });
    await loadIssuers({ identity });
  } catch (err: unknown) {
    console.error(err);
    toastStore.trigger({
      message: (err as Error).message ?? 'There was an error creating the issuer',
      background: 'variant-filled-error',
    });
  }
};

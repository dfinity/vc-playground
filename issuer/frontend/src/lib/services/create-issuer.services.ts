import { addGroup } from '$lib/api/addGroup.api';
import { isNullish } from '$lib/utils/is-nullish.utils';
import type { Identity } from '@dfinity/agent';
import { loadIssuers } from './load-issuers.services';
import type { ToastStore } from '@skeletonlabs/skeleton';
import { NO_IDENTITY_MESSAGE } from '$lib/constants/messages';
import { validateText } from '$lib/utils/validate-text.utils';

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
    validateText(issuerName);
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

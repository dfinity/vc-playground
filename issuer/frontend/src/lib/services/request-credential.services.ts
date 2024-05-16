import { isNullish } from '$lib/utils/is-nullish.utils';
import type { Identity } from '@dfinity/agent';
import { loadIssuers } from './load-issuers.services';
import { joinGroup } from '$lib/api/joinGroup.api';
import type { ToastStore } from '@skeletonlabs/skeleton';
import { NO_IDENTITY_MESSAGE } from '$lib/constants/messages';
import type { Principal } from '@dfinity/principal';
import { validateText } from '$lib/utils/validate-text.utils';

export const requestCredential = async ({
  identity,
  issuerName,
  owner,
  toastStore,
  memberData,
}: {
  identity: Identity | null | undefined;
  issuerName: string;
  owner: Principal;
  toastStore: ToastStore;
  memberData?: string | number;
}) => {
  try {
    if (memberData && typeof memberData === 'string') {
      validateText(memberData);
    }
    if (isNullish(identity)) {
      throw new Error(NO_IDENTITY_MESSAGE);
    }
    await joinGroup({ identity, issuerName, owner });
    await loadIssuers({ identity, toastStore });
  } catch (err: unknown) {
    console.error(err);
    toastStore.trigger({
      message: (err as Error).message ?? 'There was an error requesting the credential.',
      background: 'variant-filled-error',
    });
  }
};

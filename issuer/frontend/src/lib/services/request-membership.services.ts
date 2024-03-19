import { isNullish } from '$lib/utils/is-nullish.utils';
import type { Identity } from '@dfinity/agent';
import { loadIssuers } from './load-issuers.services';
import { joinGroup } from '$lib/api/joinGroup.api';

export const requestMembership = async ({
  identity,
  issuerName,
  note,
}: {
  identity: Identity | null | undefined;
  issuerName: string;
  note: string;
}) => {
  try {
    if (isNullish(identity)) {
      throw new Error('Identity not found');
    }
    await joinGroup({ identity, issuerName, note });
    await loadIssuers({ identity });
  } catch (err: unknown) {
    console.log('error creating issuer');
    console.error(err);
    // TODO: Handle error
  }
};

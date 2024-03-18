import { addGroup } from '$lib/api/addGroup.api';
import { isNullish } from '$lib/utils/is-nullish.utils';
import type { Identity } from '@dfinity/agent';
import { loadIssuers } from './load-issuers.services';

export const createIssuer = async ({
  identity,
  issuerName,
}: {
  identity: Identity | null | undefined;
  issuerName: string;
}) => {
  try {
    if (isNullish(identity)) {
      throw new Error('Identity not found');
    }
    await addGroup({ identity, issuerName });
    await loadIssuers({ identity });
  } catch (err: unknown) {
    console.log('error creating issuer');
    console.error(err);
    // TODO: Handle error
  }
};

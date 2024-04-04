import { getMetaIssuerCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';

export const joinGroup = async ({
  identity,
  issuerName,
  owner,
}: {
  identity: Identity;
  issuerName: string;
  owner: Principal;
}): Promise<void> => {
  const canister = await getMetaIssuerCanister(identity);
  const response = await canister.join_group({ owner, group_name: issuerName });
  if ('Err' in response) {
    throw response.Err;
  }
};

import { getMetaIssuerCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';

export const joinGroup = async ({
  identity,
  issuerName,
  note,
}: {
  identity: Identity;
  issuerName: string;
  note: string;
}): Promise<void> => {
  const canister = await getMetaIssuerCanister(identity);
  const response = await canister.join_group({ note: note, group_name: issuerName });
  if ('Err' in response) {
    throw response.Err;
  }
};

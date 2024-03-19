import { getMetaIssuerCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';
import type { FullGroupData } from '../../declarations/meta_issuer.did';

export const queryGroup = async ({
  identity,
  groupName,
}: {
  identity: Identity;
  groupName: string;
}): Promise<FullGroupData> => {
  const canister = await getMetaIssuerCanister(identity);
  const response = await canister.get_group({ group_name: groupName });
  if ('Ok' in response) {
    return response.Ok;
  }
  throw response.Err;
};

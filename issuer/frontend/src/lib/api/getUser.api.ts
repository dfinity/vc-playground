import { getMetaIssuerCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';
import type { UserData } from '../../declarations/meta_issuer.did';

export const getUser = async ({ identity }: { identity: Identity }): Promise<UserData> => {
  const canister = await getMetaIssuerCanister(identity);
  const response = await canister.get_user();
  if ('Err' in response) {
    throw response.Err;
  }
  return response.Ok;
};

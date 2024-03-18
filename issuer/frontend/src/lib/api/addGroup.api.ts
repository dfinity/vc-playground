import { getMetaIssuerCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';
import type { FullGroupData } from '../../declarations/meta_issuer.did';

export const addGroup = async (params: {
  identity: Identity;
  issuerName: string;
}): Promise<FullGroupData> => {
  const canister = await getMetaIssuerCanister(params.identity);
  const response = await canister.add_group({ group_name: params.issuerName });
  if ('Ok' in response) {
    return response.Ok;
  }
  throw response.Err;
};

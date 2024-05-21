import { getMetaIssuerCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';
import type { GroupType } from '../../declarations/meta_issuer/meta_issuer.did';

export const queryGroupTypes = async ({
  identity,
}: {
  identity: Identity;
}): Promise<GroupType[]> => {
  const actor = await getMetaIssuerCanister(identity);
  const response = await actor.group_types();
  if ('Ok' in response) {
    return response.Ok.types;
  }
  throw response.Err;
};

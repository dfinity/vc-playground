import { getMetaIssuerCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';
import type { PublicGroupData } from '../../declarations/meta_issuer.did';

export const queryGroups = async ({
  identity,
}: {
  identity: Identity;
}): Promise<PublicGroupData[]> => {
  const actor = await getMetaIssuerCanister(identity);
  const response = await actor.list_groups({ only_owned: [], group_name_substring: [] });
  if ('Ok' in response) {
    return response.Ok.groups;
  }
  throw response.Err;
};

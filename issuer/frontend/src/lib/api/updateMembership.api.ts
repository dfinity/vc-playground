import { getMetaIssuerCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';
import type { MembershipUpdate } from '../../declarations/meta_issuer.did';

export const updateMembership = async ({
  identity,
  groupName,
  updates,
}: {
  identity: Identity;
  groupName: string;
  updates: MembershipUpdate[];
}): Promise<void> => {
  const canister = await getMetaIssuerCanister(identity);
  const response = await canister.update_membership({ group_name: groupName, updates });
  if ('Err' in response) {
    throw response.Err;
  }
};

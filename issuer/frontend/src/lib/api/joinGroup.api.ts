import { getMetaIssuerCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';
import type { VcArguments } from '../../declarations/meta_issuer.did';

export const joinGroup = async ({
  identity,
  issuerName,
  owner,
  vcArguments,
}: {
  identity: Identity;
  issuerName: string;
  owner: Principal;
  vcArguments?: VcArguments;
}): Promise<void> => {
  const canister = await getMetaIssuerCanister(identity);
  const response = await canister.join_group({
    owner,
    group_name: issuerName,
    vc_arguments: vcArguments ? [vcArguments] : [],
  });
  if ('Err' in response) {
    throw response.Err;
  }
};

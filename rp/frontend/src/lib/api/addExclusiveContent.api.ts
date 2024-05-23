import type { ContentData, CredentialSpec } from '../../declarations/rp/rp.did';
import type { Identity } from '@dfinity/agent';
import { getRpCanister } from '$lib/utils/actor.utils';
import type { Principal } from '@dfinity/principal';

export const addExclusiveContent = async ({
  url,
  credentialSpec,
  owner,
  contentName,
  identity,
}: {
  url: string;
  credentialSpec: CredentialSpec;
  owner: Principal;
  contentName: string;
  identity: Identity;
}): Promise<ContentData> => {
  const actor = await getRpCanister(identity);
  const response = await actor.add_exclusive_content({
    url,
    content_name: contentName,
    credential_spec: credentialSpec,
    credential_issuer: owner,
  });
  if ('Ok' in response) {
    return response.Ok;
  }
  throw response.Err;
};

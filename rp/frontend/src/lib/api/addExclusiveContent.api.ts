import type { ContentData } from '../../declarations/rp/rp.did';
import type { Identity } from '@dfinity/agent';
import { getRpCanister } from '$lib/utils/actor.utils';
import type { Principal } from '@dfinity/principal';

export const addExclusiveContent = async ({
  url,
  issuerName,
  owner,
  contentName,
  identity,
}: {
  url: string;
  issuerName: string;
  owner: Principal;
  contentName: string;
  identity: Identity;
}): Promise<ContentData> => {
  const actor = await getRpCanister(identity);
  const response = await actor.add_exclusive_content({
    url,
    content_name: contentName,
    credential_group_name: issuerName,
    credential_group_owner: owner,
  });
  if ('Ok' in response) {
    return response.Ok;
  }
  throw response.Err;
};

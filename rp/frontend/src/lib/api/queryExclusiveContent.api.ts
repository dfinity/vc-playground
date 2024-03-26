import type { Identity } from '@dfinity/agent';
import type { ExclusiveContentList } from '../../declarations/rp/rp.did';
import { getRpCanister } from '$lib/utils/actor.utils';

export const queryExclusiveContent = async ({
  identity,
}: {
  identity: Identity;
}): Promise<ExclusiveContentList> => {
  const actor = await getRpCanister(identity);
  const response = await actor.list_exclusive_content({ owned_by: [] });
  console.log('in da queryExclusiveContent', response);
  if ('Ok' in response) {
    return response.Ok;
  }
  throw response.Err;
};

import { getMetaIssuerCanister } from '$lib/utils/actor.utils';
import type { Identity } from '@dfinity/agent';
import { toNullable } from '$lib/utils/to-nullable.utils';

export const setUser = async ({
  identity,
  issuerNickname,
  userNickname,
}: {
  identity: Identity;
  issuerNickname?: string;
  userNickname?: string;
}): Promise<void> => {
  const canister = await getMetaIssuerCanister(identity);
  const response = await canister.set_user({
    user_data: {
      user_nickname: toNullable(userNickname),
      issuer_nickname: toNullable(issuerNickname),
    },
  });
  if ('Err' in response) {
    throw response.Err;
  }
};

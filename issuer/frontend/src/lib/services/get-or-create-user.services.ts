import type { Identity } from '@dfinity/agent';
import type { UserData } from '../../declarations/meta_issuer.did';
import { getUser } from '$lib/api/getUser.api';
import { setUser } from '$lib/api/setUser.api';

export const getOrCreateUser = async ({ identity }: { identity: Identity }): Promise<UserData> => {
  try {
    return await getUser({ identity });
  } catch (error: unknown) {
    if (typeof error === 'object' && error !== null && 'NotFound' in error) {
      await setUser({ identity });
      return await getUser({ identity });
    }
    throw error;
  }
};

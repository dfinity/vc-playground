import { queryGroups } from '$lib/api/queryGroups.api';
import { getIssuersStore } from '$lib/stores/issuers.store';
import { type Identity } from '@dfinity/agent';

export const loadIssuers = async ({ identity }: { identity: Identity }) => {
  try {
    const groups = await queryGroups({ identity });
    const issuersStore = getIssuersStore(identity);
    issuersStore.set(groups);
  } catch (err: unknown) {
    // TODO: Handle error
  }
};

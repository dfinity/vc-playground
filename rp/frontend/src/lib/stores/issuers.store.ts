import { writable, type Writable } from 'svelte/store';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { queryGroups } from '$lib/api/queryGroups.api';
import { browser } from '$app/environment';
import type { PublicGroupData } from '../../declarations/meta_issuer/meta_issuer.did';

let issuersStore: Writable<PublicGroupData[] | undefined> | undefined = undefined;
export const getIssuersStore = (
  identity: Identity | undefined | null
): Writable<PublicGroupData[] | undefined> => {
  if (!issuersStore) {
    issuersStore = writable<PublicGroupData[] | undefined>(undefined, (_set, update) => {
      if (browser) {
        queryGroups({ identity: identity ?? new AnonymousIdentity() }).then((groups) => {
          update(() => groups);
        });
      }
    });
  }
  return issuersStore;
};

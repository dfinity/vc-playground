import { derived, writable, type Readable } from 'svelte/store';
import { AnonymousIdentity } from '@dfinity/agent';
import { queryGroups } from '$lib/api/queryGroups.api';
import { browser } from '$app/environment';
import type { PublicGroupData } from '../../declarations/meta_issuer/meta_issuer.did';
import type { Principal } from '@dfinity/principal';

export const groupsStore = writable<PublicGroupData[] | undefined>(undefined, (_set, update) => {
  if (browser) {
    queryGroups({ identity: new AnonymousIdentity() }).then((groups) => {
      update(() => groups);
    });
  }
});

export const credentialsTypesStore: Readable<string[]> = derived(groupsStore, (issuers) =>
  Array.from(new Set(issuers?.map((issuer) => issuer.group_name)).values())
);

export type Issuer = { nickname: string; owner: Principal };
// Object with credential type as key and array of issuers as value.
export type IssuersByCredential = Record<string, Issuer[]>;
export const issuersStore: Readable<IssuersByCredential> = derived(groupsStore, (issuers) =>
  Array.from(new Set(issuers?.map((issuer) => issuer.group_name)).values()).reduce(
    (acc, groupName) => ({
      ...acc,
      [groupName]: issuers
        ?.filter((issuer) => issuer.group_name === groupName)
        .map((issuer) => ({
          nickname: issuer.issuer_nickname,
          owner: issuer.owner,
        })),
    }),
    {}
  )
);

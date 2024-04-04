import { derived, writable, type Readable } from 'svelte/store';
import { AnonymousIdentity } from '@dfinity/agent';
import { queryGroups } from '$lib/api/queryGroups.api';
import { browser } from '$app/environment';
import type { PublicGroupData } from '../../declarations/meta_issuer/meta_issuer.did';
import type { Principal } from '@dfinity/principal';

const groupsStore = writable<PublicGroupData[] | undefined>(undefined, (_set, update) => {
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
export const issuersStore: Readable<Issuer[]> = derived(groupsStore, (issuers) =>
  Array.from(new Set(issuers?.map((issuer) => issuer.issuer_nickname)).values()).map(
    (nickname) => ({
      nickname,
      owner: issuers?.find((issuer) => issuer.issuer_nickname === nickname)?.owner as Principal,
    })
  )
);

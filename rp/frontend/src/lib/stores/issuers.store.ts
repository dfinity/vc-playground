import { writable, type Writable } from 'svelte/store';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { queryGroups } from '$lib/api/queryGroups.api';
import { browser } from '$app/environment';
import type { PublicGroupData } from '../../declarations/meta_issuer/meta_issuer.did';

const issuers: Record<string, Writable<PublicGroupData[] | undefined>> = {};
export const getIssuersStore = (
  identity: Identity | undefined | null
): Writable<PublicGroupData[] | undefined> => {
  const identityPrincipal = identity?.getPrincipal().toText() ?? 'no-authenticated-identity';
  if (!issuers[identityPrincipal]) {
    issuers[identityPrincipal] = writable<PublicGroupData[] | undefined>(
      undefined,
      (_set, update) => {
        if (browser) {
          queryGroups({ identity: identity ?? new AnonymousIdentity() }).then((groups) => {
            update(() => groups);
          });
        }
      }
    );
  }
  return issuers[identityPrincipal];
};

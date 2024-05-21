import { derived, writable, type Writable } from 'svelte/store';
import type { GroupType } from '../../declarations/meta_issuer.did';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { browser } from '$app/environment';
import { queryGroupTypes } from '$lib/api/queryGroupTypes.api';

const issuerTypes: Record<string, Writable<GroupType[] | undefined>> = {};
export const getIssuerTypesStore = (
  identity: Identity | undefined | null
): Writable<GroupType[] | undefined> => {
  const identityPrincipal = identity?.getPrincipal().toText() ?? 'no-authenticated-identity';
  if (!issuerTypes[identityPrincipal]) {
    issuerTypes[identityPrincipal] = writable<GroupType[] | undefined>(
      undefined,
      (_set, update) => {
        if (browser) {
          queryGroupTypes({ identity: identity ?? new AnonymousIdentity() }).then((groups) => {
            update(() => groups);
          });
        }
      }
    );
  }
  return issuerTypes[identityPrincipal];
};

export const getAllIssuerTypesStore = (identity: Identity | undefined | null) =>
  derived(getIssuerTypesStore(identity), (issuerTypes) => issuerTypes);

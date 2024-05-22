import { derived, writable, type Readable, type Writable } from 'svelte/store';
import type { GroupType } from '../../declarations/meta_issuer/meta_issuer.did';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { browser } from '$app/environment';
import { queryGroupTypes } from '$lib/api/queryGroupTypes.api';
import type { CredentialSpec } from '../../declarations/rp/rp.did';

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

// Record from group_name to input type
type IssuerCredentialSpecStoreData = Record<string, CredentialSpec>;
export type IssuerCredentialSpecStore = Readable<IssuerCredentialSpecStoreData>;
export const getIssuerCredentialSpecsStore = (
  identity: Identity | undefined | null
): IssuerCredentialSpecStore =>
  derived(getIssuerTypesStore(identity), (issuerTypes) => {
    const inputTypes: IssuerCredentialSpecStoreData = {};
    for (const groupType of issuerTypes ?? []) {
      inputTypes[groupType.group_name] = groupType.credential_spec;
    }
    return inputTypes;
  });

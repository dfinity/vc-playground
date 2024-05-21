import { derived, writable, type Readable, type Writable } from 'svelte/store';
import type { GroupType } from '../../declarations/meta_issuer/meta_issuer.did';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { browser } from '$app/environment';
import { queryGroupTypes } from '$lib/api/queryGroupTypes.api';
import { inputTypeCredentialSpec } from '$lib/utils/input-type-credential-spec.utils';

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
type IssuerInputTypeStoreData = Record<string, 'text' | 'number' | undefined>;
export type IssuerInputTypeStore = Readable<IssuerInputTypeStoreData>;
export const getIssuerInputTypesStore = (
  identity: Identity | undefined | null
): IssuerInputTypeStore =>
  derived(getIssuerTypesStore(identity), (issuerTypes) => {
    const inputTypes: IssuerInputTypeStoreData = {};
    for (const groupType of issuerTypes ?? []) {
      inputTypes[groupType.group_name] = inputTypeCredentialSpec(groupType.credential_spec);
    }
    return inputTypes;
  });

import { derived, writable, type Readable, type Writable } from 'svelte/store';
import type { PublicGroupData } from '../../declarations/meta_issuer.did';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { queryGroups } from '$lib/api/queryGroups.api';

const issuers: Record<string, Writable<PublicGroupData[] | undefined>> = {};
export const getIssuersStore = (
  identity: Identity | undefined | null
): Writable<PublicGroupData[] | undefined> => {
  const identityPrincipal = identity?.getPrincipal().toText() ?? 'no-authenticated-identity';
  if (!issuers[identityPrincipal]) {
    issuers[identityPrincipal] = writable<PublicGroupData[] | undefined>(
      undefined,
      (_set, update) => {
        queryGroups({ identity: identity ?? new AnonymousIdentity() }).then((groups) => {
          update(() => groups);
        });
      }
    );
  }
  return issuers[identityPrincipal];
};

export const getAllIssuersStore = (
  identity: Identity | undefined | null
): Readable<PublicGroupData[] | undefined> =>
  derived(getIssuersStore(identity), (issuers) => issuers);

const isIdentityCredential = ({ is_owner, membership_status }: PublicGroupData): boolean => {
  if (is_owner[0]) {
    return true;
  }
  const status = membership_status[0];
  if (status === undefined || 'Rejected' in status) {
    return false;
  }
  if ('PendingReview' in status || 'Accepted' in status) {
    return true;
  }
  throw new Error('Unexpected membership status');
};
export const getCredentialsStore = (
  identity: Identity | undefined | null
): Readable<PublicGroupData[] | undefined> =>
  derived(getAllIssuersStore(identity), (issuers) => issuers?.filter(isIdentityCredential));

const isOwner = ({ is_owner }: PublicGroupData): boolean => Boolean(is_owner[0]);
export const getIdentityIssuersStore = (
  identity: Identity | undefined | null
): Readable<PublicGroupData[] | undefined> =>
  derived(getAllIssuersStore(identity), (issuers) => issuers?.filter(isOwner));

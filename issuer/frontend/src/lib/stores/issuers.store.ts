import { derived, writable, type Readable, type Writable } from 'svelte/store';
import type { PublicGroupData } from '../../declarations/meta_issuer.did';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { queryGroups } from '$lib/api/queryGroups.api';
import { browser } from '$app/environment';

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

const sortCredentialsPerTimestampDescending = (a: PublicGroupData, b: PublicGroupData): number =>
  Number(b.stats.created_timestamp_ns - a.stats.created_timestamp_ns);
const sortCredentialsPerType = (a: PublicGroupData, b: PublicGroupData): number => {
  const statusA = a.membership_status[0];
  const statusB = b.membership_status[0];
  if (statusA === undefined || 'Rejected' in statusA) {
    if (statusB === undefined || 'Rejected' in statusB) {
      return sortCredentialsPerTimestampDescending(a, b);
    }
    return 1;
  }
  if (statusB === undefined || 'Rejected' in statusB) {
    return -1;
  }

  if ('Accepted' in statusA) {
    if ('Accepted' in statusB) {
      return sortCredentialsPerTimestampDescending(a, b);
    }
    return -1;
  }
  // Missing only that both are "PendingApproval"
  return sortCredentialsPerTimestampDescending(a, b);
};
export const getAllIssuersStore = (
  identity: Identity | undefined | null
): Readable<PublicGroupData[] | undefined> =>
  derived(getIssuersStore(identity), (issuers) => issuers?.sort(sortCredentialsPerType));

const isOwner =
  (identity: Identity | undefined | null) =>
  ({ owner }: PublicGroupData): boolean =>
    identity?.getPrincipal().compareTo(owner) === 'eq';
export const getIdentityIssuersStore = (
  identity: Identity | undefined | null
): Readable<PublicGroupData[] | undefined> =>
  derived(getAllIssuersStore(identity), (issuers) => issuers?.filter(isOwner(identity)));

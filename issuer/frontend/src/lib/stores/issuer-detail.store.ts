import { derived, writable, type Readable, type Writable } from 'svelte/store';
import type {
  FullGroupData,
  MemberData,
  MembershipStatus,
} from '../../declarations/meta_issuer.did';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { queryGroup } from '$lib/api/queryGroup.api';
import { isNullish } from '$lib/utils/is-nullish.utils';
import { browser } from '$app/environment';

export type IssuerDetailStore = Writable<FullGroupData | undefined | null>;
/**
 * Record<identity-issuer_name, Writable<FullGroupData | undefined>>
 */
const issuers: Record<string, IssuerDetailStore> = {};
export const getIssuerDetailStore = ({
  identity,
  issuerName,
}: {
  identity: Identity | undefined | null;
  issuerName: string;
}): IssuerDetailStore => {
  const identityPrincipal = identity?.getPrincipal().toText() ?? 'no-authenticated-identity';
  const key = `${identityPrincipal}-${issuerName}`;
  if (!issuers[key]) {
    issuers[key] = writable<FullGroupData | undefined | null>(undefined, (_set, update) => {
      // No need to query if we don't have the identity
      if (isNullish(identity) || !browser) {
        return;
      }
      queryGroup({ identity: identity ?? new AnonymousIdentity(), groupName: issuerName })
        .then((group) => {
          update(() => group);
        })
        .catch((error) => {
          console.error('Error fetching issuer detail', error);
          update((currentData) => {
            if (currentData === undefined) {
              return null;
            }
            return currentData;
          });
        });
    });
  }
  return issuers[key];
};

type IssuerStatus = 'Accepted' | 'Rejected' | 'PendingReview';
const getStatusKey = (status: MembershipStatus): 'Accepted' | 'Rejected' | 'PendingReview' => {
  if ('Accepted' in status) {
    return 'Accepted';
  }
  if ('Rejected' in status) {
    return 'Rejected';
  }
  return 'PendingReview';
};
/**
 * Sort members:
 *
 * 1. Status
 *  a. PendingReview
 *  b. Accepted
 * 2 Joined date descending
 * @param members
 */
const sortMembers = (a: MemberData, b: MemberData): number => {
  const statusOrder: Record<IssuerStatus, number> = {
    PendingReview: 0,
    Accepted: 1,
    Rejected: 2,
  };
  const aStatus = getStatusKey(a.membership_status);
  const bStatus = getStatusKey(b.membership_status);
  if (aStatus === bStatus) {
    return Number(b.joined_timestamp_ns - a.joined_timestamp_ns);
  }
  return statusOrder[aStatus] - statusOrder[bStatus];
};
export const getIssuerNonRevokedMembers = ({
  identity,
  issuerName,
}: {
  identity: Identity | undefined | null;
  issuerName: string;
}): Readable<MemberData[] | undefined> =>
  derived(getIssuerDetailStore({ identity, issuerName }), (issuerFullData) => {
    if (isNullish(issuerFullData)) {
      return undefined;
    }
    console.log('issuerFullData', issuerFullData.members);
    return issuerFullData?.members
      .filter((member) => !('Rejected' in member.membership_status))
      .sort(sortMembers);
  });

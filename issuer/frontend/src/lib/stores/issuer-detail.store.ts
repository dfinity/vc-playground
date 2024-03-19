import { derived, writable, type Readable, type Writable } from 'svelte/store';
import type { FullGroupData, MemberData } from '../../declarations/meta_issuer.did';
import { AnonymousIdentity, type Identity } from '@dfinity/agent';
import { queryGroup } from '$lib/api/queryGroup.api';
import { isNullish } from '$lib/utils/is-nullish.utils';

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
      if (isNullish(identity)) {
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
    return issuerFullData?.members.filter((member) => !('Rejected' in member.membership_status));
  });

import { updateMembership } from '$lib/api/updateMembership.api';
import type { Identity } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';
import { loadIssuerDetail } from './load-issuer-detail.services';
import type { MembershipStatus } from '../../declarations/meta_issuer.did';

const updaterFactory =
  (newStatus: MembershipStatus) =>
  async ({
    identity,
    issuerName,
    member,
  }: {
    issuerName: string;
    identity: Identity | null | undefined;
    member: Principal;
  }) => {
    try {
      if (!identity) {
        throw new Error('No identity');
      }
      await updateMembership({
        identity,
        groupName: issuerName,
        updates: [{ member, new_status: newStatus }],
      });
      await loadIssuerDetail({ identity, issuerName });
    } catch (e) {
      // handle error
      console.error('Error accepting credential.', e);
    }
  };

export const acceptCredential = updaterFactory({ Accepted: null });
export const revokeCredential = updaterFactory({ Rejected: null });

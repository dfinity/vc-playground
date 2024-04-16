import { updateMembership } from '$lib/api/updateMembership.api';
import type { Identity } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';
import { loadIssuerDetail } from './load-issuer-detail.services';
import type { MembershipStatus } from '../../declarations/meta_issuer.did';
import { NO_IDENTITY_MESSAGE } from '$lib/constants/messages';
import type { ToastStore } from '@skeletonlabs/skeleton';

const updaterFactory =
  (newStatus: MembershipStatus) =>
  async ({
    identity,
    issuerName,
    member,
    toastStore,
  }: {
    issuerName: string;
    identity: Identity | null | undefined;
    member: Principal;
    toastStore: ToastStore;
  }) => {
    try {
      if (!identity) {
        throw new Error(NO_IDENTITY_MESSAGE);
      }
      await updateMembership({
        identity,
        groupName: issuerName,
        updates: [{ member, new_status: newStatus }],
      });
      await loadIssuerDetail({ identity, issuerName, toastStore });
    } catch (e) {
      console.error('Error accepting credential.', e);
      toastStore.trigger({
        message: `Oops! There was an error while trying to update the credential. Please try again. ${e}`,
        background: 'variant-filled-error',
      });
    }
  };

export const acceptCredential = updaterFactory({ Accepted: null });
export const revokeCredential = updaterFactory({ Rejected: null });

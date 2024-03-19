import type { MemberData } from '../../declarations/meta_issuer.did';

const countStatusCredentials =
  (statusKey: 'Accepted' | 'Rejected' | 'PendingReview') =>
  (members: MemberData[]): number =>
    members.filter(({ membership_status }) => statusKey in membership_status).length;

export const countApprovedCredentials = countStatusCredentials('Accepted');
export const countPendingCredentials = countStatusCredentials('PendingReview');

import type { ModalComponent } from '@skeletonlabs/skeleton';
import MemberModal from './MemberModal.svelte';
import PendingMemberModal from './PendingMemberModal.svelte';

export const modalRegistry: Record<string, ModalComponent> = {
  memberModal: { ref: MemberModal },
  pendingMemberModal: { ref: PendingMemberModal },
};

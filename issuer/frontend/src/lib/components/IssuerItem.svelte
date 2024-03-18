<script lang="ts">
  import { goto } from '$app/navigation';
  import Badge from '$lib/ui-components/elements/Badge.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
  import type { MembershipStatus, PublicGroupData } from '../../declarations/meta_issuer.did';
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';

  export let issuer: PublicGroupData;
  // Must be invoked at the top level: https://www.skeleton.dev/utilities/modals
  const modalStore = getModalStore();

  let canJoin: boolean;
  $: canJoin = issuer.membership_status.length === 0 || 'Rejected' in issuer.membership_status[0];

  const statusVariant = (status: MembershipStatus | undefined): 'success' | 'default' => {
    if (status === undefined || 'Rejected' in status) {
      throw new Error('It should not show a badge');
    }
    if ('Accepted' in status) return 'success';
    // Only missing 'PendingReview'
    return 'default';
  };
  const badgeText = (status: MembershipStatus | undefined): string => {
    if (status === undefined || 'Rejected' in status) {
      throw new Error('It should not show a badge');
    }
    if ('Accepted' in status) {
      return '🪪 Obtained';
    }
    // Only missing 'PendingReview'
    return '📤 Pending';
  };

  const createOpenModal = (modal: 'memberModal' | 'pendingMemberModal') => () => {
    const meta = { name: issuer.group_name };
    const settings: ModalSettings = { type: 'component', component: modal, meta };
    modalStore.trigger(settings);
  };

  const getOnClick = (issuer: PublicGroupData): (() => void) | undefined => {
    if (issuer.is_owner[0]) {
      return () => goto(`/issuers/${encodeURIComponent(issuer.group_name)}`);
    }
    const status = issuer.membership_status[0];
    if (status === undefined || 'Rejected' in status) {
      return undefined;
    }
    if ('Accepted' in status) return createOpenModal('memberModal');
    // Only missing 'PendingReview'
    return createOpenModal('pendingMemberModal');
  };

  let onClick: (() => void) | undefined;
  $: onClick = getOnClick(issuer);
</script>

<ListItem {onClick}>
  <svelte:fragment slot="main">{issuer.group_name}</svelte:fragment>
  <svelte:fragment slot="end">
    {#if issuer.is_owner[0]}
      <Badge variant="primary">👑 Owner</Badge>
    {:else if canJoin}
      <Button variant="primary" size="sm">Request Credential</Button>
    {:else}
      <Badge variant={statusVariant(issuer.membership_status[0])}
        >{badgeText(issuer.membership_status[0])}</Badge
      >
    {/if}
  </svelte:fragment>
</ListItem>

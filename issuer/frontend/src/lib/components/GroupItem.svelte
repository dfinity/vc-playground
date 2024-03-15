<script lang="ts">
  import { goto } from '$app/navigation';
  import AvatarSize from '$lib/ui-components/elements/AvatarSize.svelte';
  import Badge from '$lib/ui-components/elements/Badge.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
  import type { MembershipStatus, PublicGroupData } from '../../declarations/meta_issuer.did';
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';

  export let group: PublicGroupData;
  // Must be invoked at the top level: https://www.skeleton.dev/utilities/modals
  const modalStore = getModalStore();

  let canJoin: boolean;
  $: canJoin = group.membership_status.length === 0 || 'Rejected' in group.membership_status[0];

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
    if ('Accepted' in status) return '🪪 Member';
    // Only missing 'PendingReview'
    return '📤 Pending';
  };

  const createOpenModal = (modal: 'memberModal' | 'pendingMemberModal') => () => {
    const meta = { name: group.group_name };
    const settings: ModalSettings = { type: 'component', component: modal, meta };
    modalStore.trigger(settings);
  };

  const getOnClick = (group: PublicGroupData): (() => void) | undefined => {
    if (group.is_owner[0]) {
      // TODO: Change empty space for "-" in the URL
      return () => goto(`/groups/${group.group_name}`);
    }
    const status = group.membership_status[0];
    if (status === undefined || 'Rejected' in status) {
      return undefined;
    }
    if ('Accepted' in status) return createOpenModal('memberModal');
    // Only missing 'PendingReview'
    return createOpenModal('pendingMemberModal');
  };

  let onClick: (() => void) | undefined;
  $: onClick = getOnClick(group);
</script>

<ListItem {onClick}>
  <AvatarSize num={group.stats.member_count} slot="start" />
  <svelte:fragment slot="main">{group.group_name}</svelte:fragment>
  <svelte:fragment slot="end">
    {#if group.is_owner[0]}
      <Badge variant="primary">👑 Owner</Badge>
    {:else if canJoin}
      <Button variant="primary" size="sm">Join</Button>
    {:else}
      <Badge variant={statusVariant(group.membership_status[0])}
        >{badgeText(group.membership_status[0])}</Badge
      >
    {/if}
  </svelte:fragment>
</ListItem>

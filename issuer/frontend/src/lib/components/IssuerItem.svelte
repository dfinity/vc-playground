<script lang="ts">
  import { goto } from '$app/navigation';
  import { requestMembership } from '$lib/services/request-membership.services';
  import { authStore } from '$lib/stores/auth.store';
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

  const openMemberModal = () => {
    const settings: ModalSettings = {
      type: 'confirm',
      title: 'Test Your Credential On the Relying Party',
      body: `You have a credential for ${issuer.group_name}. Visit Xxxxx to view content that's only accessible to users with the credential of ${issuer.group_name}.`,
      buttonTextConfirm: 'Test on Relying Party',
      buttonTextCancel: 'Close',
      response: (go: boolean) => {
        if (go) {
          window.open('https://www.skeleton.dev/', '_blank');
        }
      },
    };
    modalStore.trigger(settings);
  };

  const openPendingMemberModal = () => {
    const settings: ModalSettings = {
      type: 'alert',
      title: `The ${issuer.group_name} credential was not yet issued.`,
      body: 'Wait for the issuer to issue your credential.',
      buttonTextCancel: 'Close',
      response: (go: boolean) => {
        if (go) {
          window.open('https://www.skeleton.dev/', '_blank');
        }
      },
    };
    modalStore.trigger(settings);
  };

  const getOnClick = (issuer: PublicGroupData): (() => void) | undefined => {
    if (issuer.is_owner[0]) {
      return () => goto(`/issuers/?issuer=${encodeURIComponent(issuer.group_name)}`);
    }
    const status = issuer.membership_status[0];
    if (status === undefined || 'Rejected' in status) {
      return undefined;
    }
    if ('Accepted' in status) {
      return () => openMemberModal();
    }
    // Only missing 'PendingReview'
    return () => openPendingMemberModal();
  };

  const openRequestCredentialModal = () => {
    const settings: ModalSettings = {
      type: 'prompt',
      title: 'Request Credential',
      valueAttr: { type: 'text', required: true, placeholder: 'Credential Name' },
      body: `Enter a nickname to request the ${issuer.group_name} credential.`,
      buttonTextSubmit: 'Send Request',
      response: (note: string) => {
        requestMembership({
          identity: $authStore.identity,
          issuerName: issuer.group_name,
          note,
        });
      },
    };
    modalStore.trigger(settings);
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
      <Button on:click={openRequestCredentialModal} variant="primary" size="sm"
        >Request Credential</Button
      >
    {:else}
      <Badge variant={statusVariant(issuer.membership_status[0])}
        >{badgeText(issuer.membership_status[0])}</Badge
      >
    {/if}
  </svelte:fragment>
</ListItem>

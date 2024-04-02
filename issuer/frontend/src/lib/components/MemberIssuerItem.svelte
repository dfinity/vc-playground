<script lang="ts">
  import { requestCredential } from '$lib/services/request-credential.services';
  import { authStore } from '$lib/stores/auth.store';
  import Badge from '$lib/ui-components/elements/Badge.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import type { MembershipStatus, PublicGroupData } from '../../declarations/meta_issuer.did';
  import { getModalStore, getToastStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import { RP_ORIGIN } from '$lib/constants/env-vars';
  import IssuerItem from '$lib/components/IssuerItem.svelte';

  export let issuer: PublicGroupData;
  // Must be invoked at the top level: https://www.skeleton.dev/utilities/modals
  const modalStore = getModalStore();
  const toastStore = getToastStore();

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
      body: `You have a credential for <em>${issuer.group_name}</em>. Visit Xxxxx to view content that's only accessible to users with the credential of <em>${issuer.group_name}</em>.`,
      buttonTextConfirm: 'Test on Relying Party',
      buttonTextCancel: 'Close',
      response: (go: boolean) => {
        if (go) {
          window.open(RP_ORIGIN, '_blank');
        }
      },
    };
    modalStore.trigger(settings);
  };

  const openPendingMemberModal = () => {
    const settings: ModalSettings = {
      type: 'alert',
      title: `The <em>${issuer.group_name}</em> credential was not yet issued.`,
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

  let loadingRequestCredential = false;
  const openRequestCredentialModal = () => {
    loadingRequestCredential = true;
    const settings: ModalSettings = {
      type: 'prompt',
      title: 'Request Credential',
      valueAttr: { type: 'text', required: true, placeholder: 'Nickname' },
      body: `Enter a nickname to request the <em>${issuer.group_name}</em> credential.`,
      buttonTextSubmit: 'Send Request',
      response: async (note: string) => {
        if (note) {
          await requestCredential({
            identity: $authStore.identity,
            issuerName: issuer.group_name,
            note,
            toastStore,
          });
        }
        loadingRequestCredential = false;
      },
    };
    modalStore.trigger(settings);
  };

  let onClick: (() => void) | undefined;
  $: onClick = getOnClick(issuer);
</script>

<IssuerItem {onClick} {issuer}>
  <svelte:fragment slot="end">
    {#if canJoin}
      <Button
        on:click={openRequestCredentialModal}
        variant="primary"
        size="sm"
        loading={loadingRequestCredential}>Request Credential</Button
      >
    {:else}
      <Badge variant={statusVariant(issuer.membership_status[0])}
        >{badgeText(issuer.membership_status[0])}</Badge
      >
    {/if}
  </svelte:fragment>
</IssuerItem>
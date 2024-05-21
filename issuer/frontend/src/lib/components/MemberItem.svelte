<script lang="ts">
  import { acceptCredential, revokeCredential } from '$lib/services/manage-membership.services';
  import { authStore } from '$lib/stores/auth.store';
  import Badge from '$lib/ui-components/elements/Badge.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
  import { getToastStore } from '@skeletonlabs/skeleton';
  import type { MemberData } from '../../declarations/meta_issuer.did';
  import { vcArgumentsValue } from '$lib/utils/vc-arguments-value.utils';

  export let issuerName: string;
  export let member: MemberData;

  const toastStore = getToastStore();

  let vcArgumentValue: string | number | undefined;
  $: vcArgumentValue = vcArgumentsValue(member.vc_arguments);

  let status: 'pending' | 'approved' | 'revoked';
  $: status =
    'Rejected' in member.membership_status
      ? 'revoked'
      : 'Accepted' in member.membership_status
        ? 'approved'
        : 'pending';

  let loadingAccept = false;
  const accept = async () => {
    loadingAccept = true;
    await acceptCredential({
      identity: $authStore.identity,
      issuerName,
      member: member.member,
      toastStore,
    });
    loadingAccept = false;
  };

  let loadingRevoke = false;
  const revoke = async () => {
    loadingRevoke = true;
    await revokeCredential({
      identity: $authStore.identity,
      issuerName,
      member: member.member,
      toastStore,
    });
    loadingRevoke = false;
  };
</script>

<ListItem testId={`member ${member.nickname}`}>
  <svelte:fragment slot="main">
    {member.nickname}
    {#if vcArgumentValue !== undefined}
      {` - ${vcArgumentValue}`}
    {/if}
  </svelte:fragment>
  <svelte:fragment slot="end">
    {#if status === 'pending'}
      <Button
        testId="approve-button"
        variant="success"
        on:click={accept}
        loading={loadingAccept}
        disabled={loadingRevoke}>Approve</Button
      >
      <Button variant="error" on:click={revoke} loading={loadingRevoke} disabled={loadingAccept}
        >Decline</Button
      >
    {:else if status === 'approved'}
      <Button variant="secondary" on:click={revoke} loading={loadingRevoke}>Revoke</Button>
    {:else}
      <!-- Not used at the moment because we filter them out -->
      <Badge variant="error">Revoked</Badge>
    {/if}
  </svelte:fragment>
</ListItem>

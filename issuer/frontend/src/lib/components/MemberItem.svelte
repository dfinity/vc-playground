<script lang="ts">
  import { Principal } from '@dfinity/principal';
  import { acceptCredential, revokeCredential } from '$lib/services/manage-membership.services';
  import { authStore } from '$lib/stores/auth.store';
  import Badge from '$lib/ui-components/elements/Badge.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
  import type { MemberData } from '../../declarations/meta_issuer.did';

  export let issuerName: string;
  export let member: MemberData;

  let currentUserPrincipal: Principal | undefined | null;
  $: currentUserPrincipal = $authStore.identity?.getPrincipal();

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
    });
    loadingRevoke = false;
  };
</script>

<ListItem>
  <svelte:fragment slot="main">{member.note}</svelte:fragment>
  <svelte:fragment slot="end">
    {#if status === 'pending'}
      <Button variant="success" on:click={accept} loading={loadingAccept} disabled={loadingRevoke}
        >Approve</Button
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

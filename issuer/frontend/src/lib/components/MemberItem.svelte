<script lang="ts">
  import Button from '$lib/ui-components/elements/Button.svelte';
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
  import type { MemberData } from '../../declarations/meta_issuer.did';

  export let member: MemberData;

  let status: 'pending' | 'approved' | 'revoked';
  $: status =
    'Rejected' in member.membership_status
      ? 'revoked'
      : 'Accepted' in member.membership_status
        ? 'approved'
        : 'pending';
</script>

<ListItem>
  <svelte:fragment slot="main">{member.note}</svelte:fragment>
  <svelte:fragment slot="end">
    {#if status === 'pending'}
      <Button variant="success">Approve</Button>
      <Button variant="error">Decline</Button>
    {:else}
      <Button variant="tertiary">Revoke</Button>
    {/if}
  </svelte:fragment>
</ListItem>

<script lang="ts">
  import List from '$lib/ui-components/elements/List.svelte';
  import type { PublicGroupData } from '../../declarations/meta_issuer.did';
  import IssuerItemSkeleton from './IssuerItemSkeleton.svelte';

  export let issuers: PublicGroupData[] | undefined;
  export let noGroupsMessage: string = 'No groups found';
</script>

{#if issuers === undefined}
  <List>
    <IssuerItemSkeleton />
    <IssuerItemSkeleton />
    <IssuerItemSkeleton />
  </List>
{:else if issuers?.length === 0}
  <p>{noGroupsMessage}</p>
{:else}
  <List>
    <slot />
  </List>
{/if}

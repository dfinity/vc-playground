<script lang="ts">
  import List from '$lib/ui-components/elements/List.svelte';
  import type { PublicGroupData } from '../../declarations/meta_issuer.did';
  import IssuerItem from './IssuerItem.svelte';
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
    {#each issuers as issuer}
      <IssuerItem {issuer} />
    {/each}
  </List>
{/if}

<script lang="ts">
  import List from '$lib/ui-components/elements/List.svelte';
  import type { PublicGroupData } from '../../declarations/meta_issuer.did';
  import IssuerItemSkeleton from './IssuerItemSkeleton.svelte';

  export let issuers: PublicGroupData[] | undefined;
  export let noGroupsMessage: string | undefined = undefined;
</script>

{#if issuers === undefined}
  <List>
    <IssuerItemSkeleton />
    <IssuerItemSkeleton />
    <IssuerItemSkeleton />
  </List>
{:else if issuers?.length === 0 && noGroupsMessage !== undefined}
  <p>{noGroupsMessage}</p>
{:else if issuers?.length > 0}
  <List>
    <slot />
  </List>
{/if}

<script lang="ts">
  import List from '$lib/ui-components/elements/List.svelte';
  import type { PublicGroupData } from '../../declarations/meta_issuer.did';
  import GroupItem from './GroupItem.svelte';
  import GroupItemSkeleton from './GroupItemSkeleton.svelte';

  export let groups: PublicGroupData[] | undefined;
  export let noGroupsMessage: string = 'No groups found';
</script>

{#if groups === undefined}
  <List>
    <GroupItemSkeleton />
    <GroupItemSkeleton />
    <GroupItemSkeleton />
  </List>
{:else if groups?.length === 0}
  <p>{noGroupsMessage}</p>
{:else}
  <List>
    {#each groups as group}
      <GroupItem {group} />
    {/each}
  </List>
{/if}

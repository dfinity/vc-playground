<script lang="ts">
  import ArticleWrapper from '$lib/ui-components/elements/ArticleWrapper.svelte';
  import HeadingSkeleton from '$lib/ui-components/elements/HeadingSkeleton.svelte';
  import List from '$lib/ui-components/elements/List.svelte';
  import type { MemberData } from '../../declarations/meta_issuer.did';
  import MemberItem from './MemberItem.svelte';
  import MemberItemSkeleton from './MemberItemSkeleton.svelte';

  export let issuerName: string | undefined;
  export let members: MemberData[] | undefined;
  export let title: string | undefined = undefined;
</script>

{#if members === undefined || issuerName === undefined}
  <ArticleWrapper>
    <HeadingSkeleton slot="title" size="sm" />
    <List>
      <MemberItemSkeleton />
      <MemberItemSkeleton />
      <MemberItemSkeleton />
    </List>
  </ArticleWrapper>
{:else if members.length === 0}
  <p>There are no credentials requests nor issued credentials yet.</p>
{:else}
  <ArticleWrapper>
    <svelte:fragment slot="title">{title}</svelte:fragment>
    <List>
      {#each members as member}
        <MemberItem {issuerName} {member} />
      {/each}
    </List>
  </ArticleWrapper>
{/if}

<script lang="ts">
  import ArticleWrapper from '$lib/ui-components/elements/ArticleWrapper.svelte';
  import HeadingSkeleton from '$lib/ui-components/elements/HeadingSkeleton.svelte';
  import List from '$lib/ui-components/elements/List.svelte';
  import Paragraph from '$lib/ui-components/elements/Paragraph.svelte';
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
  <Paragraph align="center">
    You have not issued any credentials of this type, and there are no pending requests.
  </Paragraph>
{:else}
  <ArticleWrapper testId="members-list">
    <svelte:fragment slot="title">{title}</svelte:fragment>
    <List>
      {#each members as member}
        <MemberItem {issuerName} {member} />
      {/each}
    </List>
  </ArticleWrapper>
{/if}

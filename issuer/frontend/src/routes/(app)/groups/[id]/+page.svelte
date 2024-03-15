<script lang="ts">
  import AuthGuard from '$lib/components/AuthGuard.svelte';
  import MembersList from '$lib/components/MembersList.svelte';
  import Callout from '$lib/ui-components/elements/Callout.svelte';
  import HeadingSkeleton from '$lib/ui-components/elements/HeadingSkeleton.svelte';
  import DefaultPage from '$lib/ui-components/page-layouts/DefaultPage.svelte';
</script>

<AuthGuard>
  <DefaultPage>
    <svelte:fragment slot="title">My Fav Group</svelte:fragment>
    <Callout>
      <p>🎉 You are the owner of this group.</p>
    </Callout>
    <MembersList members={[]} />
  </DefaultPage>
  <DefaultPage slot="skeleton">
    <svelte:fragment slot="title"><HeadingSkeleton size="lg" /></svelte:fragment>
    <MembersList members={undefined} />
  </DefaultPage>
</AuthGuard>

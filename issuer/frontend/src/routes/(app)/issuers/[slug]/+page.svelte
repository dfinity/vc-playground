<script lang="ts">
  import AuthGuard from '$lib/components/AuthGuard.svelte';
  import MembersList from '$lib/components/MembersList.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import Callout from '$lib/ui-components/elements/Callout.svelte';
  import HeadingSkeleton from '$lib/ui-components/elements/HeadingSkeleton.svelte';
  import DefaultPage from '$lib/ui-components/page-layouts/DefaultPage.svelte';
</script>

<AuthGuard>
  <DefaultPage>
    <Callout slot="callout">
      <p>🎉 You are the issuer of this credential type.</p>
    </Callout>
    <svelte:fragment slot="title">My Fav Credential</svelte:fragment>
    <div>
      <Button variant="primary" href="https://www.skeleton.dev/">Test In relying party</Button>
    </div>
    <MembersList members={[]} title={`Credentials: 200 approved, 2 pending`} />
  </DefaultPage>
  <DefaultPage slot="skeleton">
    <svelte:fragment slot="title"><HeadingSkeleton size="lg" /></svelte:fragment>
    <MembersList members={undefined} />
  </DefaultPage>
</AuthGuard>

<script lang="ts">
  import IssuersList from '$lib/components/IssuersList.svelte';
  import AuthGuard from '$lib/components/AuthGuard.svelte';
  import { authStore } from '$lib/stores/auth.store';
  import { getAllIssuersStore } from '$lib/stores/issuers.store';
  import TestIdWrapper from '$lib/ui-components/elements/TestIdWrapper.svelte';
  import MemberIssuerItem from '$lib/components/MemberIssuerItem.svelte';
  import { setTheme } from '$lib/services/set-theme';
  import DefaultPage from '$lib/ui-components/page-layouts/DefaultPage.svelte';
  import HeadingSkeleton from '$lib/ui-components/elements/HeadingSkeleton.svelte';
  import { onMount } from 'svelte';

  onMount(() => {
    setTheme('user');
  });

  let allIssuersStore;
  $: allIssuersStore = getAllIssuersStore($authStore.identity);
</script>

<TestIdWrapper testId="home-route">
  <AuthGuard>
    <DefaultPage>
      <svelte:fragment slot="title">Credentials</svelte:fragment>
      <IssuersList issuers={$allIssuersStore}>
        {#each $allIssuersStore ?? [] as issuer}
          <MemberIssuerItem {issuer} />
        {/each}
      </IssuersList>
    </DefaultPage>
    <DefaultPage slot="skeleton">
      <svelte:fragment slot="title"><HeadingSkeleton size="lg" /></svelte:fragment>
      <IssuersList issuers={undefined} />
    </DefaultPage>
  </AuthGuard>
</TestIdWrapper>

<script lang="ts">
  import AuthGuard from '$lib/components/AuthGuard.svelte';
  import MembersList from '$lib/components/MembersList.svelte';
  import { authStore } from '$lib/stores/auth.store';
  import { getIssuerDetailStore, type IssuerDetailStore } from '$lib/stores/issuer-detail.store';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import Callout from '$lib/ui-components/elements/Callout.svelte';
  import HeadingSkeleton from '$lib/ui-components/elements/HeadingSkeleton.svelte';
  import DefaultPage from '$lib/ui-components/page-layouts/DefaultPage.svelte';
  import { page } from '$app/stores';
  import { ISSUER_PARAM } from '$lib/constants/url-params.constants';
  import { goto } from '$app/navigation';
  import {
    countApprovedCredentials,
    countPendingCredentials,
  } from '$lib/utils/count-approved-credentials.utils';

  let issuerName: string | null;
  $: issuerName = $page.url.searchParams.get(ISSUER_PARAM);

  $: {
    if (issuerName === null) {
      goto('/');
    }
  }

  let issuerStore: IssuerDetailStore;
  $: issuerStore = getIssuerDetailStore({
    identity: $authStore.identity,
    issuerName: issuerName ?? '',
  });

  $: {
    if ($issuerStore === null) {
      goto('/');
    }
  }

  let approvedCredentials: number;
  $: approvedCredentials = countApprovedCredentials($issuerStore?.members ?? []);
  let pendingCredentials: number;
  $: pendingCredentials = countPendingCredentials($issuerStore?.members ?? []);
</script>

<AuthGuard>
  <DefaultPage>
    <Callout slot="callout">
      <p>🎉 You are the issuer of this credential type.</p>
    </Callout>
    <svelte:fragment slot="title">{$issuerStore?.group_name}</svelte:fragment>
    <div>
      <Button variant="primary" href="https://www.skeleton.dev/">Test In relying party</Button>
    </div>
    <MembersList
      members={$issuerStore?.members}
      title={`Credentials: ${approvedCredentials} approved${pendingCredentials > 0 ? `, ${pendingCredentials} pending.` : '.'}`}
    />
  </DefaultPage>
  <DefaultPage slot="skeleton">
    <svelte:fragment slot="title"><HeadingSkeleton size="lg" /></svelte:fragment>
    <MembersList members={undefined} />
  </DefaultPage>
</AuthGuard>

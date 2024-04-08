<script lang="ts">
  import AuthGuard from '$lib/components/AuthGuard.svelte';
  import MembersList from '$lib/components/MembersList.svelte';
  import { authStore } from '$lib/stores/auth.store';
  import {
    getIssuerDetailStore,
    getIssuerNonRevokedMembers,
    type IssuerDetailStore,
  } from '$lib/stores/issuer-detail.store';
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
  import { browser } from '$app/environment';
  import type { Readable } from 'svelte/store';
  import type { MemberData } from '../../../declarations/meta_issuer.did';
  import { RP_ORIGIN } from '$lib/constants/env-vars';
  import { onMount } from 'svelte';
  import { setTheme } from '$lib/services/set-theme';
  import NavBarItem from '$lib/ui-components/elements/NavBarItem.svelte';

  onMount(() => {
    setTheme('issuer');
  });

  let issuerName: string | null;
  $: issuerName = browser ? $page.url.searchParams.get(ISSUER_PARAM) : null;

  $: {
    if (issuerName === null && browser) {
      goto('/');
    }
  }

  let issuerStore: IssuerDetailStore;
  $: issuerStore = getIssuerDetailStore({
    identity: $authStore.identity,
    issuerName: issuerName ?? '',
  });
  let membersStore: Readable<MemberData[] | undefined>;
  $: membersStore = getIssuerNonRevokedMembers({
    identity: $authStore.identity,
    issuerName: issuerName ?? '',
  });

  $: {
    if ($issuerStore === null && browser) {
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
    <!-- TODO: Create Breadcrumb ui-element -->
    <ol class="breadcrumb" slot="nav">
      <li class="crumb"><a class="anchor" href="/issuer-center">Issuer Control Center</a></li>
      <li class="crumb-separator" aria-hidden>&rsaquo;</li>
      <li>{$issuerStore?.group_name ?? ''}</li>
    </ol>
    <svelte:fragment slot="title">{$issuerStore?.group_name}</svelte:fragment>
    <svelte:fragment slot="subtitle">{`Issued by ${$issuerStore?.issuer_nickname}`}</svelte:fragment
    >
    <MembersList
      members={$membersStore}
      issuerName={$issuerStore?.group_name}
      title={`Credentials: ${approvedCredentials} approved${pendingCredentials > 0 ? `, ${pendingCredentials} pending.` : '.'}`}
    />
  </DefaultPage>
  <DefaultPage slot="skeleton">
    <svelte:fragment slot="title"><HeadingSkeleton size="lg" /></svelte:fragment>
    <MembersList members={undefined} issuerName={undefined} />
  </DefaultPage>
</AuthGuard>

<script lang="ts">
  import IssuersList from '$lib/components/IssuersList.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import FooterActionsWrapper from '$lib/ui-components/elements/FooterActionsWrapper.svelte';
  import { localStorageStore } from '@skeletonlabs/skeleton';
  import type { PublicGroupData } from '../../../declarations/meta_issuer.did';
  import type { Writable } from 'svelte/store';
  import AuthGuard from '$lib/components/AuthGuard.svelte';
  import Tabs from '$lib/ui-components/elements/Tabs.svelte';
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import { createIssuer } from '$lib/services/create-issuer.services';
  import { authStore } from '$lib/stores/auth.store';

  const modalStore = getModalStore();

  // Persist the selected tab in the local storage.
  const tabStore: Writable<number> = localStorageStore('groupsTab', 0);
  let tabSet = $tabStore;
  $: tabStore.set(tabSet);

  const noMyGroupsMessage =
    'Create a group to issue Verifiable Credentials that grant people access to funny images on the Relying Party app.';
  const noCredentialsMessage =
    "You don't have any credentials yet. You can request them in 'All Credentials'.";

  const openCreateModal = () => {
    const settings: ModalSettings = {
      type: 'prompt',
      title: 'Name Your Credential',
      valueAttr: { type: 'text', required: true, placeholder: 'Credential Name' },
      body: 'Create a credential type so that yuo can issue a verifiable credential. Credentials give access to exclusive images on the relying party dapp.',
      buttonTextSubmit: 'Create Issuer',
      response: (issuerName: string) => {
        createIssuer({
          identity: $authStore.identity,
          issuerName,
        });
      },
    };
    modalStore.trigger(settings);
  };

  // TODO: Replace with data from the backend.
  const yesterday = new Date(new Date().getTime() - 24 * 60 * 60 * 1000);
  const groups: PublicGroupData[] = [
    {
      membership_status: [{ Accepted: null }],
      is_owner: [false],
      stats: {
        created_timestamp_ns: BigInt(yesterday.getTime()) * 1000000n,
        member_count: 32,
      },
      group_name: 'Group A',
    },
    {
      membership_status: [],
      is_owner: [false],
      stats: {
        created_timestamp_ns: BigInt(yesterday.getTime()) * 1000000n,
        member_count: 705,
      },
      group_name: 'Group B',
    },
    {
      membership_status: [{ PendingReview: null }],
      is_owner: [false],
      stats: {
        created_timestamp_ns: BigInt(yesterday.getTime()) * 1000000n,
        member_count: 0,
      },
      group_name: 'Group C',
    },
    {
      membership_status: [{ Rejected: null }],
      is_owner: [false],
      stats: {
        created_timestamp_ns: BigInt(yesterday.getTime()) * 1000000n,
        member_count: 2100,
      },
      group_name: 'Group D',
    },
    {
      membership_status: [{ Accepted: null }],
      is_owner: [true],
      stats: {
        created_timestamp_ns: BigInt(yesterday.getTime()) * 1000000n,
        member_count: 11,
      },
      group_name: 'Group Z',
    },
  ];
</script>

<Tabs
  bind:tabSet
  tabs={[
    { name: 'all-credentials', label: 'All Credentials', value: 0 },
    { name: 'my-credentials', label: 'My Credentials', value: 1 },
    { name: 'issuer-control-cernter', label: 'Issuer Control Center', value: 2 },
  ]}
>
  <AuthGuard>
    {#if tabSet === 0}
      <IssuersList issuers={groups} />
    {:else if tabSet === 1}
      <IssuersList issuers={groups} noGroupsMessage={noCredentialsMessage} />
    {:else if tabSet === 2}
      <FooterActionsWrapper>
        <IssuersList issuers={groups} noGroupsMessage={noMyGroupsMessage} />
        <Button on:click={openCreateModal} variant="primary" slot="actions">Become an Issuer</Button
        >
      </FooterActionsWrapper>
    {/if}
    <svelte:fragment slot="skeleton">
      <IssuersList issuers={undefined} />
    </svelte:fragment>
  </AuthGuard>
</Tabs>

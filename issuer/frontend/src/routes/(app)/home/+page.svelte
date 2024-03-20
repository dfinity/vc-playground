<script lang="ts">
  import IssuersList from '$lib/components/IssuersList.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import FooterActionsWrapper from '$lib/ui-components/elements/FooterActionsWrapper.svelte';
  import { getToastStore, localStorageStore } from '@skeletonlabs/skeleton';
  import type { Writable } from 'svelte/store';
  import AuthGuard from '$lib/components/AuthGuard.svelte';
  import Tabs from '$lib/ui-components/elements/Tabs.svelte';
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import { createIssuer } from '$lib/services/create-issuer.services';
  import { authStore } from '$lib/stores/auth.store';
  import {
    getAllIssuersStore,
    getCredentialsStore,
    getIdentityIssuersStore,
  } from '$lib/stores/issuers.store';

  const modalStore = getModalStore();
  const toastStore = getToastStore();

  // Persist the selected tab in the local storage.
  const tabStore: Writable<number> = localStorageStore('groupsTab', 0);
  let tabSet = $tabStore;
  $: tabStore.set(tabSet);

  let allIssuersStore;
  $: allIssuersStore = getAllIssuersStore($authStore.identity);
  let myCredentialsStore;
  $: myCredentialsStore = getCredentialsStore($authStore.identity);
  let myIssuersStore;
  $: myIssuersStore = getIdentityIssuersStore($authStore.identity);

  const noMyGroupsMessage =
    'Create a group to issue Verifiable Credentials that grant people access to funny images on the Relying Party app.';
  const noCredentialsMessage =
    "You don't have any credentials yet. You can request them in 'All Credentials'.";

  let loadingCreateIssuer = false;
  const openCreateModal = () => {
    loadingCreateIssuer = true;
    const settings: ModalSettings = {
      type: 'prompt',
      title: 'Name Your Credential',
      valueAttr: { type: 'text', required: true, placeholder: 'Credential Name' },
      body: 'Create a credential type so that yuo can issue a verifiable credential. Credentials give access to exclusive images on the relying party dapp.',
      buttonTextSubmit: 'Create Issuer',
      response: async (issuerName: string) => {
        await createIssuer({
          identity: $authStore.identity,
          issuerName,
          toastStore,
        });
        loadingCreateIssuer = false;
      },
    };
    modalStore.trigger(settings);
  };
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
      <IssuersList issuers={$allIssuersStore} />
    {:else if tabSet === 1}
      <IssuersList issuers={$myCredentialsStore} noGroupsMessage={noCredentialsMessage} />
    {:else if tabSet === 2}
      <FooterActionsWrapper>
        <IssuersList issuers={$myIssuersStore} noGroupsMessage={noMyGroupsMessage} />
        <Button
          on:click={openCreateModal}
          variant="primary"
          slot="actions"
          loading={loadingCreateIssuer}>Become an Issuer</Button
        >
      </FooterActionsWrapper>
    {/if}
    <svelte:fragment slot="skeleton">
      <IssuersList issuers={undefined} />
    </svelte:fragment>
  </AuthGuard>
</Tabs>

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
  import { getAllIssuersStore, getIdentityIssuersStore } from '$lib/stores/issuers.store';
  import TestIdWrapper from '$lib/ui-components/elements/TestIdWrapper.svelte';
  import AdminIssuerItem from '$lib/components/AdminIssuerItem.svelte';
  import MemberIssuerItem from '$lib/components/MemberIssuerItem.svelte';
  import { setTheme } from '$lib/services/set-theme';

  const modalStore = getModalStore();
  const toastStore = getToastStore();

  // Persist the selected tab in the local storage.
  const tabStore: Writable<number> = localStorageStore('groupsTab', 0);
  let tabSet = $tabStore;
  $: tabStore.set(tabSet);

  let allIssuersStore;
  $: allIssuersStore = getAllIssuersStore($authStore.identity);
  let myIssuersStore;
  $: myIssuersStore = getIdentityIssuersStore($authStore.identity);

  const noMyGroupsMessage =
    'Issue credentials to users so that they can access exclusive images on the relying party dapp.';

  let loadingCreateIssuer = false;
  const openCreateModal = () => {
    loadingCreateIssuer = true;
    const settings: ModalSettings = {
      type: 'prompt',
      title: 'Name Your Credential',
      valueAttr: { type: 'text', required: true, placeholder: 'Credential Name' },
      body: 'Create a credential type so that you can issue a verifiable credential. Credentials give access to exclusive images on the relying party dapp.',
      buttonTextSubmit: 'Create Issuer',
      response: async (issuerName: string) => {
        if (issuerName) {
          await createIssuer({
            identity: $authStore.identity,
            issuerName,
            toastStore,
          });
        }
        loadingCreateIssuer = false;
      },
    };
    modalStore.trigger(settings);
  };

  $: {
    if (tabSet === 1) {
      setTheme('issuer');
    } else {
      setTheme('credentials');
    }
  }
</script>

<TestIdWrapper testId="home-route">
  <Tabs
    bind:tabSet
    tabs={[
      { name: 'Credentials', label: 'Credentials', value: 0 },
      { name: 'Issuer', label: 'Issuer Control Center', value: 1 },
    ]}
  >
    <AuthGuard>
      {#if tabSet === 0}
        <IssuersList issuers={$allIssuersStore}>
          {#each $allIssuersStore ?? [] as issuer}
            <MemberIssuerItem {issuer} />
          {/each}
        </IssuersList>
      {:else if tabSet === 1}
        <FooterActionsWrapper>
          <IssuersList issuers={$myIssuersStore} noGroupsMessage={noMyGroupsMessage}>
            {#each $myIssuersStore ?? [] as issuer}
              <AdminIssuerItem {issuer} />
            {/each}
          </IssuersList>
          <Button
            on:click={openCreateModal}
            variant="primary"
            slot="actions"
            loading={loadingCreateIssuer}>Create Credential</Button
          >
        </FooterActionsWrapper>
      {/if}
      <svelte:fragment slot="skeleton">
        <IssuersList issuers={undefined} />
      </svelte:fragment>
    </AuthGuard>
  </Tabs>
</TestIdWrapper>

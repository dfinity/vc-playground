<script lang="ts">
  import IssuersList from '$lib/components/IssuersList.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import FooterActionsWrapper from '$lib/ui-components/elements/FooterActionsWrapper.svelte';
  import { getToastStore } from '@skeletonlabs/skeleton';
  import AuthGuard from '$lib/components/AuthGuard.svelte';
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import { createIssuer } from '$lib/services/create-issuer.services';
  import { authStore } from '$lib/stores/auth.store';
  import { getIdentityIssuersStore } from '$lib/stores/issuers.store';
  import TestIdWrapper from '$lib/ui-components/elements/TestIdWrapper.svelte';
  import AdminIssuerItem from '$lib/components/AdminIssuerItem.svelte';
  import { setTheme } from '$lib/services/set-theme';
  import DefaultPage from '$lib/ui-components/page-layouts/DefaultPage.svelte';
  import HeadingSkeleton from '$lib/ui-components/elements/HeadingSkeleton.svelte';
  import { onMount } from 'svelte';

  onMount(() => {
    setTheme('issuer');
  });

  const modalStore = getModalStore();
  const toastStore = getToastStore();

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
</script>

<TestIdWrapper testId="home-route">
  <AuthGuard>
    <DefaultPage>
      <svelte:fragment slot="title">Issuer Control Center</svelte:fragment>
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
    </DefaultPage>
    <DefaultPage slot="skeleton">
      <svelte:fragment slot="title"><HeadingSkeleton size="lg" /></svelte:fragment>
      <IssuersList issuers={undefined} />
    </DefaultPage>
  </AuthGuard>
</TestIdWrapper>

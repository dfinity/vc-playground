<script lang="ts">
  import IssuersList from '$lib/components/IssuersList.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import ActionsWrapper from '$lib/ui-components/elements/ActionsWrapper.svelte';
  import { getToastStore } from '@skeletonlabs/skeleton';
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import { createIssuer } from '$lib/services/create-issuer.services';
  import { authStore } from '$lib/stores/auth.store';
  import { getIdentityIssuersStore } from '$lib/stores/issuers.store';
  import TestIdWrapper from '$lib/ui-components/elements/TestIdWrapper.svelte';
  import AdminIssuerItem from '$lib/components/AdminIssuerItem.svelte';
  import { setTheme } from '$lib/services/set-theme';
  import DefaultPage from '$lib/ui-components/page-layouts/DefaultPage.svelte';
  import { onMount } from 'svelte';
  import { getIssuerNickname } from '$lib/stores/user.store';
  import type { Readable } from 'svelte/store';
  import { addIssuerNickname } from '$lib/services/add-issuer-nickname.services';
  import type { Identity } from '@dfinity/agent';
  import { isNullish } from '$lib/utils/is-nullish.utils';
  import { login } from '$lib/services/auth.services';
  import Stack from '$lib/ui-components/elements/Stack.svelte';

  onMount(() => {
    setTheme('issuer');
  });

  const modalStore = getModalStore();
  const toastStore = getToastStore();

  let myIssuersStore;
  $: myIssuersStore = getIdentityIssuersStore($authStore.identity);

  let loadingCreateIssuer = false;
  const openCreateModal = () => {
    loadingCreateIssuer = true;
    const settings: ModalSettings = {
      type: 'component',
      component: 'createCredentialModal',
      meta: { issuerNickname: $issuerNickname },
      response: async (credential: string) => {
        if (credential) {
          await createIssuer({
            identity: $authStore.identity,
            issuerName: credential,
            toastStore,
          });
        }
        loadingCreateIssuer = false;
      },
    };
    modalStore.trigger(settings);
  };

  let issuerNickname: Readable<undefined | null | string>;
  $: issuerNickname = getIssuerNickname($authStore.identity);

  const openIssuerNicknameModal = () => {
    const settings: ModalSettings = {
      type: 'prompt',
      title: 'Name your issuer',
      body: 'In the playground, users will see that the credential has been "issued by" this name. Typically, organizations will issue credentials to individuals, so you might name your issuer after your organization.',
      valueAttr: { placeholder: 'Issuer name' },
      buttonTextSubmit: 'Name issuer',
      buttonTextCancel: 'Close',
      response: (nickname: boolean | string) => {
        if (nickname) {
          addIssuerNickname({
            identity: $authStore.identity as Identity,
            nickname: nickname as string,
          });
        }
      },
    };
    modalStore.trigger(settings);
  };

  $: {
    if ($issuerNickname === null) {
      openIssuerNicknameModal();
    }
  }
</script>

<TestIdWrapper testId="home-route">
  <DefaultPage>
    <svelte:fragment slot="title">
      {isNullish($issuerNickname) ? 'Organization' : $issuerNickname}
    </svelte:fragment>
    <svelte:fragment slot="subtitle">
      {#if $authStore.identity === null}
        Log in to create, issue and revoke credentials from users.
      {:else if !isNullish($authStore.identity)}
        This is the Issuer Control Center. From here you can create, issue and revoke credentials
        from users.
      {/if}
    </svelte:fragment>
    {#if $authStore.identity === null}
      <Stack align="center">
        <Button variant="primary" on:click={login}>Login</Button>
      </Stack>
    {:else if !isNullish($authStore.identity)}
      <ActionsWrapper>
        <IssuersList issuers={$myIssuersStore}>
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
      </ActionsWrapper>
    {/if}
  </DefaultPage>
</TestIdWrapper>

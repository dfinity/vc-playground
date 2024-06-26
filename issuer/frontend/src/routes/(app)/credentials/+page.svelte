<script lang="ts">
  import IssuersList from '$lib/components/IssuersList.svelte';
  import { authStore } from '$lib/stores/auth.store';
  import { getAllIssuersStore } from '$lib/stores/issuers.store';
  import TestIdWrapper from '$lib/ui-components/elements/TestIdWrapper.svelte';
  import MemberIssuerItem from '$lib/components/MemberIssuerItem.svelte';
  import { setTheme } from '$lib/services/set-theme';
  import DefaultPage from '$lib/ui-components/page-layouts/DefaultPage.svelte';
  import HeadingSkeleton from '$lib/ui-components/elements/HeadingSkeleton.svelte';
  import { onMount } from 'svelte';
  import { getUserNickname } from '$lib/stores/user.store';
  import { isNullish } from '$lib/utils/is-nullish.utils';
  import type { Readable } from 'svelte/store';
  import { getModalStore, getToastStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import { addUserNickname } from '$lib/services/add-user-nickname.services';
  import type { Identity } from '@dfinity/agent';
  import { login } from '$lib/services/auth.services';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import Stack from '$lib/ui-components/elements/Stack.svelte';

  const modalStore = getModalStore();
  const toastStore = getToastStore();

  onMount(() => {
    setTheme('user');
  });

  let allIssuersStore;
  $: allIssuersStore = getAllIssuersStore($authStore.identity);

  let userNickname: Readable<undefined | null | string>;
  $: userNickname = getUserNickname($authStore.identity);

  const openUserNicknameModal = () => {
    const settings: ModalSettings = {
      type: 'prompt',
      title: 'Create a username',
      body: 'The username is what the issuers willl see when you request a credential.',
      valueAttr: { placeholder: 'username' },
      buttonTextSubmit: 'Create username',
      buttonTextCancel: 'Close',
      response: (nickname: boolean | string) => {
        if (nickname) {
          addUserNickname({
            identity: $authStore.identity as Identity,
            nickname: nickname as string,
            toastStore,
          });
        }
      },
    };
    modalStore.trigger(settings);
  };

  $: {
    if ($userNickname === null) {
      openUserNicknameModal();
    }
  }
</script>

<TestIdWrapper testId="credentials-page">
  <DefaultPage>
    <svelte:fragment slot="title">
      {#if $authStore.identity === undefined}
        <HeadingSkeleton size="lg" />
      {:else if $authStore.identity === null}
        Credentials
      {:else}
        {isNullish($userNickname) ? 'Credentials' : `@${$userNickname}'s Credentials`}
      {/if}
    </svelte:fragment>
    <svelte:fragment slot="subtitle">
      {#if $authStore.identity === null}
        Login to request credentials.
      {/if}
      Below is a list of all the credentials in the Verifiable Credentials Playground ecosystem.
    </svelte:fragment>
    {#if $authStore.identity === null}
      <Stack align="center">
        <Button testId="login-button" variant="primary" on:click={() => login(toastStore)}
          >Login</Button
        >
      </Stack>
    {/if}
    <IssuersList issuers={$allIssuersStore} noGroupsMessage="No credentials found.">
      {#each $allIssuersStore ?? [] as issuer}
        <MemberIssuerItem {issuer} />
      {/each}
    </IssuersList>
  </DefaultPage>
</TestIdWrapper>

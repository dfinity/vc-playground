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
  import { getUserNickname } from '$lib/stores/user.store';
  import { isNullish } from '$lib/utils/is-nullish.utils';
  import type { Readable } from 'svelte/store';
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import { addUserNickname } from '$lib/services/add-user-nickname.services';
  import type { Identity } from '@dfinity/agent';
  import AdminIssuerItem from '$lib/components/AdminIssuerItem.svelte';

  const modalStore = getModalStore();

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
      valueAttr: { placeholder: '@username' },
      buttonTextSubmit: 'Create username',
      buttonTextCancel: 'Close',
      response: (nickname: boolean | string) => {
        if (nickname) {
          addUserNickname({
            identity: $authStore.identity as Identity,
            nickname: nickname as string,
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

<TestIdWrapper testId="home-route">
  <AuthGuard>
    <DefaultPage>
      <svelte:fragment slot="title">
        {isNullish($userNickname) ? 'Credentials' : `@${$userNickname}'s Credentials`}
      </svelte:fragment>
      <svelte:fragment slot="subtitle">
        Below is a list of all the credentials in the VC playground ecosystem.
      </svelte:fragment>
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

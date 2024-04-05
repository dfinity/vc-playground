<script lang="ts">
  import MainWrapper from '$lib/ui-components/elements/MainWrapper.svelte';
  import { onMount } from 'svelte';
  import { AppShell, AppBar, Modal, Toast, type ModalComponent } from '@skeletonlabs/skeleton';
  import { syncAuth } from '$lib/services/auth.services';
  import { initializeStores } from '@skeletonlabs/skeleton';
  import SettingsDropdown from '$lib/components/SettingsDropdown.svelte';
  import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
  import { storePopup } from '@skeletonlabs/skeleton';
  import { page } from '$app/stores';
  import { authStore } from '$lib/stores/auth.store';
  import { isNullish } from '$lib/utils/is-nullish.utils';
  import HeaderTitle from '$lib/ui-components/elements/HeaderTitle.svelte';
  import Box from '$lib/ui-components/elements/Box.svelte';
  import CreateCredentialModal from '$lib/modals/CreateCredentialModal.svelte';

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  initializeStores();

  const modalRegistry: Record<string, ModalComponent> = {
    createCredentialModal: { ref: CreateCredentialModal },
  };

  onMount(() => {
    syncAuth();
  });

  let currentRole: 'User' | 'Issuer';
  $: currentRole = $page.route.id === '/(app)/credentials' ? 'User' : 'Issuer';
</script>

<Modal components={modalRegistry} />
<Toast />

<AppShell>
  <AppBar slot="header">
    <Box slot="lead">
      <HeaderTitle>VC Playground</HeaderTitle>
      <Box responsive={false}>
        <a class={$page.route.id === '/(app)/credentials' ? 'underline' : ''} href="/credentials"
          >Request</a
        >
        <a class={$page.route.id === '/(app)/credentials' ? '' : 'underline'} href="/issuer-center"
          >Issue</a
        >
      </Box>
    </Box>
    <svelte:fragment slot="trail">
      {#if !isNullish($authStore.identity)}
        <SettingsDropdown {currentRole} />
      {/if}
    </svelte:fragment>
  </AppBar>
  <MainWrapper>
    <!-- Page Route Content -->
    <slot />
  </MainWrapper>
</AppShell>

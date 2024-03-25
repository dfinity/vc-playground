<script lang="ts">
  import { page } from '$app/stores';
  import AuthButton from '$lib/components/AuthButton.svelte';
  import Button from '$lib/components/Button.svelte';
  import ChooseImageModal from '$lib/components/ChooseImageModal.svelte';
  import MainWrapper from '$lib/components/MainWrapper.svelte';
  import ViewExclusiveContentModal from '$lib/components/ViewExclusiveContentModal.svelte';
  import { onMount } from 'svelte';
  import '../app.postcss';
  import {
    AppShell,
    AppBar,
    Modal,
    type ModalComponent,
    initializeStores,
    Toast,
  } from '@skeletonlabs/skeleton';
  import { syncAuth } from '$lib/services/auth.services';
  import { authStore } from '$lib/stores/auth.store';
  import { nonNullish } from '$lib/utils/non-nullish';

  initializeStores();

  onMount(() => {
    syncAuth();
  });

  const modalRegistry: Record<string, ModalComponent> = {
    chooseImageModal: { ref: ChooseImageModal },
    viewExclusiveContentModal: { ref: ViewExclusiveContentModal },
  };
</script>

<Modal components={modalRegistry} />
<Toast />

<!-- App Shell -->
<AppShell>
  <svelte:fragment slot="header">
    <!-- App Bar -->
    <AppBar>
      <a href="/" class="text-xl uppercase font-bold" aria-label="Go to Home" slot="lead"
        >Relying Party
      </a>
      <svelte:fragment slot="trail">
        {#if $page.route.id !== '/share' && nonNullish($authStore.identity)}
          <Button variant="primary" href="/share">Share Image</Button>
        {/if}
        <AuthButton />
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>
  <MainWrapper>
    <!-- Page Route Content -->
    <slot />
  </MainWrapper>
</AppShell>

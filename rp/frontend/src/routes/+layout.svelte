<script lang="ts">
  import { page } from '$app/stores';
  import Button from '$lib/components/Button.svelte';
  import ChooseImageModal from '$lib/components/ChooseImageModal.svelte';
  import MainWrapper from '$lib/components/MainWrapper.svelte';
  import ViewExclusiveContentModal from '$lib/components/ViewExclusiveContentModal.svelte';
  import '../app.postcss';
  import {
    AppShell,
    AppBar,
    Modal,
    type ModalComponent,
    initializeStores,
  } from '@skeletonlabs/skeleton';

  initializeStores();

  const modalRegistry: Record<string, ModalComponent> = {
    chooseImageModal: { ref: ChooseImageModal },
    viewExclusiveContentModal: { ref: ViewExclusiveContentModal },
  };
</script>

<Modal components={modalRegistry} />

<!-- App Shell -->
<AppShell>
  <svelte:fragment slot="header">
    <!-- App Bar -->
    <AppBar>
      <a href="/" class="text-xl uppercase font-bold" aria-label="Go to Home" slot="lead"
        >Relying Party
      </a>
      <svelte:fragment slot="trail">
        {#if $page.route.id !== '/share'}
          <Button variant="primary" href="/share">Share Image</Button>
        {/if}
        <Button variant="secondary">Login</Button>
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>
  <MainWrapper>
    <!-- Page Route Content -->
    <slot />
  </MainWrapper>
</AppShell>

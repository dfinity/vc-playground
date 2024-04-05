<script lang="ts">
  import { page } from '$app/stores';
  import AuthButton from '$lib/components/AuthButton.svelte';
  import Button from '$lib/components/Button.svelte';
  import ChooseImageModal from '$lib/components/ChooseImageModal.svelte';
  import MainWrapper from '$lib/components/MainWrapper.svelte';
  import ViewExclusiveContentModal from '$lib/components/ViewExclusiveContentModal.svelte';
  import { onMount } from 'svelte';
  import {
    AppShell,
    AppBar,
    Modal,
    type ModalComponent,
    initializeStores,
    Toast,
    LightSwitch,
  } from '@skeletonlabs/skeleton';
  import { syncAuth } from '$lib/services/auth.services';
  import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
  import { storePopup } from '@skeletonlabs/skeleton';

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
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
      <div slot="lead" class="flex flex-col sm:flex-row sm:gap-8 sm:items-center">
        <a href="/" class="text-xl uppercase font-bold" aria-label="Go to Home"
          >Image Sharing Platform
        </a>
        <div class="flex gap-4">
          <a href="/feed" class={$page.route.id === '/(app)/feed' ? 'underline' : ''}>View</a>
          <a href="/share" class={$page.route.id === '/(app)/share' ? 'underline' : ''}>Publish</a>
        </div>
      </div>
      <div slot="trail" class="flex flex-col gap-2 items-end">
        <AuthButton />
      </div>
    </AppBar>
  </svelte:fragment>
  <MainWrapper>
    <!-- Page Route Content -->
    <slot />
  </MainWrapper>
  <span class="fixed bottom-0 right-0 p-4">
    <LightSwitch />
  </span>
</AppShell>

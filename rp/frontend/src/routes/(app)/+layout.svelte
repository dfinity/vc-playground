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
  import { authStore } from '$lib/stores/auth.store';
  import { nonNullish } from '$lib/utils/non-nullish';
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
      <a href="/" class="text-xl uppercase font-bold" aria-label="Go to Home" slot="lead"
        >Relying Party
      </a>
      <div slot="trail" class="flex flex-col gap-2 items-end">
        <div class="flex gap-2">
          {#if $page.route.id !== '/share' && nonNullish($authStore.identity)}
            <Button variant="primary" href="/share">Share Image</Button>
          {/if}
          <AuthButton />
        </div>
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

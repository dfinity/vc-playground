<script lang="ts">
  import { page } from '$app/stores';
  import ChooseImageModal from '$lib/components/ChooseImageModal.svelte';
  import MainWrapper from '$lib/components/MainWrapper.svelte';
  import ViewExclusiveContentModal from '$lib/components/ViewExclusiveContentModal.svelte';
  import { onMount } from 'svelte';
  import {
    AppShell,
    Modal,
    type ModalComponent,
    initializeStores,
    Toast,
    LightSwitch,
  } from '@skeletonlabs/skeleton';
  import { syncAuth } from '$lib/services/auth.services';
  import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
  import { storePopup } from '@skeletonlabs/skeleton';
  import Header from '$lib/components/Header.svelte';
  import Footer from '$lib/components/Footer.svelte';

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
  initializeStores();

  onMount(() => {
    syncAuth();
  });

  const modalRegistry: Record<string, ModalComponent> = {
    chooseImageModal: { ref: ChooseImageModal },
    viewExclusiveContentModal: { ref: ViewExclusiveContentModal },
  };

  let currentRole: 'View' | 'Publish';
  $: currentRole = $page.route.id === '/(app)/feed' ? 'View' : 'Publish';
</script>

<Modal components={modalRegistry} />
<Toast />

<!-- App Shell -->
<AppShell>
  <Header {currentRole} />
  <MainWrapper>
    <!-- Page Route Content -->
    <slot />
  </MainWrapper>
  <span class="fixed bottom-0 right-0 p-4">
    <LightSwitch />
  </span>
  <Footer slot="footer" />
</AppShell>

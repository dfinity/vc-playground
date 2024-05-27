<script lang="ts">
  import MainWrapper from '$lib/ui-components/elements/MainWrapper.svelte';
  import { onMount } from 'svelte';
  import { AppShell, Modal, Toast, type ModalComponent } from '@skeletonlabs/skeleton';
  import { syncAuth } from '$lib/services/auth.services';
  import { initializeStores } from '@skeletonlabs/skeleton';
  import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
  import { storePopup } from '@skeletonlabs/skeleton';
  import { page } from '$app/stores';
  import CreateCredentialModal from '$lib/modals/CreateCredentialModal.svelte';
  import Header from '$lib/ui-components/elements/Header.svelte';
  import Footer from '$lib/ui-components/elements/Footer.svelte';
  import RequestVerifiedResidenceModal from '$lib/modals/RequestVerifiedResidenceModal.svelte';

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  initializeStores();

  const modalRegistry: Record<string, ModalComponent> = {
    createCredentialModal: { ref: CreateCredentialModal },
    requestVerifiedResidenceModal: { ref: RequestVerifiedResidenceModal },
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
  <Header slot="header" {currentRole} />
  <MainWrapper>
    <!-- Page Route Content -->
    <slot />
  </MainWrapper>
  <Footer slot="footer" />
</AppShell>

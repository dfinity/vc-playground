<script lang="ts">
  import MainWrapper from '$lib/ui-components/elements/MainWrapper.svelte';
  import { onMount } from 'svelte';
  import '../../app.postcss';
  import { AppShell, AppBar, Modal, Toast } from '@skeletonlabs/skeleton';
  import { syncAuth } from '$lib/services/auth.services';
  import { initializeStores } from '@skeletonlabs/skeleton';
  import SettingsDropdown from '$lib/components/SettingsDropdown.svelte';
  import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
  import { storePopup } from '@skeletonlabs/skeleton';
  import { page } from '$app/stores';

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  initializeStores();

  onMount(() => {
    syncAuth();
  });

  let currentRole: 'User' | 'Issuer';
  $: currentRole = $page.route.id === '/(app)/credentials' ? 'User' : 'Issuer';
</script>

<Modal />
<Toast />

<!-- App Shell -->
<AppShell>
  <svelte:fragment slot="header">
    <!-- App Bar -->
    <AppBar>
      <span slot="lead" class="flex gap-4 items-center">
        <span class="text-xl uppercase text-surface-500">VC Playground</span>
        <span class="text-xl font-heading-token font-bold">{currentRole}</span>
      </span>
      <SettingsDropdown slot="trail" {currentRole} />
    </AppBar>
  </svelte:fragment>
  <MainWrapper>
    <!-- Page Route Content -->
    <slot />
  </MainWrapper>
</AppShell>

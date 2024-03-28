<script lang="ts">
  import Button from '$lib/ui-components/elements/Button.svelte';
  import MainWrapper from '$lib/ui-components/elements/MainWrapper.svelte';
  import { onMount } from 'svelte';
  import '../../app.postcss';
  import { AppShell, AppBar, Modal, Toast } from '@skeletonlabs/skeleton';
  import { syncAuth } from '$lib/services/auth.services';
  import { initializeStores } from '@skeletonlabs/skeleton';
  import Dropdown from '$lib/components/Dropdown.svelte';
  import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
  import { storePopup } from '@skeletonlabs/skeleton';

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  initializeStores();

  onMount(() => {
    syncAuth();
  });
</script>

<Modal />
<Toast />

<!-- App Shell -->
<AppShell>
  <svelte:fragment slot="header">
    <!-- App Bar -->
    <AppBar>
      <a href="/home" class="text-xl uppercase font-bold" aria-label="Go to Home" slot="lead"
        >VC Playground
      </a>
      <svelte:fragment slot="trail">
        <Dropdown />
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>
  <MainWrapper>
    <!-- Page Route Content -->
    <slot />
  </MainWrapper>
</AppShell>

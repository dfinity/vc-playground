<script lang="ts">
  import Button from '$lib/ui-components/elements/Button.svelte';
  import MainWrapper from '$lib/ui-components/elements/MainWrapper.svelte';
  import { onMount } from 'svelte';
  import '../../app.postcss';
  import { AppShell, AppBar, Modal } from '@skeletonlabs/skeleton';
  import { logout, syncAuth } from '$lib/services/auth.services';
  import { initializeStores } from '@skeletonlabs/skeleton';

  initializeStores();

  onMount(() => {
    syncAuth();
  });
</script>

<Modal />

<!-- App Shell -->
<AppShell>
  <svelte:fragment slot="header">
    <!-- App Bar -->
    <AppBar>
      <a href="/home" class="text-xl uppercase font-bold" aria-label="Go to Home" slot="lead"
        >VC Playground
      </a>
      <svelte:fragment slot="trail">
        <Button variant="ghost" on:click={logout}>Logout</Button>
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>
  <MainWrapper>
    <!-- Page Route Content -->
    <slot />
  </MainWrapper>
</AppShell>

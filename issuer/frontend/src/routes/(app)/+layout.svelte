<script lang="ts">
  import AuthGuard from '$lib/components/AuthGuard.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import MainWrapper from '$lib/ui-components/elements/MainWrapper.svelte';
  import { onMount } from 'svelte';
  import '../../app.postcss';
  import { AppShell, AppBar, Modal } from '@skeletonlabs/skeleton';
  import { logout, syncAuth } from '$lib/services/auth.services';
  import { initializeStores } from '@skeletonlabs/skeleton';
  import { modalRegistry } from '$lib/modals/registry.modals';

  initializeStores();

  onMount(() => {
    syncAuth();
  });
</script>

<Modal components={modalRegistry} />

<AuthGuard>
  <!-- App Shell -->
  <AppShell>
    <svelte:fragment slot="header">
      <!-- App Bar -->
      <AppBar>
        <svelte:fragment slot="lead">
          <strong class="text-xl uppercase">VC Playground</strong>
        </svelte:fragment>
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
</AuthGuard>

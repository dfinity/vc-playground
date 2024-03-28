<script lang="ts">
  import Button from '$lib/ui-components/elements/Button.svelte';
  import MainWrapper from '$lib/ui-components/elements/MainWrapper.svelte';
  import { onMount } from 'svelte';
  import '../../app.postcss';
  import { AppShell, AppBar, Modal, Toast } from '@skeletonlabs/skeleton';
  import { logout, syncAuth } from '$lib/services/auth.services';
  import { initializeStores } from '@skeletonlabs/skeleton';
  import { authStore } from '$lib/stores/auth.store';

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
      <div slot="trail" class="flex flex-col sm:flex-row gap-4 sm:items-center items-end">
        <Button variant="ghost" on:click={logout}>Logout</Button>
        <p class="max-w-36 truncate">
          {`Principal: ${$authStore.identity?.getPrincipal().toText()}`}
        </p>
      </div>
    </AppBar>
  </svelte:fragment>
  <MainWrapper>
    <!-- Page Route Content -->
    <slot />
  </MainWrapper>
</AppShell>

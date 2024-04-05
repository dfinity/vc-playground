<script lang="ts">
  import ImagesGrid from '$lib/components/ImagesGrid.svelte';
  import type { Readable } from 'svelte/store';
  import { authStore } from '$lib/stores/auth.store';
  import {
    getVisibleContentData,
    type VisibleContentData,
  } from '$lib/stores/content-data-visible.store';
  import Button from '$lib/components/Button.svelte';
  import { nonNullish } from '$lib/utils/non-nullish';
  import { login } from '$lib/services/auth.services';

  let contentDataStore: Readable<VisibleContentData[]>;
  $: contentDataStore = getVisibleContentData($authStore.identity);
</script>

<div class="flex flex-col gap-6 items-center">
  <h1 class="h1 text-center">View Gated Images</h1>
  <p class="text-center">
    You can view an image if you hold the particular credential required to access the image.
  </p>

  {#if nonNullish($authStore.identity)}
    <Button variant="primary" href="/share">Publish Image</Button>
  {:else}
    <Button variant="secondary" on:click={() => login()} loading={$authStore.identity === undefined}
      >Login</Button
    >
  {/if}
</div>

{#if nonNullish($authStore.identity)}
  <ImagesGrid images={$contentDataStore} />
{/if}

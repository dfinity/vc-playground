<script lang="ts">
  import ImagesGrid from '$lib/components/ImagesGrid.svelte';
  import type { Readable } from 'svelte/store';
  import { authStore } from '$lib/stores/auth.store';
  import {
    getVisibleContentData,
    type VisibleContentData,
  } from '$lib/stores/content-data-visible.store';

  let contentDataStore: Readable<VisibleContentData[]>;
  $: contentDataStore = getVisibleContentData($authStore.identity);
</script>

<h1 class="h1 text-center">View and Share Content</h1>
<p class="text-center">
  This is an example of a Relying Party. You can view an image if you hold the particular credential
  required to access the image.
</p>
<ImagesGrid images={$contentDataStore} />

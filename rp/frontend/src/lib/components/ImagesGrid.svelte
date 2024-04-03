<script lang="ts">
  import type { VisibleContentData } from '$lib/stores/content-data-visible.store';
  import ImageItem from './ImageItem.svelte';

  export let images: VisibleContentData[] = [];
</script>

<section class="grid grid-cols-2 md:grid-cols-3 gap-4">
  {#each images as image}
    <ImageItem {image} />
  {/each}
</section>

<style>
</style>

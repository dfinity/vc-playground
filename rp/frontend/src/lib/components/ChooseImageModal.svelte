<script lang="ts">
  import { getModalStore } from '@skeletonlabs/skeleton';
  import Modal from './Modal.svelte';
  import { getImagesStore } from '$lib/stores/images.store';
  import { authStore } from '$lib/stores/auth.store';
  import type { Writable } from 'svelte/store';
  import type { ImageData } from '../../declarations/rp/rp.did';
  import Button from './Button.svelte';

  /* eslint-disable-next-line */
  export let parent: any;
  const modalStore = getModalStore();

  let imagesStore: Writable<ImageData[] | undefined>;
  $: imagesStore = getImagesStore($authStore.identity);

  const selectImageFactory = (image: ImageData) => (evt: Event) => {
    evt.preventDefault();
    $modalStore[0].response?.(image);
    parent.onClose();
  };

  const close = () => {
    parent.onClose();
  };

  let isLoading = false;
  $: isLoading = $imagesStore === undefined;
</script>

<Modal>
  <svelte:fragment slot="header">Choose Image</svelte:fragment>
  <article class="grid grid-cols-2 md:grid-cols-4 gap-4">
    {#if isLoading}
      <div class="placeholder h-auto max-w-full rounded-container-token aspect-square" />
      <div class="placeholder h-auto max-w-full rounded-container-token aspect-square" />
      <div class="placeholder h-auto max-w-full rounded-container-token aspect-square" />
      <div class="placeholder h-auto max-w-full rounded-container-token aspect-square" />
      <div class="placeholder h-auto max-w-full rounded-container-token aspect-square" />
      <div class="placeholder h-auto max-w-full rounded-container-token aspect-square" />
      <div class="placeholder h-auto max-w-full rounded-container-token aspect-square" />
      <div class="placeholder h-auto max-w-full rounded-container-token aspect-square" />
      <div class="placeholder h-auto max-w-full rounded-container-token aspect-square" />
    {:else}
      {#each $imagesStore ?? [] as image, i (image.url)}
        <a href={'#'} on:click={selectImageFactory(image)}>
          <img class="h-auto max-w-full rounded-container-token" src={image.url} alt={`Option ${i}`} />
        </a>
      {/each}
    {/if}
  </article>
  <Button slot="footer" on:click={close} variant="ghost-primary">Close</Button>
</Modal>

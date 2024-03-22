<script lang="ts">
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import Button from './Button.svelte';

  export let images: { imageUrl: string; issuerName: string }[] = [];

  const modalStore = getModalStore();

  const openImageFactory = (image: { imageUrl: string; issuerName: string }) => () => {
    const modal: ModalSettings = {
      type: 'component',
      component: 'viewExclusiveContentModal',
      meta: { image },
    };
    modalStore.trigger(modal);
  };
</script>

<section class="grid grid-cols-2 md:grid-cols-3 gap-4">
  {#each images as image}
    <div class="relative">
      <div class="absolute -top-0 -left-0 w-full flex flex-col items-center py-2 px-2 h-full">
        <h5 class="h5">{image.issuerName}</h5>
        <div class="flex-1 flex justify-center items-center">
          <Button variant="ghost" on:click={openImageFactory(image)}>View</Button>
        </div>
        <p class="text-sm self-start">1 minute ago</p>
      </div>
      <div
        class="h-auto max-w-full rounded-lg aspect-square bg-gradient-to-b from-primary-500 to-secondary-500"
      />
    </div>
  {/each}
</section>

<style>
</style>

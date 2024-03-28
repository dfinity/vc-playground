<script lang="ts">
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import Button from './Button.svelte';
  import { authStore } from '$lib/stores/auth.store';
  import { nonNullish } from '$lib/utils/non-nullish';
  import { login } from '$lib/services/auth.services';
  import { nanoSecondsToDateTime } from '$lib/utils/date.utils';
  import type { VisibleContentData } from '$lib/stores/content-data-visible.store';

  export let images: VisibleContentData[] = [];

  const modalStore = getModalStore();

  const openImageFactory = (content: VisibleContentData) => () => {
    if (nonNullish($authStore.identity)) {
      const modal: ModalSettings = {
        type: 'component',
        component: 'viewExclusiveContentModal',
        meta: { content, issuerName: content.credential_group_name },
      };
      modalStore.trigger(modal);
    } else {
      login();
    }
  };

  const visibleImageGradient = `
    background-image: linear-gradient(
      to bottom, 
      rgba(0,0,0,0.6) 0%, 
      rgba(0,0,0,0) 3rem,
      rgba(0,0,0,0) calc(100% - 3rem), /* Transparent middle */
      rgba(0,0,0,0.6) 100%);
  `;
</script>

<section class="grid grid-cols-2 md:grid-cols-3 gap-4 text-surface-50">
  {#each images as image}
    <div class="relative">
      {#if !image.visible}
        <div class="absolute -top-0 -left-0 w-full rounded-container-token aspect-square backdrop-blur-xl"></div>
      {/if}
      <div
        class="absolute -top-0 -left-0 w-full flex flex-col rounded-container-token justify-between items-center py-2 px-2 h-full"
        style={visibleImageGradient}
      >
        <h5 class="h5 truncate w-full">{image.credential_group_name}</h5>
        {#if !image.visible}
          <div class="flex-1 flex justify-center items-center">
            <Button variant="secondary" on:click={openImageFactory(image)}>View</Button>
          </div>
        {/if}
        <p class="text-sm self-start">{nanoSecondsToDateTime(image.created_timestamp_ns)}</p>
      </div>
      <div
        class="h-auto max-w-full rounded-container-token aspect-square"
        style="background-image: url({image.url}); background-size: cover; background-position: center;"
      />
    </div>
  {/each}
</section>

<style>
</style>

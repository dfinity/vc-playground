<script lang="ts">
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import Button from './Button.svelte';
  import { authStore } from '$lib/stores/auth.store';
  import { nonNullish } from '$lib/utils/non-nullish';
  import { login } from '$lib/services/auth.services';
  import type { ContentData } from '../../declarations/rp/rp.did';
  import { nanoSecondsToDateTime } from '$lib/utils/date.utils';

  export let images: ContentData[] = [];

  const modalStore = getModalStore();

  const openImageFactory = (content: ContentData) => () => {
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
</script>

<section class="grid grid-cols-2 md:grid-cols-3 gap-4">
  {#each images as image}
    <!-- TODO: Show image if the user has the credential -->
    <div class="relative">
      <div class="absolute -top-0 -left-0 w-full flex flex-col items-center py-2 px-2 h-full">
        <h5 class="h5 truncate w-full">{image.credential_group_name}</h5>
        <div class="flex-1 flex justify-center items-center">
          <Button variant="ghost-primary" on:click={openImageFactory(image)}>View</Button>
        </div>
        <p class="text-sm self-start">{nanoSecondsToDateTime(image.created_timestamp_ns)}</p>
      </div>
      <div
        class="h-auto max-w-full rounded-lg aspect-square bg-gradient-to-b from-primary-500 to-secondary-500"
      />
    </div>
  {/each}
</section>

<style>
</style>

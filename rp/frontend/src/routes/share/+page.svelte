<script lang="ts">
  import { goto } from '$app/navigation';
  import Button from '$lib/components/Button.svelte';
  import { authStore } from '$lib/stores/auth.store';
  import { nonNullish } from '$lib/utils/non-nullish';
  import { getModalStore, getToastStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import type { Writable } from 'svelte/store';
  import type { PublicGroupData } from '../../declarations/meta_issuer/meta_issuer.did';
  import { getIssuersStore } from '$lib/stores/issuers.store';
  import type { ImageData } from '../../declarations/rp/rp.did';
  import { shareContent } from '$lib/services/shareContent.services';

  const modalStore = getModalStore();
  const toastStore = getToastStore();

  $: {
    if ($authStore.identity === null) {
      goto('/');
    }
  }

  let issuersStore: Writable<PublicGroupData[] | undefined>;
  $: issuersStore = getIssuersStore($authStore.identity);

  let selectedIssuerName: string | undefined;

  let selectedImage: ImageData | undefined = undefined;
  const openChooseImageModal = () => {
    const modal: ModalSettings = {
      type: 'component',
      component: 'chooseImageModal',
      response: (r: undefined | false | ImageData) => {
        if (r) {
          selectedImage = r;
        }
      },
    };
    modalStore.trigger(modal);
  };

  let enableShareButton = false;
  $: enableShareButton = (selectedIssuerName ?? '').length > 0 && selectedImage !== undefined;

  let isLoading = false;
  const share = async () => {
    isLoading = true;
    // Edge case, should never happen because button is disabled.
    if (!selectedIssuerName || !selectedImage) {
      return;
    }
    try {
      await shareContent({
        issuerName: selectedIssuerName,
        image: selectedImage,
        identity: $authStore.identity,
      });
      toastStore.trigger({
        message: 'Content shared successfully!',
        background: 'variant-filled-success',
      });
      goto('/');
    } catch (error) {
      console.error('Error sharing content', error);
      toastStore.trigger({
        message: `Oops! There was an error sharing the content. Please try again. ${error}`,
        background: 'variant-filled-error',
      });
    } finally {
      isLoading = false;
    }
  };
</script>

{#if nonNullish($authStore.identity)}
  <h1 class="h1">Give a Credential Type Access To an Exclusive Image</h1>
  <div class="flex flex-col gap-4">
    <label for="credentials">
      <h5 class="h5">With whom would you like to share this?</h5>
    </label>
    <select bind:value={selectedIssuerName} id="credentials" class="select px-4">
      <option value="" disabled selected>Select an issuer</option>
      {#each $issuersStore ?? [] as issuer}
        <option value={issuer.group_name} id={issuer.group_name}>
          {issuer.group_name}
        </option>
      {/each}
    </select>
  </div>

  <div class="flex flex-col gap-4">
    <h5 class="h5">Pick an image to share</h5>
    {#if selectedImage}
      <div class="flex justify-center">
        <img src={selectedImage.url} alt="Selected" class="max-w-72 h-auto rounded-sm" />
      </div>
    {/if}
    <div class="flex justify-center">
      <Button on:click={openChooseImageModal} variant="secondary">Choose Image</Button>
    </div>
  </div>

  <div class="flex justify-end">
    <Button on:click={share} variant="primary" disabled={!enableShareButton} loading={isLoading}
      >Share</Button
    >
  </div>
{:else}
  <div class="placeholder" />
{/if}

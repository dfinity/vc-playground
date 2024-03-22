<script lang="ts">
  import Button from '$lib/components/Button.svelte';
  import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';

  const modalStore = getModalStore();

  const credentials: { value: string; label: string }[] = [
    { value: '1', label: 'Option 1' },
    { value: '2', label: 'Option 2' },
    { value: '3', label: 'Option 3' },
  ];

  let selectedImage: string | undefined = undefined;
  const openChooseImageModal = () => {
    const modal: ModalSettings = {
      type: 'component',
      component: 'modalChooseImage',
      response: (r: unknown) => {
        if (r) {
          selectedImage = r as string;
        }
      },
    };
    modalStore.trigger(modal);
  };
</script>

<h1 class="h1">Give a Credential Type Access To an Exclusive Image</h1>

<div class="flex flex-col gap-4">
  <label for="credentials">
    <h5 class="h5">With whom would you like to share this?</h5>
  </label>
  <select id="credentials" name="tabs" class="select">
    {#each credentials as credential}
      <option value={credential.value} id={credential.value}>
        {credential.label}
      </option>
    {/each}
  </select>
</div>

<div class="flex flex-col gap-4">
  <h5 class="h5">Pick an image to share</h5>
  {#if selectedImage}
    <div class="sm:px-16">
      <img src={selectedImage} alt="Selected" class="max-w-full h-auto rounded-lg" />
    </div>
  {/if}
  <div class="flex justify-center">
    <Button on:click={openChooseImageModal} variant="secondary">Choose Image</Button>
  </div>
</div>

<div class="flex justify-end">
  <Button variant="primary">Share</Button>
</div>

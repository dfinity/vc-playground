<script lang="ts">
  /* eslint-disable svelte/no-at-html-tags */
  import { getModalStore } from '@skeletonlabs/skeleton';
  import Modal from './Modal.svelte';
  import Button from './Button.svelte';

  /* eslint-disable-next-line */
  export let parent: any;

  const modalStore = getModalStore();

  let issuerName = '';
  $: issuerName = $modalStore[0]?.meta.issuerName;
  let imageUrl = '';
  $: imageUrl = $modalStore[0]?.meta.content.url;

  const close = () => {
    parent.onClose();
  };
</script>

<Modal>
  <svelte:fragment slot="header">Choose Image</svelte:fragment>
  <div class="flex flex-col">
    {#if imageUrl}
      <div class="sm:px-36">
        <img class="h-auto max-w-full rounded-lg" src={imageUrl} alt="Visible" />
      </div>
    {:else}
      <p>{@html `You did not prove you hold the <em>${issuerName}</em> credential.`}</p>
      <p>
        {@html `If you want to access the image, request the <em>${issuerName}</em> credential from the issuer and share your credential with the demo relying party.`}
      </p>
    {/if}
  </div>
  <Button slot="footer" on:click={close} variant="ghost-primary">Close</Button>
</Modal>

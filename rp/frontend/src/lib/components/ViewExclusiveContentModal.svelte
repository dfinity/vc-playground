<script lang="ts">
  /* eslint-disable svelte/no-at-html-tags */
  import { getModalStore } from '@skeletonlabs/skeleton';
  import Modal from './Modal.svelte';
  import Button from './Button.svelte';
  import { getCredential } from '$lib/services/get-credential.services';
  import { authStore } from '$lib/stores/auth.store';

  /* eslint-disable-next-line */
  export let parent: any;

  const modalStore = getModalStore();

  let issuerName = '';
  $: issuerName = $modalStore[0]?.meta.issuerName;
  let imageUrl = '';
  $: imageUrl = $modalStore[0]?.meta.content.url;

  let vcFlowLoading = false;
  let credential: string | undefined | null = undefined;
  const startFlow = async () => {
    vcFlowLoading = true;
    credential = await getCredential({ groupName: issuerName, identity: $authStore.identity });
    vcFlowLoading = false;
  };

  const close = () => {
    parent.onClose();
  };
</script>

<Modal>
  <svelte:fragment slot="header">Choose Image</svelte:fragment>
  {#if !vcFlowLoading && credential === undefined}
    <Button on:click={startFlow} variant="primary">Get Credential</Button>
  {:else if vcFlowLoading}
    <div class="placehoolder" />
  {:else}
    <div class="flex flex-col">
      {#if credential}
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
  {/if}
  <Button slot="footer" on:click={close} variant="ghost-primary">Close</Button>
</Modal>

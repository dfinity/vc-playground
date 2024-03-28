<script lang="ts">
  /* eslint-disable svelte/no-at-html-tags */
  import { getModalStore } from '@skeletonlabs/skeleton';
  import Modal from './Modal.svelte';
  import Button from './Button.svelte';
  import { loadCredential } from '$lib/services/load-credential.services';
  import { authStore } from '$lib/stores/auth.store';
  import { credentialsStore } from '$lib/stores/credentials.store';

  /* eslint-disable-next-line */
  export let parent: any;

  const modalStore = getModalStore();

  let issuerName = '';
  $: issuerName = $modalStore[0]?.meta.issuerName;
  let imageUrl = '';
  $: imageUrl = $modalStore[0]?.meta.content.url;

  let vcFlowLoading = false;
  const startFlow = async () => {
    vcFlowLoading = true;
    await loadCredential({ groupName: issuerName, identity: $authStore.identity });
    vcFlowLoading = false;
  };

  const close = () => {
    parent.onClose();
  };

  let hasCredential: boolean | undefined;
  $: hasCredential = $credentialsStore[issuerName]?.hasCredential;
</script>

<Modal>
  <svelte:fragment slot="header">Get Credential</svelte:fragment>
  {#if !vcFlowLoading && hasCredential === undefined}
    <div class="flex-1 flex flex-col justify-center items-center gap-4">
      <p>
        Get the credential <em>{issuerName}</em> to view this image.
      </p>
      <Button on:click={startFlow} variant="primary">Get Credential</Button>
    </div>
  {:else if vcFlowLoading}
    <div class="placehoolder" />
  {:else}
    <div class="flex flex-col gap-4">
      {#if hasCredential}
        <p>
          You've presented the credential <em>{issuerName}</em> so you can now view the exclusive content
        </p>
        <div class="sm:px-36">
          <img class="h-auto max-w-full rounded-container-token" src={imageUrl} alt="Visible" />
        </div>
      {:else}
        <p>You did not prove you hold the <em>{issuerName}</em> credential.</p>
        <p>
          If you want to access the image, request the <em>{issuerName}</em> credential from the issuer
          and share your credential with the demo relying party.
        </p>
      {/if}
    </div>
  {/if}
  <Button slot="footer" on:click={close} variant="ghost-primary">Close</Button>
</Modal>

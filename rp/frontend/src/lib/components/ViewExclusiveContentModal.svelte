<script lang="ts">
  /* eslint-disable svelte/no-at-html-tags */
  import { getModalStore } from '@skeletonlabs/skeleton';
  import Modal from './Modal.svelte';
  import Button from './Button.svelte';
  import { loadCredential } from '$lib/services/load-credential.services';
  import { authStore } from '$lib/stores/auth.store';
  import { credentialsStore } from '$lib/stores/credentials.store';
  import { onMount } from 'svelte';
  import { ISSUER_ORIGIN } from '$lib/constants/env-vars';
  import type { Principal } from '@dfinity/principal';
  import type { VisibleContentData } from '$lib/stores/content-data-visible.store';

  /* eslint-disable-next-line */
  export let parent: any;

  const modalStore = getModalStore();

  onMount(() => {
    if ($modalStore[0]?.meta.startFlow) {
      startFlow();
    }
  });

  let contentData: VisibleContentData | undefined;
  $: contentData = $modalStore[0]?.meta.content;
  let credentialName: string | undefined;
  $: credentialName = $modalStore[0]?.meta.credentialGroupName;
  let owner: Principal | undefined;
  $: owner = contentData?.credential_issuer;
  let issuerName: string | undefined;
  $: issuerName = contentData?.issuer_nickname;
  let imageUrl: string | undefined;
  $: imageUrl = contentData?.url ?? '';

  // `undefined` means the flow has not started yet.
  let vcFlowLoading: undefined | boolean = undefined;
  const startFlow = async () => {
    if (owner && credentialName && contentData?.credential_spec) {
      vcFlowLoading = true;
      await loadCredential({
        groupName: credentialName,
        owner,
        credentialSpec: contentData?.credential_spec,
        identity: $authStore.identity,
      });
      vcFlowLoading = false;
    }
  };

  const close = () => {
    parent.onClose();
  };

  let hasCredential: boolean | undefined;
  // TODO: Compare the whole credential spec instead of just the group name and issuer.
  $: hasCredential = $credentialsStore.find(
    (credential) =>
      credential.groupName === credentialName && credential.owner.toText() === owner?.toText()
  )?.hasCredential;
</script>

<Modal>
  <svelte:fragment slot="header">
    {#if hasCredential}
      Access granted
    {:else if vcFlowLoading || vcFlowLoading === undefined}
      Present Your Credential
    {:else}
      Access Denied
    {/if}
  </svelte:fragment>
  {#if vcFlowLoading === undefined && hasCredential === undefined}
    <div class="flex-1 flex flex-col justify-center items-center gap-4">
      <p>
        Present the credential <em>{credentialName}</em> to view this image.
      </p>
      <Button on:click={startFlow} variant="primary">Get Credential</Button>
    </div>
  {:else if vcFlowLoading}
    <div class="flex flex-col justify-center items-center animate-pulse">
      <div class="placeholder min-w-56 min-h-56" />
      <p>Verify your credential in Internet Identity</p>
    </div>
  {:else}
    <div class="flex flex-col gap-4">
      {#if hasCredential}
        <div class="sm:px-36" data-tid="verify-credential-image-success">
          <img class="h-auto max-w-full rounded-container-token" src={imageUrl} alt="Visible" />
        </div>
      {:else}
        <p>
          You did not prove you hold the <em>{credentialName}</em> credential issued by
          <em>{issuerName}</em>.
        </p>
        <p>Visit the issuer to request the credential that grants you access to this image.</p>
      {/if}
    </div>
  {/if}
  <div class="flex gap-4" slot="footer">
    <Button on:click={close} variant="ghost-primary">Close</Button>
    {#if !hasCredential && !vcFlowLoading}
      <Button href={ISSUER_ORIGIN} variant="primary">Go to Issuer</Button>
    {/if}
  </div>
</Modal>

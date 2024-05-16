<script lang="ts">
  import { goto } from '$app/navigation';
  import Button from '$lib/components/Button.svelte';
  import { authStore } from '$lib/stores/auth.store';
  import { nonNullish } from '$lib/utils/non-nullish';
  import { getModalStore, getToastStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import { credentialsTypesStore, issuersStore, type Issuer } from '$lib/stores/issuers.store';
  import type { ImageData } from '../../../declarations/rp/rp.did';
  import { shareContent } from '$lib/services/shareContent.services';
  import TestIdWrapper from '$lib/components/TestIdWrapper.svelte';
  import {
    AGE_CREDENTIAL_GROUP,
    CREDENTIALS_WITH_INPUT,
    CREDENTIALS_WITH_INPUT_NUMBER,
    CREDENTIALS_WITH_INPUT_TEXT,
    EMPLOYMENT_CREDENTIAL_GROUP,
    RESIDENCE_CREDENTIAL_GROUP,
  } from '$lib/constants/credentials';

  const modalStore = getModalStore();
  const toastStore = getToastStore();

  let selectedCredential: string | undefined;
  let issuersToSelect: Issuer[] | undefined = [];
  $: issuersToSelect = $issuersStore[selectedCredential ?? ''];
  let selectedIssuer: Issuer | undefined;
  let predicateCredentialText: string | undefined;
  let predicateCredentialNumber: number | undefined;

  const predicateTextMapper: Record<string, string> = {
    [AGE_CREDENTIAL_GROUP]: 'Enter the minimum age required to view this image.',
    [RESIDENCE_CREDENTIAL_GROUP]: 'Enter the country of residence required to view this image.',
    [EMPLOYMENT_CREDENTIAL_GROUP]: 'Enter the employment status required to view this image.',
  };

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

  // Return `true` if:
  // * selectedCredential doesn't need an input.
  // * selectedCredential needs a text input and the predicateCredentialText is filled.
  // * selectedCredential needs a number input and the predicateCredentialNumber is filled.
  let filledRequiredInput = false;
  $: filledRequiredInput =
    !CREDENTIALS_WITH_INPUT.includes(selectedCredential ?? '') ||
    (CREDENTIALS_WITH_INPUT_NUMBER.includes(selectedCredential ?? '') &&
      predicateCredentialNumber !== undefined) ||
    (CREDENTIALS_WITH_INPUT_TEXT.includes(selectedCredential ?? '') &&
      predicateCredentialText !== undefined);
  let enableShareButton = false;
  $: enableShareButton =
    (selectedCredential ?? '').length > 0 && selectedImage !== undefined && filledRequiredInput;

  let showSuccessfulMessage = false;

  const shareAnotherImage = () => {
    showSuccessfulMessage = false;
    selectedCredential = undefined;
    selectedIssuer = undefined;
    selectedImage = undefined;
    predicateCredentialText = undefined;
  };

  let isLoading = false;
  const share = async () => {
    isLoading = true;
    // Edge case, should never happen because button is disabled.
    if (!selectedCredential || !selectedImage || !selectedIssuer) {
      return;
    }
    const isSuccessful = await shareContent({
      issuerName: selectedCredential,
      image: selectedImage,
      owner: selectedIssuer.owner,
      identity: $authStore.identity,
      toastStore,
      predicate: predicateCredentialText ?? predicateCredentialNumber,
    });
    if (isSuccessful) {
      toastStore.trigger({
        message: 'Content shared successfully!',
        background: 'variant-filled-success',
      });
      showSuccessfulMessage = true;
    }
    isLoading = false;
    predicateCredentialText = undefined;
  };
</script>

<TestIdWrapper testId="share-page">
  {#if showSuccessfulMessage && selectedImage}
    <h1 data-tid="success-message" class="h1">Published!</h1>
    <div class="flex justify-center">
      <img src={selectedImage.url} alt="Selected" class="max-w-72 h-auto rounded-container-token" />
    </div>
    <div class="flex justify-center gap-6">
      <Button on:click={() => goto('/feed')} variant="secondary">View Published Images</Button>
      <Button on:click={shareAnotherImage} variant="primary">Publish another image</Button>
    </div>
  {:else}
    <h1 class="h1">Choose a credential type, issuer, and image.</h1>
    <div class="flex flex-col gap-4">
      <label for="credentials">
        <h5 class="h5">Select the credential type that gets acces to this image.</h5>
      </label>
      <select bind:value={selectedCredential} id="credentials" class="select px-4">
        <option value="" disabled selected>Credential type</option>
        {#each $credentialsTypesStore as credential}
          <option value={credential} id={credential}>
            {credential}
          </option>
        {/each}
      </select>
    </div>

    <div class="flex flex-col gap-4">
      <label for="credentials">
        <h5 class="h5">Choose the issuer that you trust.</h5>
      </label>
      <select bind:value={selectedIssuer} id="issuers" class="select px-4">
        <option value="" disabled selected>Issuer</option>
        {#each issuersToSelect ?? [] as issuer}
          <option value={issuer} id={issuer.nickname}>
            {issuer.nickname}
          </option>
        {/each}
      </select>
    </div>

    {#if nonNullish(selectedCredential) && CREDENTIALS_WITH_INPUT_TEXT.includes(selectedCredential)}
      <div class="flex flex-col gap-4">
        <label for="credentials">
          <h5 class="h5">{predicateTextMapper[selectedCredential]}</h5>
        </label>
        <input type="text" class="input" bind:value={predicateCredentialText} />
      </div>
    {/if}

    {#if nonNullish(selectedCredential) && CREDENTIALS_WITH_INPUT_NUMBER.includes(selectedCredential)}
      <div class="flex flex-col gap-4">
        <label for="credentials">
          <h5 class="h5">{predicateTextMapper[selectedCredential]}</h5>
        </label>
        <input type="number" class="input" bind:value={predicateCredentialNumber} />
      </div>
    {/if}

    <div class="flex flex-col gap-4">
      <h5 class="h5">Choose an image to share</h5>
      {#if selectedImage}
        <div class="flex justify-center">
          <img
            src={selectedImage.url}
            alt="Selected"
            class="max-w-72 h-auto rounded-container-token"
          />
        </div>
      {/if}
      <div class="flex justify-center">
        <Button testId="choose-image" on:click={openChooseImageModal} variant="secondary"
          >Choose Image</Button
        >
      </div>
    </div>

    <div class="flex justify-end">
      <Button
        testId="publish-button"
        on:click={share}
        variant="primary"
        disabled={!enableShareButton}
        loading={isLoading}>Publish</Button
      >
    </div>
  {/if}
</TestIdWrapper>

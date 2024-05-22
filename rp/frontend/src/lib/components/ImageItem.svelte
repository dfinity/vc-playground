<script lang="ts">
  import type { VisibleContentData } from '$lib/stores/content-data-visible.store';
  import { nanoSecondsToDateTime } from '$lib/utils/date.utils';
  import { nonNullish } from '$lib/utils/non-nullish';
  import { getModalStore, getToastStore, type ModalSettings } from '@skeletonlabs/skeleton';
  import Button from './Button.svelte';
  import { authStore } from '$lib/stores/auth.store';
  import { login } from '$lib/services/auth.services';
  import { credentialSpecPredicate } from '$lib/utils/credential-spec-predicate.utils';

  export let image: VisibleContentData;

  const modalStore = getModalStore();
  const toastStore = getToastStore();

  let credentialPredicate: string | number | undefined;
  $: credentialPredicate = credentialSpecPredicate(image.credential_spec);

  let title: string;
  $: title = `${image.credential_group_name}${credentialPredicate ? ` - ${credentialPredicate}` : ''}`;

  const openModal = ({
    content,
    startFlow,
  }: {
    content: VisibleContentData;
    startFlow: boolean;
  }) => {
    const modal: ModalSettings = {
      type: 'component',
      component: 'viewExclusiveContentModal',
      meta: {
        content,
        startFlow,
      },
    };
    modalStore.trigger(modal);
  };

  const openImageFactory = (content: VisibleContentData) => () => {
    if (nonNullish($authStore.identity)) {
      openModal({ content, startFlow: true });
    } else {
      login({ toastStore, cb: () => openModal({ content, startFlow: false }) });
    }
  };

  const visibleImageGradient = `
    background-image: linear-gradient(
      to bottom, 
      rgba(0,0,0,0) calc(100% - 3rem), /* Transparent middle */
      rgba(0,0,0,0.6) 100%);
  `;
</script>

<article class="card" data-tid="image-item" data-credential-name={image.credential_group_name}>
  <header class="p-2">
    <!-- TODO: Fix UI misaligment for titles with multiple lines -->
    <h5 class="h5 w-full">
      {title}
    </h5>
    <p class="text-sm text-surface-600-300-token truncate">
      {`Trusted Issuer: ${image.issuer_nickname}`}
    </p>
  </header>
  <div class="relative text-surface-50">
    {#if !image.visible}
      <div
        class="absolute rounded-bl-container-token rounded-br-container-token -top-0 -left-0 w-full aspect-square backdrop-blur-sm"
      ></div>
    {/if}
    <div
      class={`absolute -top-0 -left-0 rounded-bl-container-token rounded-br-container-token w-full flex flex-col justify-end items-center py-2 px-2 h-full`}
      style={visibleImageGradient}
    >
      {#if !image.visible}
        <div class="flex-1 flex justify-center items-center">
          <Button variant="secondary" on:click={openImageFactory(image)}>View</Button>
        </div>
      {/if}
      <p class="text-sm self-start">{nanoSecondsToDateTime(image.created_timestamp_ns)}</p>
    </div>
    <div
      class="h-auto max-w-full aspect-square rounded-bl-container-token rounded-br-container-token"
      style="background-image: url({image.url}); background-size: cover; background-position: center;"
    />
  </div>
</article>

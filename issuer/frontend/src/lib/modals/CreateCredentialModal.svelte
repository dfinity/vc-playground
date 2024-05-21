<script lang="ts">
  import Button from '$lib/ui-components/elements/Button.svelte';
  import { isNullish } from '$lib/utils/is-nullish.utils';
  import { getModalStore } from '@skeletonlabs/skeleton';
  import type { GroupType } from '../../declarations/meta_issuer.did';
  import { getAllIssuerTypesStore } from '$lib/stores/issyer-types.store';
  import { authStore } from '$lib/stores/auth.store';
  import type { Readable } from 'svelte/store';

  /* eslint-disable-next-line */
  export let parent: any;

  const modalStore = getModalStore();

  let issuerTypes: Readable<GroupType[] | undefined>;
  $: issuerTypes = getAllIssuerTypesStore($authStore.identity);

  const close = () => {
    parent.onClose();
  };

  let issuerNickname = '';
  $: issuerNickname = $modalStore[0]?.meta.issuerNickname;

  let selectedCredential: undefined | string;

  const sendCredential = () => {
    $modalStore[0].response?.(selectedCredential);
    close();
  };
</script>

<div
  class="modal block overflow-y-auto bg-surface-100-800-token w-modal h-auto min-h-80 p-4 rounded-container-token shadow-xl flex flex-col gap-4"
  role="dialog"
  aria-modal="true"
>
  <header class="modal-header text-2xl font-bold">Create Credential</header>
  <div class="flex-1 flex flex-col gap-6">
    <div>
      <p>Using this playground, you can gate access to images on the image sharing platform.</p>
    </div>
    <div class="flex flex-col gap-4">
      <label for="choose-credential">
        <p>Credential type</p>
        <p class="text-sm text-surface-500">
          Select a type of credential from the list below. These serve as examples of the types of
          credentials issuers might create for users.
        </p>
      </label>
      <select bind:value={selectedCredential} id="choose-credential" class="select px-4">
        <option value="" disabled selected>Select a credential type</option>
        {#each $issuerTypes ?? [] as issuerType}
          <option value={issuerType.group_name} id={issuerType.group_name}>
            {issuerType.group_name}
          </option>
        {/each}
      </select>
    </div>
    <div>
      <p class="text-surface-500">
        {`This credential will be issued by "${issuerNickname}".`}
      </p>
    </div>
  </div>
  <div class="modal-footer flex justify-end space-x-2">
    <Button on:click={close} variant="ghost">Cancel</Button>
    <Button
      testId="create-credential"
      on:click={sendCredential}
      disabled={isNullish(selectedCredential) || selectedCredential === ''}
      variant="primary">Create Credential</Button
    >
  </div>
</div>

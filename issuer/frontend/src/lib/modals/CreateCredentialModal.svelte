<script lang="ts">
  import Button from '$lib/ui-components/elements/Button.svelte';
  import { isNullish } from '$lib/utils/is-nullish.utils';
  import { getModalStore } from '@skeletonlabs/skeleton';

  /* eslint-disable-next-line */
  export let parent: any;

  const modalStore = getModalStore();

  const close = () => {
    parent.onClose();
  };

  let issuerNickname = '';
  $: issuerNickname = $modalStore[0]?.meta.issuerNickname;

  const credentials = [
    'Proof of humanity',
    '18 or older',
    'Lives in Switzerland',
    'Worsk at DFINITY',
  ];

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
  <header class="modal-header text-2xl font-bold">Create credential</header>
  <div class="flex-1 flex flex-col gap-6">
    <div>
      <p>Select a credential type so that you can issue credentials to users.</p>
      <p>VC Playground credentials gate access to images on the relying party dapp.</p>
    </div>
    <div class="flex flex-col gap-4">
      <label for="choose-credential">
        <p>Credential type</p>
        <p class="text-sm text-surface-500">
          Choose a popular example of type of credentials from the list below. It will be the name
          of a credential.
        </p>
      </label>
      <select bind:value={selectedCredential} id="choose-credential" class="select px-4">
        <option value="" disabled selected>Select a credential</option>
        {#each credentials as credential}
          <option value={credential} id={credential}>
            {credential}
          </option>
        {/each}
      </select>
    </div>
    <div>
      <p class="text-surface-500">
        {`Credential will be issued by "${issuerNickname}'s Organization".`}
      </p>
    </div>
  </div>
  <div class="modal-footer flex justify-end space-x-2">
    <Button on:click={close} variant="ghost">Cancel</Button>
    <Button on:click={sendCredential} disabled={isNullish(selectedCredential)} variant="primary"
      >Create Credential</Button
    >
  </div>
</div>
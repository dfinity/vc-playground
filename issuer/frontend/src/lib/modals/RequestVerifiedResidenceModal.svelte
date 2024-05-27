<script lang="ts">
  import Button from '$lib/ui-components/elements/Button.svelte';
  import { isNullish } from '$lib/utils/is-nullish.utils';
  import { getModalStore } from '@skeletonlabs/skeleton';
  import CountriesSelect from '$lib/components/CountriesSelect.svelte';

  /* eslint-disable-next-line */
  export let parent: any;

  const modalStore = getModalStore();

  const close = () => {
    parent.onClose();
  };

  let selectedCountry: undefined | string;

  const requestCredential = () => {
    $modalStore[0].response?.(selectedCountry);
    close();
  };
</script>

<div
  class="modal block overflow-y-auto bg-surface-100-800-token w-modal h-auto p-4 rounded-container-token shadow-xl flex flex-col gap-6"
  role="dialog"
  aria-modal="true"
>
  <header class="modal-header text-2xl font-bold">Request Credential: Verified Residence</header>
  <div class="flex-1 flex flex-col gap-4">
    <div>
      <p>Select your country of residence.</p>
    </div>
    <div class="flex flex-col gap-4">
      <CountriesSelect bind:selectedCountry />
    </div>
  </div>
  <div class="modal-footer flex justify-end space-x-2">
    <Button on:click={close} variant="ghost">Cancel</Button>
    <Button
      testId="request-credential"
      on:click={requestCredential}
      disabled={isNullish(selectedCountry)}
      variant="primary">Submit</Button
    >
  </div>
</div>

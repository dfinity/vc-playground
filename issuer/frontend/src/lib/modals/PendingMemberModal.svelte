<script lang="ts">
  import Modal from '$lib/ui-components/elements/Modal.svelte';
  import { getModalStore } from '@skeletonlabs/skeleton';

  const modalStore = getModalStore();

  let credentialName: string;
  $: credentialName = $modalStore[0] ? $modalStore[0].meta.name : '';
</script>

{#if $modalStore[0]}
  <Modal>
    <svelte:fragment slot="title">Pending Credential Request</svelte:fragment>
    <p>
      {`The ${credentialName} credential was not yet issued.`}
    </p>
    <p>
      {`Wait for the issuer to issue your credential.`}
    </p>
  </Modal>
{/if}

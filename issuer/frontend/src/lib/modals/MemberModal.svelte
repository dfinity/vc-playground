<script lang="ts">
  import Button from '$lib/ui-components/elements/Button.svelte';
  import Modal from '$lib/ui-components/elements/Modal.svelte';
  import { getModalStore } from '@skeletonlabs/skeleton';

  const modalStore = getModalStore();

  let credentialName: string;
  $: credentialName = $modalStore[0] ? $modalStore[0].meta.name : '';
</script>

{#if $modalStore[0]}
  <Modal>
    <svelte:fragment slot="title">Test Your Credential On the Relying Party</svelte:fragment>
    <p>
      {`You have a credential for ${credentialName}. Visit Xxxxx to view content that's only accessible to users with the credential of ${credentialName}.`}
    </p>
    <Button slot="actions" variant="primary" href="https://www.skeleton.dev/"
      >Test on Relying Party</Button
    >
  </Modal>
{/if}

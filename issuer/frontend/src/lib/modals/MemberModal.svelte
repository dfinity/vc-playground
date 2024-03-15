<script lang="ts">
  import Button from '$lib/ui-components/elements/Button.svelte';
  import Modal from '$lib/ui-components/elements/Modal.svelte';
  import { getModalStore } from '@skeletonlabs/skeleton';

  const modalStore = getModalStore();

  let groupName: string;
  $: groupName = $modalStore[0] ? $modalStore[0].meta.name : '';
</script>

{#if $modalStore[0]}
  <Modal>
    <svelte:fragment slot="title">Test Your Credential On the Relying Party</svelte:fragment>
    <p>
      {`You are a member ${groupName}. Visit Xxxxx to view content that's only accessible to members of ${groupName}.`}
    </p>
    <Button slot="actions" variant="primary" href="https://www.skeleton.dev/"
      >Test on Relying Party</Button
    >
  </Modal>
{/if}

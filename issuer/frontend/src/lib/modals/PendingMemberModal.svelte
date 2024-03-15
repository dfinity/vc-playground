<script lang="ts">
  import Modal from '$lib/ui-components/elements/Modal.svelte';
  import { getModalStore } from '@skeletonlabs/skeleton';

  const modalStore = getModalStore();

  let groupName: string;
  $: groupName = $modalStore[0] ? $modalStore[0].meta.name : '';
</script>

{#if $modalStore[0]}
  <Modal>
    <svelte:fragment slot="title">Pending Join Request</svelte:fragment>
    <p>
      {`You are not approved for ${groupName} credential yet.`}
    </p>
    <p>
      {`Wait for the owner of the group to approve you.`}
    </p>
  </Modal>
{/if}

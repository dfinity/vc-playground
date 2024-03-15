<script lang="ts">
  import Heading from './Heading.svelte';
</script>

<div class="flex flex-col gap-6 card p-8 w-modal shadow-xl min-h-96">
  <Heading level="2"><slot name="title" /></Heading>
  <div class="flex flex-col gap-2 flex-1">
    <slot />
  </div>
  {#if $$slots.actions}
    <div class="flex justify-end">
      <slot name="actions" />
    </div>
  {/if}
</div>

<script lang="ts">
  import IconRight from '../icons/IconRight.svelte';
  import Heading from './Heading.svelte';
  import Stack from './Stack.svelte';

  export let onClick: (() => void) | undefined = undefined;
  export let testId: string | undefined = undefined;

  function handleClick() {
    if (onClick) {
      onClick();
    }
  }
  let hoverClass = '';
  $: hoverClass = onClick ? 'cursor-pointer' : '';
</script>

<li
  on:click={handleClick}
  on:keypress={handleClick}
  role="row"
  data-tid={testId}
  class={`flex justify-between items-center gap-4 p-2 hover:bg-tertiary-hover-token rounded-container-token ${hoverClass}`}
>
  <slot name="start" />
  <div class="flex-1">
    <Stack>
      <Heading level="5"><slot name="main" /></Heading>
      {#if $$slots.sub}
        <span class="text-sm text-surface-500"><slot name="sub" /></span>
      {/if}
    </Stack>
  </div>
  <span class="flex gap-2 items-center">
    <slot name="end" />
    {#if onClick}
      <IconRight />
    {/if}
  </span>
</li>

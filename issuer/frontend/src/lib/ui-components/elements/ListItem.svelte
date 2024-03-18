<script lang="ts">
  import IconRight from '../icons/IconRight.svelte';
  import Heading from './Heading.svelte';

  export let onClick: (() => void) | undefined = undefined;

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
  class={`flex justify-between items-center gap-4 p-2 hover:bg-tertiary-hover-token rounded-container-token ${hoverClass}`}
>
  <slot name="start" />
  <div class="flex-1">
    <Heading level="5"><slot name="main" /></Heading>
  </div>
  <span class="flex gap-2 items-center">
    <slot name="end" />
    {#if onClick}
      <IconRight />
    {/if}
  </span>
</li>

<script lang="ts">
  import Heading from './Heading.svelte';

  export let testId: string | undefined = undefined;
</script>

<button
  data-tid={testId}
  class="card card-hover p-4 flex flex-col justify-start items-center"
  on:click
>
  <div class="p-8 flex justify-center items-center">
    <slot name="icon" />
  </div>
  <Heading level="3" align="center"><slot name="title" /></Heading>
  <p class="text-center"><slot /></p>
</button>

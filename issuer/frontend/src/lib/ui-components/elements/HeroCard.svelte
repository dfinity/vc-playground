<script lang="ts">
  import Heading from './Heading.svelte';

  export let testId: string | undefined = undefined;
  export let href: string;
</script>

<a {href} data-tid={testId} class="card card-hover p-4 flex flex-col justify-start items-center">
  <div class="sm:p-8 p-4 flex justify-center items-center">
    <slot name="icon" />
  </div>
  <Heading level="3" align="center"><slot name="title" /></Heading>
  <p class="text-center"><slot /></p>
</a>

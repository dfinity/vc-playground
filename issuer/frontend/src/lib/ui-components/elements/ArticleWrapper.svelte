<script lang="ts">
  import Heading from './Heading.svelte';

  export let testId: string | undefined = undefined;
</script>

<article class="flex flex-col gap-4" data-tid={testId}>
  <Heading level="3">
    <slot name="title" />
  </Heading>
  <div>
    <slot />
  </div>
</article>
